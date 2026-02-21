import express from 'express';
import cors from 'cors';
import path from 'path';
import dotenv from 'dotenv';
import {BenchmarkManager, BenchmarkObj, ConnectionObj, ProgressObj} from './base/dto';
import ConnectionManager from './senarios/request_response/connection_manager';
import KafkaProducerManager from './senarios/kafka/kafka_connection_manager';
import {WebSocketConsumerManager} from './senarios/ws_consumer/ws_consumer_manager';

dotenv.config();

interface LogEntry {
    id: number;
    timestamp: number;
    level: 'info' | 'warn' | 'error' | 'debug';
    message: string;
    source?: string;
}

interface BenchmarkStatus {
    id: string;
    scenario: string;
    status: 'running' | 'completed' | 'error';
    progress: {
        connection: number;
        benchmark: number;
    };
    metrics: {
        messagesSent: number;
        responsesReceived: number;
        throughput: number;
        latency: number;
        successRate: number;
    };
    startTime: number;
    endTime?: number;
}

class Logger {
    private logs: LogEntry[] = [];
    private logId = 0;
    private maxLogs = 1000;
    private waitingClients: Array<{
        res: express.Response;
        lastLogId: number;
        timeout: NodeJS.Timeout;
    }> = [];
    private isLoggingEnabled = false; // Add flag to control logging
    private originalConsoleLog: typeof console.log;
    private originalConsoleError: typeof console.error;
    private originalConsoleWarn: typeof console.warn;
    private isCurrentlyLogging = false; // Prevent circular logging

    constructor() {
        // Store original console methods to avoid circular calls
        this.originalConsoleLog = console.log.bind(console);
        this.originalConsoleError = console.error.bind(console);
        this.originalConsoleWarn = console.warn.bind(console);
    }

    log(level: LogEntry['level'], message: string, source?: string) {
        // Prevent circular calls by checking if we're already logging
        if (!this.isLoggingEnabled || this.isCurrentlyLogging) {
            return;
        }

        // Set flag to prevent recursive calls
        this.isCurrentlyLogging = true;

        try {
            const logEntry: LogEntry = {
                id: ++this.logId,
                timestamp: Date.now(),
                level,
                message,
                source
            };

            this.logs.push(logEntry);

            // Keep only the last maxLogs entries
            if (this.logs.length > this.maxLogs) {
                this.logs = this.logs.slice(-this.maxLogs);
            }

            // Use original console methods to avoid circular calls
            const timestamp = new Date(logEntry.timestamp).toISOString();
            const prefix = source ? `[${source}]` : '';
            this.originalConsoleLog(`[${timestamp}] ${level.toUpperCase()} ${prefix} ${message}`);

            // Notify waiting clients
            this.notifyWaitingClients();
        } finally {
            // Always reset the flag
            this.isCurrentlyLogging = false;
        }
    }

    info(message: string, source?: string) {
        this.log('info', message, source);
    }

    warn(message: string, source?: string) {
        this.log('warn', message, source);
    }

    error(message: string, source?: string) {
        this.log('error', message, source);
    }

    debug(message: string, source?: string) {
        this.log('debug', message, source);
    }

    getLogsAfter(lastLogId: number): LogEntry[] {
        return this.logs.filter(log => log.id > lastLogId);
    }

    getAllLogs(): LogEntry[] {
        return [...this.logs];
    }

    waitForNewLogs(res: express.Response, lastLogId: number, timeoutMs = 30000) {
        // Check if there are already new logs
        const newLogs = this.getLogsAfter(lastLogId);
        if (newLogs.length > 0) {
            res.json({logs: newLogs});
            return;
        }

        // Set up long polling
        const timeout = setTimeout(() => {
            this.removeWaitingClient(res);
            res.json({logs: []}); // Return empty array on timeout
        }, timeoutMs);

        this.waitingClients.push({
            res,
            lastLogId,
            timeout
        });
    }

    private notifyWaitingClients() {
        const clientsToNotify = [...this.waitingClients];
        this.waitingClients = [];

        clientsToNotify.forEach(client => {
            clearTimeout(client.timeout);
            const newLogs = this.getLogsAfter(client.lastLogId);
            if (newLogs.length > 0) {
                client.res.json({logs: newLogs});
            } else {
                client.res.json({logs: []});
            }
        });
    }

    removeWaitingClient(res: express.Response) {
        const index = this.waitingClients.findIndex(client => client.res === res);
        if (index !== -1) {
            clearTimeout(this.waitingClients[index].timeout);
            this.waitingClients.splice(index, 1);
        }
    }

