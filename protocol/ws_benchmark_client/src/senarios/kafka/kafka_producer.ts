import {Kafka, Producer} from 'kafkajs';
import {BenchmarkClient, BenchmarkObj, createStringOfSizeKB, ProgressObj, RequestMessage} from "../../base/dto";
import {clearInterval} from "node:timers";

export interface KafkaProducerConfig {
    brokers: string;
    client_id: string;
    topic: string;
}

export interface KafkaMessage {
    value: string;
    key?: string | null;
}

class KafkaProducer implements BenchmarkClient {
    id: string;
    kafka_config: KafkaProducerConfig;
    kafka: Kafka;
    times: any[];
    count: number;
    last_count: number[];
    producer: Producer;
    connected: boolean;
    benchmark_obj: BenchmarkObj;
    connection_progress_obj: ProgressObj;
    benchmark_progress_obj: ProgressObj;

    constructor(
        id: string,
        benchmark_obj: BenchmarkObj,
        connection_progress_obj: ProgressObj,
        benchmark_progress_obj: ProgressObj,
        kafka_config: KafkaProducerConfig
    ) {
        this.id = id;
        this.kafka_config = kafka_config;
        this.times = [];
        this.count = 0;
        this.last_count = new Array(20);
        this.kafka = new Kafka({
            clientId: kafka_config.client_id,
            brokers: kafka_config.brokers.split(','),
            retry: {
                retries: 0
            }
        });
        this.producer = this.kafka.producer();
        this.connected = false;

        this.benchmark_obj = benchmark_obj;
        this.connection_progress_obj = connection_progress_obj;
        this.benchmark_progress_obj = benchmark_progress_obj;
    }

    async connect(): Promise<void> {
        try {
            await this.producer.connect();
            this.connected = true;
            this.connection_progress_obj.counter++;
            console.log(`Kafka producer ${this.id} connected successfully`);
        } catch (error) {
            this.connected = false;
            console.error(`Failed to connect Kafka producer ${this.id}:`, error);
            throw new Error(`Kafka broker connection failed: ${error}`);
        }
    }

    async close() {
        await this.producer.disconnect();
        this.connected = false;
        return Promise.resolve();
    }

    async disconnect(): Promise<void> {
        if (this.connected) {
            await this.producer.disconnect();
            this.connected = false;
            console.log('Kafka producer disconnected');
        }
    }

    async publishMessage(message: string, key: string | null = null): Promise<number> {
        const startTime = Date.now();
        try {
            await this.producer.send({
                topic: this.kafka_config.topic,
                messages: [{
                    key: key,
                    value: message,
                    timestamp: Date.now().toString()
                }]
            });
            const endTime = Date.now();
            return endTime - startTime;
        } catch (error) {
            console.error('Error publishing to Kafka:', error);
            throw error;
        }
    }

    markAsDone(message_marker: number) {
        let received_timestamp = Date.now();
        let self = this;
        if (self.times[message_marker]['received'] === undefined
            && self.times[message_marker]['finish'] === undefined) {

            // store the corresponding timestamps in the times array
            self.times[message_marker]['received'] = received_timestamp;
            self.times[message_marker]['finish'] = Date.now();

            // increment the successful request counters by 1
            self.benchmark_progress_obj.counter++;
            self.count++;
        }
    }

    async sendData(round: number): Promise<any[]> {
        // Set batch size, can be from config or a default value
        const batchSize = 100;
        const total = this.benchmark_obj.request_interval;
        const batches: any[][] = [];
        // Prepare all messages and times
        for (let i = 0; i < total; i++) {
            this.times[i] = {'start': Date.now()};
        }
        const messageSizeKB = Number(process.env.MESSAGE_SIZE_KB);
        const content = createStringOfSizeKB(messageSizeKB, 'a');
        // Group messages into batches
        for (let i = 0; i < total; i += batchSize) {
            const batch: { key: string, value: string, timestamp: string, _index: number }[] = [];
            for (let j = i; j < Math.min(i + batchSize, total); j++) {
                const request: RequestMessage = {
                    client_id: this.id,
                    round: round,
                    c: j,
                    content: content
                };
                batch.push({
                    key: this.id,
                    value: JSON.stringify(request),
                    timestamp: Date.now().toString(),
                    _index: j // keep track of original index for markAsDone
                });
            }
            batches.push(batch);
        }
        return new Promise<any[]>((resolve, _reject) => {
            let batchCount = 0;
            const sendBatch = async (batchIdx: number) => {
                if (!this.producer) {
                    resolve([]);
                    return;
                }
                const batch = batches[batchIdx];
                // Remove _index before sending to Kafka
                const kafkaMessages = batch.map(({_index, ...msg}) => msg);
                try {
                    await this.producer.send({
                        topic: this.kafka_config.topic,
                        messages: kafkaMessages
                    });
                    // Mark all messages in this batch as done
                    batch.forEach(msg => this.markAsDone(msg._index));
                } catch (error) {
                    console.error('Kafka send error:', error);
                }
                batchCount++;
                if (batchCount < batches.length) {
                    sendBatch(batchCount);
                }
            };
            // Start sending batches
            if (batches.length > 0) {
                sendBatch(0);
            }
            // Wait for all messages to be marked as done
            const self = this;
            let timer = 0;
            const finishCount = setInterval(function () {
                let readyToResolve = self.times.every(function (time: any, _message_index: number) {
                    return time['finish'] !== undefined;
                });
                if (readyToResolve
                    || ((self.count / self.benchmark_obj.request_interval) === 1)
                    || (self.count === self.last_count[0]
                        && (((self.count / self.benchmark_obj.request_interval) > .9)
                            || (timer++ >= 100)
                        ))) {
                    clearInterval(finishCount);
                    resolve(self.times);
                }
                self.last_count.push(self.count);
            }, 1000);
        });
    }

}

export default KafkaProducer;
