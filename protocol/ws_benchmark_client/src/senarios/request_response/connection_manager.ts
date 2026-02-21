import BenchmarkWsClient from './ws_connection';
import {BenchmarkManager, BenchmarkObj, ConnectionObj, ProgressObj, ResultWriter} from "../../base/dto";
import ProgressBar from "../../base/progress_bar";
import RoundingResultsWriter from "../../base/roundingResultsWriter";
import FileManager from "../../base/file_manager";

class ConnectionManager implements BenchmarkManager {
    connected: (number | undefined)[];
    benchmark_obj: BenchmarkObj;
    connection_obj: ConnectionObj;
    connection_progress_obj: ProgressObj;
    benchmark_progress_obj: ProgressObj;
    fm: FileManager;
    result_writer: ResultWriter

    constructor(
        benchmark_obj: BenchmarkObj,
        connection_obj: ConnectionObj,
        connection_progress_obj: ProgressObj,
        benchmark_progress_obj: ProgressObj
    ) {
        this.connected = new Array(benchmark_obj.connection_interval);
        this.benchmark_obj = benchmark_obj;
        this.connection_obj = connection_obj;
        this.connection_progress_obj = connection_progress_obj;
        this.benchmark_progress_obj = benchmark_progress_obj;
        this.fm = new FileManager("request_response");
        this.result_writer = new RoundingResultsWriter(this.fm, this.benchmark_obj.request_interval);
    }

    createConnections(round: number): Promise<void> {
        let existing_client_count = this.benchmark_obj.connection_interval * round;
        let new_client_count = this.benchmark_obj.connection_interval * (round + 1) - 1;
        this.connected[new_client_count] = undefined;
        return new Promise((resolve, reject) => {
            let connection_start = Date.now();
            for (let i = 0; i < this.benchmark_obj.connection_interval; i++) {
                let client_index = existing_client_count + i;

                let client = new BenchmarkWsClient(this.benchmark_obj, this.connection_progress_obj, this.benchmark_progress_obj);
                this.connection_obj.clients[client_index] = client;
                client.connect().then(() => {
                    this.connected[client_index] = 1;
                    if (!this.connected.includes(undefined)) {
                        this.connection_obj.connection_time = Date.now() - connection_start;
                        resolve();
                    }
                });
            }
        });
    }

    sendRequests(round: number): Promise<void> {
        this.connection_obj.times = new Array(this.benchmark_obj.connection_interval * (round + 1));
        return new Promise((resolve, reject) => {
            for (let i = 0; i < this.connection_obj.clients.length; i++) {
                (this.connection_obj.clients[i] as BenchmarkWsClient).sendData(round)
                    .then((time: any) => {
                        this.connection_obj.times[i] = time;
                        if (!this.connection_obj.times.includes(undefined)) {
                            resolve();
                        }
                    });
            }
        });
    }

    async close() {
        for (let i = 0; i < this.connection_obj.clients.length; i++) {
            (this.connection_obj.clients[i] as BenchmarkWsClient).close();
        }
    }

    async runBenchmark(rounds: number): Promise<void> {
        await this.fm.createFile();

        for (let i = 0; i < rounds; i++) {
            // Set up progress bar for connections
            this.connection_progress_obj.total = (i + 1) * this.benchmark_obj.connection_interval;
            let connection_bar = new ProgressBar(this.connection_progress_obj);
            connection_bar.start();
            await this.createConnections(i);
            this.connection_progress_obj.bar.update(this.connection_progress_obj.counter);
            connection_bar.stop();
            // Output connection time
            console.log("\nConnection Time: " + this.connection_obj.connection_time);
            // Set up progress bar for benchmarking
            this.benchmark_progress_obj.total = this.benchmark_obj.request_interval * this.connection_obj.clients.length;
            let benchmark_bar = new ProgressBar(this.benchmark_progress_obj);
            benchmark_bar.clear();
            benchmark_bar.start();

            await this.sendRequests(i);
            benchmark_bar.stop();
            await this.writeResults();
        }
        // once all round have been completed, close the websocket connections
        await this.close();
    }

    async writeResults(): Promise<void> {
        await this.result_writer.calculate(this.connection_obj);
    }
}

export default ConnectionManager;