    enableLogging() {
        this.isLoggingEnabled = true;
        this.log('info', 'Logging enabled', 'LOGGER');
    }

    disableLogging() {
        this.isLoggingEnabled = false;
        // Clear all waiting clients
        this.clearWaitingClients();
    }

    isEnabled(): boolean {
        return this.isLoggingEnabled;
    }

    clearWaitingClients() {
        this.waitingClients.forEach(client => {
            clearTimeout(client.timeout);
            client.res.json({logs: [], stopped: true});
        });
        this.waitingClients = [];
    }

    clearLogs() {
        this.logs = [];
        this.logId = 0;
    }
}

class WebServer {
    private app: express.Application;
    private port: number;
    private runningBenchmarks: Map<string, BenchmarkStatus> = new Map();
    private benchmarkManagers: Map<string, BenchmarkManager> = new Map();
    private logger: Logger = new Logger();

    constructor() {
        this.app = express();
        this.port = Number(process.env.WEB_PORT) || 3000;
        this.setupMiddleware();
        this.setupRoutes();
        this.initializeLogger();
    }

    private initializeLogger() {
        this.logger.info('WebSocket Benchmark Client server starting...', 'SERVER');

        // Override console methods to capture all logs
        const originalConsoleLog = console.log;
        const originalConsoleError = console.error;
        const originalConsoleWarn = console.warn;

        console.log = (...args) => {
            const message = args.map(arg => typeof arg === 'string' ? arg : JSON.stringify(arg)).join(' ');
            this.logger.info(message, 'CONSOLE');
            originalConsoleLog.apply(console, args);
        };

        console.error = (...args) => {
            const message = args.map(arg => typeof arg === 'string' ? arg : JSON.stringify(arg)).join(' ');
            this.logger.error(message, 'CONSOLE');
            originalConsoleError.apply(console, args);
        };

        console.warn = (...args) => {
            const message = args.map(arg => typeof arg === 'string' ? arg : JSON.stringify(arg)).join(' ');
            this.logger.warn(message, 'CONSOLE');
            originalConsoleWarn.apply(console, args);
        };
    }

    private setupMiddleware() {
        this.app.use(cors());
        this.app.use(express.json());
        this.app.use(express.static(path.join(__dirname, '../public')));
    }

    private setupRoutes() {
        // Serve HTML pages
        this.app.get('/', (req, res) => {
            res.sendFile(path.join(__dirname, '../public/index.html'));
        });

        this.app.get('/kafka-producer', (req, res) => {
            res.sendFile(path.join(__dirname, '../public/kafka-producer.html'));
        });

        this.app.get('/request-response', (req, res) => {
            res.sendFile(path.join(__dirname, '../public/request-response.html'));
        });

        this.app.get('/ws-consumer', (req, res) => {
            res.sendFile(path.join(__dirname, '../public/ws-consumer.html'));
        });

        this.app.get('/results', (req, res) => {
            res.sendFile(path.join(__dirname, '../public/results.html'));
        });

        this.app.get('/monitor', (req, res) => {
            res.sendFile(path.join(__dirname, '../public/monitor.html'));
        });

        // API endpoints
        this.app.post('/api/benchmark/start', this.startBenchmark.bind(this));
        this.app.post('/api/benchmark/stop/:id', this.stopBenchmark.bind(this));
        this.app.get('/api/benchmark/status/:id', this.getBenchmarkStatus.bind(this));
        this.app.get('/api/benchmark/list', this.listBenchmarks.bind(this));
        this.app.get('/api/system/status', this.getSystemStatus.bind(this));
        this.app.get('/api/results', this.getResults.bind(this));

        // Long polling endpoint for logs
        this.app.get('/api/logs/poll', this.pollLogs.bind(this));
        this.app.get('/api/logs', this.getLogs.bind(this));
        this.app.delete('/api/logs', this.clearLogs.bind(this));

        // New endpoints for log polling control
        this.app.post('/api/logs/start', this.startLogPolling.bind(this));
        this.app.post('/api/logs/stop', this.stopLogPolling.bind(this));
        this.app.get('/api/logs/status', this.getLogStatus.bind(this));
    }

    private pollLogs(req: express.Request, res: express.Response) {
        const lastLogId = parseInt(req.query.lastLogId as string) || 0;
        const timeout = parseInt(req.query.timeout as string) || 30000;

        // Set up connection close handler
        req.on('close', () => {
            this.logger.removeWaitingClient(res);
        });

        this.logger.waitForNewLogs(res, lastLogId, timeout);
    }

