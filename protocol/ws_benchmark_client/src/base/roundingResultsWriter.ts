import FileManager from "./file_manager";
import {ConnectionObj, ResultWriter} from "./dto";

class RoundingResultsWriter implements ResultWriter {
    file_manager: FileManager;
    request_interval: number;

    constructor(file_manager: FileManager, request_interval: number) {
        this.file_manager = file_manager;
        this.request_interval = request_interval;
    }

    async calculate(stats: ConnectionObj): Promise<void> {
        let client_length = stats.clients.length;
        let times = stats.times;
        let connection_time = stats.connection_time;
        let start_time = Math.floor(new Date(8640000000000000).getTime() / 1000);
        let stop_time = 0;
        let longest_rt = 0;
        let shortest_rt = Number.MAX_SAFE_INTEGER;
        let total_rt = 0;
        let count = 0;
        times.forEach((client_time: any[], key: number) => {
            client_time.forEach((trip: any) => {
                if (trip['start'] !== undefined && trip['received'] !== undefined && trip['finish'] !== undefined) {
                    if (trip['start'] < start_time) {
                        start_time = trip['start'];
                    }
                    if (trip['finish'] > stop_time) {
                        stop_time = trip['finish'];
                    }
                    let trip_time = trip['finish'] - trip['start'];
                    if (trip_time > longest_rt) {
                        longest_rt = trip_time;
                    }
                    if (trip_time < shortest_rt) {
                        shortest_rt = trip_time;
                    }
                    total_rt += trip_time;
                    count++;
                }
            })
        });
        let average_rt = total_rt / count;
        let time_elapse = stop_time - start_time;
        console.log("Count: " + count + "/" + (this.request_interval * client_length) + " (" + count / (this.request_interval * client_length) * 100 + "% ) " + " | Time Elapse: " + time_elapse);
        console.log("Longest Trip: " + longest_rt + " | Shortest Trip: " + shortest_rt + " | Average Trip: " + average_rt);
        let data = {
            "clients": client_length,
            "count": count,
            "total": (this.request_interval * client_length),
            "percentage": count / (this.request_interval * client_length) * 100 + "%",
            "time": time_elapse,
            "longest": longest_rt,
            "shortest": shortest_rt,
            "average": average_rt,
            "connection_time": connection_time
        };
        this.file_manager.saveDataToFile(data).then(() => {
            return new Promise((resolve) => {
                resolve(undefined);
            });
        });
    }
}

export default RoundingResultsWriter;
