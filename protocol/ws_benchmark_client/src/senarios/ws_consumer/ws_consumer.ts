import {client as WebSocketClient, connection as WebSocketConnection} from 'websocket';
import {BenchmarkObj, ResponseMessage} from "../../base/dto";


export default class WebSocketConsumer {
    private client: WebSocketClient;
    private connection: WebSocketConnection | undefined;
    private isConnected: boolean = false;
    private benchmarkObject: BenchmarkObj;
    private messageCallback?: (message: ResponseMessage) => void;

    constructor(id: string, benchmarkObject: BenchmarkObj) {
        this.benchmarkObject = benchmarkObject;
        this.client = new WebSocketClient();
        this.setupEventHandlers();
    }

    setupEventHandlers(): void {
        this.client.on('connectFailed', (error) => {
            console.error('WebSocket connection failed:', error);
            this.isConnected = false;
        });

        this.client.on('connect', (connection) => {
            this.connection = connection;
            this.isConnected = true;

            connection.on('error', (error) => {
                console.error('WebSocket connection error:', error);
                this.isConnected = false;
            });

            connection.on('close', () => {
                console.log('WebSocket connection closed');
                this.isConnected = false;
            });

            connection.on('message', (message) => {
                if (message.type === 'utf8' && message.utf8Data) {
                    this.handleMessage(message.utf8Data);
                }
                if (message.type === 'binary') {
                    this.handleMessage(message.binaryData.toString('utf8'));
                }
            });
        });
    }

    handleMessage(data: string): void {
        try {
            const message: ResponseMessage = JSON.parse(data);

            // Validate message structure
            // if (this.isValidMessage(message)) {
            // Call callback if provided
            if (this.messageCallback) {
                this.messageCallback(message);
            }
            // } else {
            //     console.warn('Invalid message structure:', message);
            // }
        } catch (error) {
            console.error('Error parsing message:', error);
        }
    }

    // isValidMessage(message: any): message is ResponseMessage {
    //     return (
    //         typeof message.client_id === 'string' &&
    //         typeof message.round === 'number' &&
    //         typeof message.c === 'number' &&
    //         typeof message.ts != null
    //     );
    // }

    async connect(): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            const url = `ws://${this.benchmarkObject.websocket_address}:${this.benchmarkObject.websocket_port}`;
            console.log(`Connecting to WebSocket at ${url}`);
            const connectTimeout = setTimeout(() => {
                reject(new Error('Connection timeout'));
            }, 30_000);

            this.client.once('connect', () => {
                clearTimeout(connectTimeout);
                resolve();
            });

            this.client.once('connectFailed', (error) => {
                clearTimeout(connectTimeout);
                reject(error);
            });

            this.client.connect(url);
        });
    }

    setMessageCallback(callback: (message: ResponseMessage) => void): void {
        this.messageCallback = callback;
    }

    disconnect(): void {
        if (this.connection && this.isConnected) {
            this.connection.close();
        }
        this.isConnected = false;
    }

    isConnectionOpen(): boolean {
        return this.isConnected;
    }
}