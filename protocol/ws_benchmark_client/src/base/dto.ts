export interface RequestMessage {
    client_id: string;
    round: number
    c: number;
    content: string; // Add optional content field
}

export interface ResponseMessage {
    client_id: string;
    round: number
    c: number;
    ts: number;
    content?: string;
}

export interface ProgressObj {
    counter: number;
    total: number;
    message: string;
    bar?: any;
}

export interface BenchmarkClient {
    connect(): Promise<void>;

    close(): Promise<void>;
}

export interface BenchmarkManager {
    createConnections(round: number): Promise<void>;

    sendRequests(round: number): Promise<void>;

    close(): Promise<void>;

    runBenchmark(ROUNDS: number): Promise<void>;
}

export interface ResultWriter {
    calculate(connection_obj: ConnectionObj): Promise<void>;
}

export interface ConnectionObj {
    connection_time: number;
    times: any[];
    clients: BenchmarkClient[];
}

export interface BenchmarkObj {
    websocket_address: string;
    websocket_port: number;
    connection_interval: number;
    request_interval: number;
    request_timeout?: number;
    extra: any;
}

// Utility function to create a string of specific size in KB
export function createStringOfSizeKB(sizeKB: number, character: string = 'a'): string {
    const sizeBytes = sizeKB * 1024; // Convert KB to bytes
    return character.repeat(sizeBytes);
}