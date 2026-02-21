import readline from 'readline';
import {client as WebSocketClient} from 'websocket';
import dotenv from 'dotenv';
import FileManager from "./base/file_manager";
import RoundingResultsWriter from "./base/roundingResultsWriter";
import ConnectionManager from "./senarios/request_response/connection_manager";
import {BenchmarkManager, BenchmarkObj, ConnectionObj, ProgressObj} from "./base/dto";
import KafkaProducerManager from "./senarios/kafka/kafka_connection_manager";
import WebSocketConsumerManager from "./senarios/ws_consumer/ws_consumer_manager";

dotenv.config();

class Benchmarker {
    benchmark_scenario: string | undefined;
    connection_progress_obj: ProgressObj;
    benchmark_progress_obj: ProgressObj;
    connection_obj: ConnectionObj;
    benchmark_obj: BenchmarkObj;
    fm: FileManager | null;
    result: RoundingResultsWriter | null;
    cm: BenchmarkManager;
    ROUNDS: number;

    constructor() {
        /**
         * An object storing data on the connections currently being made each round
         * {
         *      counter: {number} the number of clients currently created each round,
         *      total: {number} the total number of clients expected to me created each round,
         *      message: {string} the message to output before starting the connection process
         * }
         * @type {Object}
         */
        this.connection_progress_obj = {
            counter: 0,
            total: 0,
            message: "Connecting..."
        };

        /**
         * An object storing data on all the requests currently being made each round
         * {
         *      counter: {number} the number of requests currently completed each round,
         *      total: {number} the total number of requests expected to me completed each round,
         *      message: {string} the message to output before starting the benchmarking process
         * }
         * @type {Object}
         */
        this.benchmark_progress_obj = {
            counter: 0,
            total: 0,
            message: "Benchmarking..."
        };

        /**
         * An object storing websocket client connections and connection data
         * {
         *      connection_time: {number} the total time it took for all the clients to connect each round
         *      times: {Array}, time data produces by each client for each request to the websocket server
         clients: {Array} list of all connected clients
         * }
         * @type {Object}
         */
        this.connection_obj = {
            connection_time: 0,
            times: [],
            clients: []
        };
        this.benchmark_obj = {
            websocket_address: process.env.WEBSOCKET_ADDRESS || "127.0.0.1",
            websocket_port: Number(process.env.WEBSOCKET_PORT) || 8080,
            connection_interval: Number(process.env.ADD_CONNECTIONS) || 100,
            request_interval: Number(process.env.REQUESTS) || 50,
            extra: null
        };
        this.fm = null;
        this.result = null;
        this.cm = new ConnectionManager(
            this.benchmark_obj,
            this.connection_obj,
            this.connection_progress_obj,
            this.benchmark_progress_obj
        );
        this.ROUNDS = Number(process.env.ROUNDS) || 25;

    }

    /**
     * Creates the appropriate benchmark manager based on the selected scenario
     * @param scenario {string} The selected scenario ('request-response' or 'kafka')
     */
    createBenchmarkManager(scenario: string) {
        switch (scenario) {
            case 'request-response':
                this.cm = new ConnectionManager(
                    this.benchmark_obj,
                    this.connection_obj,
                    this.connection_progress_obj,
                    this.benchmark_progress_obj
                );
                break;
            case 'producer-kafka':
                this.cm = new KafkaProducerManager(
                    this.benchmark_obj,
                    this.connection_obj,
                    this.connection_progress_obj,
                    this.benchmark_progress_obj
                );
                break;
            case 'ws-consumer':
                this.cm = new WebSocketConsumerManager(
                    'ws-consumer',
                    this.benchmark_obj,
                    1
                );
                break;
            case 'socketio-consumer':
                this.cm = new WebSocketConsumerManager(
                    'socketio-consumer',
                    this.benchmark_obj,
                    1
                );
                break;
            default:
                console.log("Invalid scenario. Please choose 'request-response' or 'kafka'");
                return;
        }

    }

