import {io, Socket} from 'socket.io-client';
import {BenchmarkObj, ResponseMessage} from "../../base/dto";

export default class SocketIOConsumer {
    id: string;
    socket: Socket;
    isConnected: boolean = false;
    benchmarkObject: BenchmarkObj;
    messageCallback?: (message: ResponseMessage) => void;

    constructor(
        id: string,
        benchmarkObject: BenchmarkObj
    ) {
        this.id = id;
        this.benchmarkObject = benchmarkObject;
        const url = `http://${this.benchmarkObject.websocket_address}:${this.benchmarkObject.websocket_port}?token=${this.id}`;
        this.socket = io(url, {
            path: benchmarkObject.extra.path,
            // auth: {
            //     token: this.id
            // }
            transports: ['websocket'],
        });
        console.log(`SocketIOConsumer initialized with URL: ${url} and ID: ${this.id}`);
        this.setupEventHandlers();
    }

    setMessageCallback(callback: (message: ResponseMessage) => void): void {
        console.log("Setting message callback in SocketIOConsumer");
        this.messageCallback = callback;
    }

    setupEventHandlers(): void {
        this.socket.on('connect', () => {
            this.isConnected = true;
        });
        this.socket.on('disconnect', () => {
            this.isConnected = false;
        });
        this.socket.on('order-event', (data: ResponseMessage) => {
            // Process message immediately without storing
            if (this.messageCallback) {
                this.messageCallback(data);
            }
            // No need to store or clear content - let GC handle it
        });
        this.socket.on('user-trade', (data: ResponseMessage) => {
            if (this.messageCallback) {
                this.messageCallback(data);
            }
        });
        this.socket.on('market-data', (data: ResponseMessage) => {
            if (this.messageCallback) {
                this.messageCallback(data);
            }
        });
        this.socket.on('connect_error', (error: any) => {
            console.error('Socket.IO connection failed:', error);
            this.isConnected = false;
        });
    }

    async disconnect(): Promise<void> {
        return new Promise<void>((resolve) => {
            if (this.socket && this.isConnected) {
                // Set up a timeout to prevent hanging
                const disconnectTimeout = setTimeout(() => {
                    console.warn('Socket.IO disconnect timeout, forcing cleanup');
                    this.isConnected = false;
                    resolve();
                }, 5000);

                // Listen for the disconnect event
                this.socket.once('disconnect', () => {
                    clearTimeout(disconnectTimeout);
                    this.isConnected = false;
                    resolve();
                });

                // Initiate the disconnect
                try {
                    this.socket.disconnect();
                } catch (error) {
                    clearTimeout(disconnectTimeout);
                    console.error('Error disconnecting Socket.IO:', error);
                    this.isConnected = false;
                    resolve();
                }
            } else {
                this.isConnected = false;
                resolve();
            }
        });
    }
}