    private getLogs(req: express.Request, res: express.Response) {
        const limit = parseInt(req.query.limit as string) || 100;
        const level = req.query.level as string;

        let logs = this.logger.getAllLogs();

        if (level) {
            logs = logs.filter(log => log.level === level);
        }

        // Return the last 'limit' logs
        logs = logs.slice(-limit);

        res.json({logs});
    }

    private clearLogs(req: express.Request, res: express.Response) {
        this.logger.clearLogs();
        this.logger.enableLogging(); // Make sure logging is enabled
        this.logger.info('Logs cleared via API', 'SERVER');
        res.json({success: true, message: 'Logs cleared'})
    }

    private async startBenchmark(req: express.Request, res: express.Response) {
        try {
            const {scenario, config} = req.body;
            const benchmarkId = `${scenario}-${Date.now()}`;

            // Enable logging when starting a benchmark
            this.logger.enableLogging();

            this.logger.info(`Starting ${scenario} benchmark with ID: ${benchmarkId}`, 'BENCHMARK');
            this.logger.debug(`Configuration: ${JSON.stringify(config)}`, 'BENCHMARK');

            // Create benchmark configuration
            const benchmarkObj: BenchmarkObj = {
                websocket_address: config.wsAddress || process.env.WEBSOCKET_ADDRESS || "127.0.0.1",
                websocket_port: config.wsPort || Number(process.env.WEBSOCKET_PORT) || 8080,
                connection_interval: config.connectionInterval || 10,
                request_interval: config.requestInterval || 100,
                extra: config
            };

            const connectionObj: ConnectionObj = {
                connection_time: 0,
                times: [],
                clients: []
            };

            const connectionProgressObj: ProgressObj = {
                counter: 0,
                total: benchmarkObj.connection_interval,
                message: "Connecting..."
            };

            const benchmarkProgressObj: ProgressObj = {
                counter: 0,
                total: benchmarkObj.connection_interval * benchmarkObj.request_interval,
                message: "Benchmarking..."
            };

            let benchmarkManager: BenchmarkManager;

            // Create appropriate benchmark manager
            switch (scenario) {
                case 'kafka-producer':
                    benchmarkManager = new KafkaProducerManager(
                        benchmarkObj,
                        connectionObj,
                        connectionProgressObj,
                        benchmarkProgressObj
                    );
                    break;
                case 'request-response':
                    benchmarkManager = new ConnectionManager(
                        benchmarkObj,
                        connectionObj,
                        connectionProgressObj,
                        benchmarkProgressObj
                    );
                    break;
                case 'ws-consumer':
                    benchmarkManager = new WebSocketConsumerManager(
                        'ws-consumer',
                        benchmarkObj,
                        config.connectionCount || 1
                    );
                    break;
                case 'socketio-consumer':
                    benchmarkManager = new WebSocketConsumerManager(
                        'socketio-consumer',
                        benchmarkObj,
                        config.connectionCount || 1
                    );
                    break;
                default:
                    return res.status(400).json({error: 'Invalid scenario'});
            }

            // Initialize benchmark status
            const benchmarkStatus: BenchmarkStatus = {
                id: benchmarkId,
                scenario,
                status: 'running',
                progress: {
                    connection: 0,
                    benchmark: 0
                },
                metrics: {
                    messagesSent: 0,
                    responsesReceived: 0,
                    throughput: 0,
                    latency: 0,
                    successRate: 0
                },
                startTime: Date.now()
            };

            this.runningBenchmarks.set(benchmarkId, benchmarkStatus);
            this.benchmarkManagers.set(benchmarkId, benchmarkManager);

            // Start benchmark asynchronously
            this.runBenchmarkAsync(benchmarkId, benchmarkManager, config.rounds || 1);

            // Update status
            this.logger.info(`Benchmark ${benchmarkId} initiated successfully`, 'BENCHMARK');

            res.json({
                success: true,
                benchmarkId,
                message: 'Benchmark started successfully',
                loggingEnabled: true
            });

        } catch (error) {
            this.logger.error(`Error starting benchmark: ${error}`, 'BENCHMARK');
            res.status(500).json({error: 'Failed to start benchmark'});
        }
    }

