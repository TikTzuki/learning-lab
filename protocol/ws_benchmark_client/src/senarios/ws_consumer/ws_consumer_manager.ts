import WebSocketConsumer from "./ws_consumer";
import {BenchmarkManager, BenchmarkObj, ResponseMessage} from "../../base/dto";
import ConsumerResultsWriter from "../../base/consumer_results_writer";
import FileManager from "../../base/file_manager";
import SocketIOConsumer from "./ws_consumer_socketio";

// ConsumeResult: groups messages by client_id
export interface ConsumeResult {
    [client_id: string]: ConsumeMetrics[];
}

export interface ConsumeMetrics {
    client_id: string;
    receiver: string;
    round: number
    c: number;
    ts: number;
    client_receive_ts: number;
}

export class WebSocketConsumerManager implements BenchmarkManager {
    private consumers: any[] = [];
    benchmark_object: BenchmarkObj;
    connectionCount: number;
    consumeResult: ConsumeResult = {};
    fm: FileManager;
    writer: ConsumerResultsWriter;
    timeoutMs: number;
    scenarios: string;
    msg_count: number = 0;
    consume_msg_limit: number;
    isStopped: boolean = false;

    constructor(
        scenarios: string,
        benchmark_object: BenchmarkObj,
        connectionCount: number
    ) {
        console.log("Creating WebSocketConsumerManager with scenarios:", scenarios);
        this.benchmark_object = benchmark_object;
        this.connectionCount = connectionCount;
        this.scenarios = scenarios;
        this.fm = new FileManager(scenarios);
        this.writer = new ConsumerResultsWriter(this.fm);
        this.timeoutMs = benchmark_object.extra.consumerDuration || Number(process.env.CONSUMER_DURATION) || 1_000;
        this.consume_msg_limit = benchmark_object.extra.consumeMsgLimit || Number(process.env.CONSUME_MSG_LIMIT);
    }

    async runBenchmark(ROUNDS: number): Promise<void> {
        await this.fm.createFile();

        // Create connections (consumers)
        await this.createConnections(0);

        // Wait for messages to be consumed for the given timeout OR until message limit is reached
        console.log(`Waiting for messages for ${this.timeoutMs / 1000} seconds or until ${this.consume_msg_limit} messages are received...`);

        return new Promise<void>((resolve) => {

            const timeout = setTimeout(() => {
                console.log(`Timeout reached after ${this.timeoutMs / 1000} seconds`);
                this.resolveAndCleanup(resolve);
            }, this.timeoutMs);

            // Check message count periodically
            const checkInterval = setInterval(() => {
                if (this.msg_count >= this.consume_msg_limit || this.isStopped) {
                    console.log(`Message limit reached: ${this.msg_count}/${this.consume_msg_limit}`);
                    clearTimeout(timeout);
                    clearInterval(checkInterval);
                    this.resolveAndCleanup(resolve);
                }
            }, 1_000); // Check every 500ms
        });
    }

    private async resolveAndCleanup(resolve: () => void) {
        try {
            // Write results using ConsumerResultsWriter
            await this.writer.writeReport(this.consumeResult);
            console.log("Consumer report written.");
            // Close consumers
            await this.close();
            this.consumers = [];
            this.msg_count = 0;
            this.consumeResult = {};
        } catch (error) {
            console.error("Error during cleanup:", error);
        } finally {
            resolve();
        }
    }

    async createConnections(_round: number): Promise<void> {
        for (let i = 0; i < this.connectionCount; i++) {
            let consumer: SocketIOConsumer | WebSocketConsumer;
            let clientId = i.toString();
            switch (this.scenarios) {
                case 'socketio-consumer':
                    consumer = new SocketIOConsumer(clientId, this.benchmark_object);
                    consumer.setMessageCallback((msg: ResponseMessage) => {
                        this.msg_count++;
                        this.addToResult(clientId, msg);
                    });
                    break;
                default:
                    consumer = new WebSocketConsumer(i.toString(), this.benchmark_object);
                    await consumer.connect();
                    consumer.setMessageCallback((msg: ResponseMessage) => {
                        this.msg_count++;
                        this.addToResult(clientId, msg);
                    });

            }
            // await consumer.connect();
            this.consumers.push(consumer);
        }
    }

    addToResult(clientId: string, msg: ResponseMessage) {
        if (this.msg_count % 500_000 == 0) {
            console.log(`Received message count: ${this.msg_count}`);
        }
        const metrics: ConsumeMetrics = {
            client_id: msg.client_id,
            receiver: clientId,
            round: msg.round,
            c: msg.c,
            ts: msg.ts,
            client_receive_ts: Date.now(),
        };
        if (!this.consumeResult[metrics.receiver]) {
            this.consumeResult[metrics.receiver] = [];
        }
        this.consumeResult[metrics.receiver].push(metrics);
    }

    async sendRequests(_: number): Promise<void> {
        throw new Error("sendRequests method is not implemented for WebSocketConsumerManager");
    }

    async close(): Promise<void> {
        console.log(`Closing WebSocketConsumerManager with ${this.consumers.length} consumers`);
        this.isStopped = true;
        try {
            // Disconnect all consumers
            for (const consumer of this.consumers) {
                try {
                    await consumer.disconnect();
                    console.log(`Consumer ${consumer.id || 'unknown'} disconnected`);
                } catch (error) {
                    console.error(`Error disconnecting consumer:`, error);
                }
            }


            console.log("All WebSocket consumers closed successfully");
        } catch (error) {
            console.error("Error during WebSocketConsumerManager close:", error);
            throw error;
        }
    }

    // Add a method to force stop/close the benchmark
    async forceStop(): Promise<void> {
        console.log("Force stopping WebSocket consumer benchmark");
        await this.close();
    }

    // Add a method to check if the manager is active
    isActive(): boolean {
        return this.consumers.length > 0;
    }

    // Add a method to get current status
    getStatus() {
        return {
            consumerCount: this.consumers.length,
            messageCount: this.msg_count,
            messageLimit: this.consume_msg_limit,
            scenario: this.scenarios,
            isActive: this.isActive()
        };
    }
}

export default WebSocketConsumerManager;
