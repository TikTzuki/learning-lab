import * as fs from 'fs';

class FileManager {
    benchmark_folder: string;
    save_file: string;

    constructor(scenarios: string) {
        let benchmark_results_directory = process.env.BENCHMARK_FOLDER || "./benchmarks";
        this.benchmark_folder = benchmark_results_directory + "/" + scenarios + "/";
        this.save_file = "";
    }

    async getLastFile(): Promise<string> {
        return new Promise((resolve) => {
            const last_file = "0_0.csv";
            try {
                if (!fs.existsSync(this.benchmark_folder)) {
                    fs.mkdirSync(this.benchmark_folder, {recursive: true});
                }
                fs.readdir(this.benchmark_folder, (err: NodeJS.ErrnoException | null, files: string[]) => {
                    if (err) {
                        resolve(last_file);
                        return;
                    }
                    if (files && files.length > 0) {
                        resolve(files[files.length - 1]);
                    } else {
                        resolve(last_file);
                    }
                });
            } catch (e) {
                resolve(last_file);
            }
        });
    }

    async createFile(): Promise<void> {
        const file = await this.getLastFile();
        let found: RegExpMatchArray | null;
        if (file === undefined) {
            found = ['0_0.csv', '0', '0'];
        } else {
            const regex = /(?<test>\d+)_(?<run>\d+)\.csv/;
            found = file.match(regex);
        }

        if (found && found.length >= 2) {
            const run_count = 1;
            const testNumber = parseInt(found[1]) + 1;
            this.save_file = this.benchmark_folder + "/" + testNumber.toString() + "_" + run_count + ".csv";
        } else {
            this.save_file = this.benchmark_folder + "/1_1.csv";
        }
    }

    saveDataToFile(data: Record<string, any>): Promise<void> {
        const newLine = '\r\n';
        const fields = Object.keys(data) + newLine;
        console.log("Saving data to file:", this.save_file);
        return new Promise((resolve, reject) => {
            let self: FileManager = this;
            fs.stat(this.save_file, (err: NodeJS.ErrnoException | null, stat?: fs.Stats) => {
                if (err == null) {
                    // File exists, append data
                    const csv = Object.keys(data).map((k) => data[k]).join(",") + newLine;
                    fs.appendFile(this.save_file, csv, (appendErr: NodeJS.ErrnoException | null) => {
                        if (appendErr) {
                            reject(appendErr);
                        } else {
                            resolve();
                        }
                    });
                } else {
                    // File doesn't exist, create it with headers first
                    fs.writeFile(this.save_file, fields, (writeErr: NodeJS.ErrnoException | null) => {
                        if (writeErr) {
                            reject(writeErr);
                        } else {
                            // Then append the data
                            const csv = Object.keys(data).map((k) => data[k]).join(",") + newLine;
                            fs.appendFile(this.save_file, csv, (appendErr: NodeJS.ErrnoException | null) => {
                                if (appendErr) {
                                    reject(appendErr);
                                } else {
                                    resolve();
                                }
                            });
                        }
                    });
                }
            });
        });
    }
}

export default FileManager;