    /**
     * Prompts the user at the beginning of the application for the current language being benchmarked
     * and the scenario to run
     * @returns {void}
     */
    prompt() {
        console.log("Start test: ", Date.now())
        // allows this to be used inside nested functions
        let self = this;

        // create a readline interface for console input/output
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });

        // continue running the programming asynchronously
        let run_benchmark = async function () {
            rl.close();

            await self.cm.runBenchmark(self.ROUNDS);

            console.log("End test: ", Date.now());
        };

        let prompt_scenario = function () {
            if (self.benchmark_scenario === undefined) {
                console.log("\nAvailable scenarios:");
                console.log("1. request-response - Standard WebSocket request/response benchmark");
                console.log("2. kafka - Publish to kafka only");
                console.log("3. websocket-consumer - Consume WebSocket messages only");
                console.log("4. socketio-consumer - Consume Socket.IO messages only");

                rl.question('Select scenario:', async (scenario) => {
                    switch (scenario.toLowerCase()) {
                        case 'request-response':
                        case '1':
                            scenario = 'request-response';
                            break;
                        case 'producer-kafka':
                        case '2':
                            scenario = 'producer-kafka';
                            break;
                        case 'ws-consumer':
                        case '3':
                            scenario = 'ws-consumer';
                            break;
                        case 'socketio-consumer':
                        case '4':
                            scenario = 'socketio-consumer';
                            break;
                        default:
                            console.log("Invalid scenario. Please choose 1,2,3,4 or type the scenario name.");
                            return prompt_scenario();
                    }
                    self.benchmark_scenario = scenario;
                    self.createBenchmarkManager(scenario);
                    run_benchmark();
                });
            } else {
                self.createBenchmarkManager(self.benchmark_scenario);
                run_benchmark();
            }
        };

        prompt_scenario();
    }

    /**
     * Performs the benchmarking process
     * @param round {number} The current iteration count of the round being performed
     * @return {Promise} resolves once the round of the benchmarking process is complete
     */
    // async benchmark(round: number) {
    //     return new Promise(async (resolve, reject) => {
    //         try {
    //
    //             // determine the total number of expected connections
    //             // REQUEST_INTERVAL * ROUND_NUMBER
    //             this.connection_progress_obj.total = (round + 1) * this.benchmark_obj.connection_interval;
    //
    //             // start the connection progress bar
    //             let connection_bar = new ProgressBar(this.connection_progress_obj);
    //             connection_bar.start();
    //
    //             // begin the connection process, and wait for it to finish
    //             await this.cm.createConnections(round);
    //
    //             // finalize the progress bar, and stop it from updating any further
    //             this.connection_progress_obj.bar.update(this.connection_progress_obj.counter);
    //             connection_bar.stop();
    //
    //             // output to the conole the time elapse for the new connections to connect
    //             console.log("\nConnection Time: " + this.connection_obj.connection_time);
    //
    //             // start the benchmarking progress bar
    //             this.benchmark_progress_obj.total = this.benchmark_obj.request_interval * this.connection_obj.clients.length;
    //             let benchmark_bar = new ProgressBar(this.benchmark_progress_obj);
    //             benchmark_bar.clear();
    //             benchmark_bar.start();
    //
    //             // start the benchmarking process, and wait for it to finish
    //             await this.cm.sendRequests(round);
    //
    //             // stop the progress bar from updating any further
    //             benchmark_bar.stop();
    //
    //             // calculate the results for the current round of benchmarking
    //             if (this.result) {
    //                 await this.result.calculate(this.connection_obj);
    //             }
    //
    //             // resolve when done
    //             resolve(void 0);
    //         } catch (error) {
    //             reject(error);
    //         }
    //     });
    // }

    /**
     * Checks if there is a websocket server accepting connections on the given IP and Port. If no server found,
     * the application terminates
     *
     * @return {Promise} Resolves when a connection is made to the websocket server, otherwise terminates
     */
    async serverCheck() {

        // connect to the websocket server
        let url = "ws://" + this.benchmark_obj.websocket_address + ":" + this.benchmark_obj.websocket_port;
        let client = new WebSocketClient();
        client.connect(url);

        return new Promise(async (resolve, reject) => {

            // terminate the program is the connection is unsuccessful
            client.on('connectFailed', function (error) {
                console.log("Server Not Found");
                process.exit();
            });

            // resolve on a successful connection
            client.on('connect', function (connection) {
                connection.close();
                resolve(void 0);
            });
        });

    }
}

new Benchmarker().prompt();
