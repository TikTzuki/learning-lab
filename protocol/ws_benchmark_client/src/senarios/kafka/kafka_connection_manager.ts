import KafkaProducer, {KafkaProducerConfig} from './kafka_producer';
import {
    BenchmarkClient,
    BenchmarkManager,
    BenchmarkObj,
    ConnectionObj,
    ProgressObj,
    ResultWriter
} from '../../base/dto';
import ProgressBar from "../../base/progress_bar";
import FileManager from "../../base/file_manager";
import RoundingResultsWriter from "../../base/roundingResultsWriter";

class KafkaProducerManager implements BenchmarkManager {
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
        benchmark_progress_obj: ProgressObj,
    ) {
        this.benchmark_obj = benchmark_obj;
        this.connection_obj = connection_obj;
        this.connection_progress_obj = connection_progress_obj;
        this.benchmark_progress_obj = benchmark_progress_obj;
        this.fm = new FileManager("producer_kafka");
        this.result_writer = new RoundingResultsWriter(this.fm, this.benchmark_obj.request_interval);

    }

    loadKafkaConfig(): KafkaProducerConfig {
        return {
            brokers: `${this.benchmark_obj.websocket_address}:${this.benchmark_obj.websocket_port}` || process.env.KAFKA_BROKERS || 'localhost:9092',
            client_id: process.env.KAFKA_CLIENT_ID || 'benchmark-client',
            topic: this.benchmark_obj.extra.kafkaTopic || process.env.KAFKA_TOPIC || 'benchmark-topic',
        }
    }

    async createConnections(round: number): Promise<void> {
        // For Kafka, 'connections' are producer connections
        let connection_start = Date.now();
        let kafka_producers: BenchmarkClient[] = [];
        for (let i = 0; i < this.benchmark_obj.connection_interval; i++) {
            let producer = new KafkaProducer(
                i.toString(),
                this.benchmark_obj,
                this.connection_progress_obj,
                this.benchmark_progress_obj,
                this.loadKafkaConfig(),
            )
            await producer.connect().then(() => {
                this.connection_obj.connection_time = Date.now() - connection_start;
            })
            kafka_producers.push(producer);
        }
        this.connection_obj.clients = kafka_producers;
    }

    async sendRequests(round: number): Promise<void> {
        // Increase messages each round, same as request_response
        let promises: Promise<number[]>[] = [];
        for (let i = 0; i < this.connection_obj.clients.length; i++) {
            let producer = this.connection_obj.clients[i] as KafkaProducer;
            promises.push(producer.sendData(round));
        }
        this.connection_obj.times = await Promise.all(promises);
    }

    async close() {
        for (let producer of this.connection_obj.clients) {
            await producer.close();
        }
    }

    async runBenchmark(rounds: number): Promise<void> {
        await this.fm.createFile();

        for (let i = 0; i < rounds; i++) {
            this.connection_progress_obj.total = (i + 1) * this.benchmark_obj.connection_interval;
            let connection_bar = new ProgressBar(this.connection_progress_obj);
            connection_bar.start();
            await this.createConnections(i);
            this.connection_progress_obj.bar?.update(this.connection_progress_obj.counter);
            connection_bar.stop();
            console.log("\nConnection Time: " + this.connection_obj.connection_time);
            this.benchmark_progress_obj.total = this.benchmark_obj.request_interval * this.connection_obj.clients.length;
            let benchmark_bar = new ProgressBar(this.benchmark_progress_obj);
            benchmark_bar.clear();
            benchmark_bar.start();
            await this.sendRequests(i);
            benchmark_bar.stop();
            await this.writeResults();
        }

        await this.close();
    }

    async writeResults(): Promise<void> {
        await this.result_writer.calculate(this.connection_obj);
    }
}

export default KafkaProducerManager;
