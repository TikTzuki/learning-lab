import FileManager from "./file_manager";
import {ConsumeResult} from "../senarios/ws_consumer/ws_consumer_manager";

export class ConsumerResultsWriter {
    file_manager: FileManager;

    constructor(file_manager: FileManager) {
        this.file_manager = file_manager;
    }

    async writeReport(result: ConsumeResult) {
        let client_length = Object.keys(result).length;
        let count = 0;
        let start_time = 0;
        let end_time = 0;
        let latencies: number[] = [];

        for (const client_id of Object.keys(result)) {
            const messages = result[client_id];
            for (let i = 0; i < messages.length; i++) {
                let m = messages[i];
                if (start_time === 0 || m.ts < start_time) {
                    start_time = m.ts;
                }
                if (end_time === 0 || m.client_receive_ts > end_time) {
                    end_time = m.client_receive_ts;
                }

                // Calculate latency if both timestamps are available
                if (m.ts && m.client_receive_ts) {
                    const latency = m.client_receive_ts - m.ts;
                    latencies.push(latency);
                }
            }
            count += messages.length;
        }

        // Calculate latency statistics
        let longest = 0;
        let shortest = 0;
        let average = 0;

        if (latencies.length > 0) {
            longest = latencies[0];
            shortest = latencies[0];
            let sum = 0;
            for (const latency of latencies) {
                if (latency > longest) {
                    longest = latency;
                }
                if (latency < shortest) {
                    shortest = latency;
                }
                sum += latency;
            }
            average = sum / latencies.length;
        }

        let data = {
            "clients": client_length,
            "count": count,
            "total": count,
            "percentage": 0,
            "time": end_time - start_time,
            "longest": longest,
            "shortest": shortest,
            "average": average,
            "connection_time": 0
        }
        await this.file_manager.saveDataToFile(data);
    }

}

export default ConsumerResultsWriter;