    private async runBenchmarkAsync(benchmarkId: string, benchmarkManager: BenchmarkManager, rounds: number) {
        try {
            const status = this.runningBenchmarks.get(benchmarkId);
            if (!status) return;

            // Run the benchmark
            await benchmarkManager.runBenchmark(rounds);

            // Update status to completed
            status.status = 'completed';
            status.endTime = Date.now();
            this.runningBenchmarks.set(benchmarkId, status);

        } catch (error) {
            console.error(`Error running benchmark ${benchmarkId}:`, error);
            const status = this.runningBenchmarks.get(benchmarkId);
            if (status) {
                status.status = 'error';
                status.endTime = Date.now();
                this.runningBenchmarks.set(benchmarkId, status);
            }
        } finally {
            // Clean up benchmark manager
            this.benchmarkManagers.delete(benchmarkId);
        }
    }

    private async stopBenchmark(req: express.Request, res: express.Response) {
        try {
            const {id} = req.params;
            const benchmarkManager = this.benchmarkManagers.get(id);
            const status = this.runningBenchmarks.get(id);

            if (!benchmarkManager || !status) {
                return res.status(404).json({error: 'Benchmark not found'});
            }

            this.logger.info(`Stopping benchmark ${id}`, 'BENCHMARK');

            // Handle different types of benchmark managers
            if (benchmarkManager instanceof WebSocketConsumerManager) {
                // Use the enhanced close method for WebSocket consumers
                await benchmarkManager.forceStop();
                this.logger.info(`WebSocket consumer benchmark ${id} force stopped`, 'BENCHMARK');
            } else {
                // Use the standard close method for other benchmarks
                await benchmarkManager.close();
                this.logger.info(`Benchmark ${id} stopped using standard close`, 'BENCHMARK');
            }

            // Update status
            status.status = 'completed';
            status.endTime = Date.now();
            this.runningBenchmarks.set(id, status);

            // Clean up
            this.benchmarkManagers.delete(id);

            res.json({
                success: true,
                message: 'Benchmark stopped successfully',
                benchmarkId: id,
                stoppedAt: status.endTime
            });

        } catch (error) {
            this.logger.error(`Error stopping benchmark: ${error}`, 'BENCHMARK');
            res.status(500).json({error: 'Failed to stop benchmark', details: `${error}`});
        }
    }

    private getBenchmarkStatus(req: express.Request, res: express.Response) {
        const {id} = req.params;
        const status = this.runningBenchmarks.get(id);

        if (!status) {
            return res.status(404).json({error: 'Benchmark not found'});
        }

        res.json(status);
    }

    private listBenchmarks(req: express.Request, res: express.Response) {
        const benchmarks = Array.from(this.runningBenchmarks.values());
        res.json(benchmarks);
    }

    private getSystemStatus(req: express.Request, res: express.Response) {
        const systemStatus = {
            status: 'healthy',
            uptime: process.uptime(),
            memory: process.memoryUsage(),
            activeConnections: this.runningBenchmarks.size,
            timestamp: Date.now()
        };

        res.json(systemStatus);
    }

    private getResults(req: express.Request, res: express.Response) {
        // This would typically fetch from a database or file system
        // For now, return mock data
        const results = [
            {
                id: 1,
                date: new Date().toISOString(),
                scenario: 'Kafka Producer',
                duration: '120s',
                connections: 50,
                throughput: '2,840 msg/s',
                latency: '35.2 ms',
                successRate: '99.8%'
            },
            {
                id: 2,
                date: new Date(Date.now() - 3600000).toISOString(),
                scenario: 'Request/Response',
                duration: '180s',
                connections: 100,
                throughput: '1,950 req/s',
                latency: '48.7 ms',
                successRate: '99.2%'
            }
        ];

        res.json(results);
    }

    private startLogPolling(req: express.Request, res: express.Response) {
        this.logger.enableLogging();
        res.json({success: true, message: 'Log polling started', loggingEnabled: true});
    }

    private stopLogPolling(req: express.Request, res: express.Response) {
        this.logger.disableLogging();
        res.json({success: true, message: 'Log polling stopped', loggingEnabled: false});
    }

    private getLogStatus(req: express.Request, res: express.Response) {
        const isEnabled = this.logger.isEnabled();
        res.json({success: true, loggingEnabled: isEnabled});
    }

    public start() {
        this.logger.info(`WebSocket Benchmark Client Web UI started on port ${this.port}`, 'SERVER');
        this.logger.info(`Dashboard: http://localhost:${this.port}`, 'SERVER');
        this.logger.info(`Monitor: http://localhost:${this.port}/monitor`, 'SERVER');
        this.logger.info(`Results: http://localhost:${this.port}/results`, 'SERVER');

        this.app.listen(this.port, () => {
            this.logger.info("WebSocket Benchmark Client server is running", 'SERVER');
        });
    }
}

// Start the web server
const webServer = new WebServer();
webServer.start();
