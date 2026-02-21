const fs = require('fs');
const path = require('path');

class BenchmarkReporter {
    constructor() {
        this.benchmarkDir = path.join(__dirname, '../benchmarks');
        this.templatePath = path.join(__dirname, 'report_template.md');
        this.outputDir = path.join(__dirname, '../reports');
    }

    // Get the latest CSV file from benchmarks/rust directory
    getLatestCsvFile() {
        try {
            const files = fs.readdirSync(this.benchmarkDir)
                .filter(file => file.endsWith('.csv'))
                .map(file => ({
                    name: file,
                    path: path.join(this.benchmarkDir, file),
                    mtime: fs.statSync(path.join(this.benchmarkDir, file)).mtime
                }))
                .sort((a, b) => b.mtime - a.mtime);

            if (files.length === 0) {
                throw new Error('No CSV files found in benchmarks/rust directory');
            }

            return files[0];
        } catch (error) {
            console.error('Error reading benchmark directory:', error);
            throw error;
        }
    }

    // Parse CSV data and calculate metrics
    parseCsvData(csvPath) {
        try {
            const csvContent = fs.readFileSync(csvPath, 'utf8');
            const lines = csvContent.trim().replace(/\r\n/g, '\n').replace(/\r/g, '\n').split('\n');
            const headers = lines[0].split(',').map(h => h.trim());

            const data = lines.slice(1).map(line => {
                const values = line.split(',').map(v => v.trim());
                const row = {};
                headers.forEach((header, index) => {
                    row[header] = values[index] || '';
                });
                return row;
            });

            return {headers, data};
        } catch (error) {
            console.error('Error parsing CSV file:', error);
            throw error;
        }
    }

    // Calculate performance metrics from parsed data
    calculateMetrics(data) {
        const numericData = data.map(row => ({
            clients: parseInt(row.clients),
            count: parseInt(row.count),
            total: parseInt(row.total),
            percentage: parseFloat(row.percentage.replace('%', '')),
            time: parseFloat(row.time),
            longest: parseFloat(row.longest),
            shortest: parseFloat(row.shortest),
            average: parseFloat(row.average),
            connection_time: parseFloat(row.connection_time)
        }));

        const maxConnections = Math.max(...numericData.map(d => d.clients));
        const totalMessages = numericData.reduce((sum, d) => sum + d.total, 0);
        const totalTime = numericData.reduce((sum, d) => sum + d.time, 0);

        // Calculate throughput (messages per second)
        const avgThroughput = totalMessages / totalTime;

        // Calculate latency percentiles (approximated from average and min/max)
        const latencies = numericData.map(d => d.average);
        latencies.sort((a, b) => a - b);

        const p50 = this.calculatePercentile(latencies, 50);
        const p75 = this.calculatePercentile(latencies, 75);
        const p95 = this.calculatePercentile(latencies, 95);
        const p99 = this.calculatePercentile(latencies, 99);

        // Calculate success rate (assuming 100% based on percentage field)
        const avgSuccessRate = numericData.reduce((sum, d) => sum + d.percentage, 0) / numericData.length;
        const errorRate = 100 - avgSuccessRate;

        // Calculate connection establishment rate
        const avgConnectionTime = numericData.reduce((sum, d) => sum + d.connection_time, 0) / numericData.length;
        const connectionRate = 1000 / avgConnectionTime; // connections per second

        return {
            maxConnections: maxConnections || 0,
            totalMessages: totalMessages || 0,
            throughput: isNaN(avgThroughput) ? '0' : avgThroughput.toFixed(2),
            p50Latency: isNaN(p50) ? '0' : p50.toFixed(2),
            p75Latency: isNaN(p75) ? '0' : p75.toFixed(2),
            p95Latency: isNaN(p95) ? '0' : p95.toFixed(2),
            p99Latency: isNaN(p99) ? '0' : p99.toFixed(2),
            successRate: isNaN(avgSuccessRate) ? '0' : avgSuccessRate.toFixed(2),
            errorRate: isNaN(errorRate) ? '0' : errorRate.toFixed(2),
            connectionRate: isNaN(connectionRate) ? '0' : connectionRate.toFixed(2),
            avgConnectionTime: isNaN(avgConnectionTime) ? '0' : avgConnectionTime.toFixed(2),
            latencyDistribution: this.generateLatencyDistribution(numericData),
            detailedTable: this.generateDetailedTable(numericData)
        };
    }

    calculatePercentile(sortedArray, percentile) {
        const index = (percentile / 100) * (sortedArray.length - 1);
        const lower = Math.floor(index);
        const upper = Math.ceil(index);

        if (lower === upper) {
            return sortedArray[lower];
        }

        const weight = index - lower;
        return sortedArray[lower] * (1 - weight) + sortedArray[upper] * weight;
    }

    generateLatencyDistribution(data) {
        return data.map(row =>
            `${row.clients} clients: avg ${row.average.toFixed(2)}ms, min ${row.shortest}ms, max ${row.longest}ms`
        ).join('\n');
    }

    generateDetailedTable(data) {
        let table = '| Clients | Messages | Success Rate | Avg Latency (ms) | Min (ms) | Max (ms) | Connection Time (ms) |\n';
        table += '|---------|----------|--------------|------------------|----------|----------|---------------------|\n';

        data.forEach(row => {
            table += `| ${row.clients} | ${row.total.toLocaleString()} | ${row.percentage.toFixed(1)}% | ${row.average.toFixed(2)} | ${row.shortest} | ${row.longest} | ${row.connection_time} |\n`;
        });

        return table;
    }

    generatePerformanceMetrics(metrics) {
        return `| Metric | Value |
|--------|-------|
| Max Concurrent Connections | ${metrics.maxConnections} |
| Message Throughput (avg) | ${metrics.throughput} msg/s |
| P50 Latency | ${metrics.p50Latency} ms |
| P75 Latency | ${metrics.p75Latency} ms |
| P95 Latency | ${metrics.p95Latency} ms |
| P99 Latency | ${metrics.p99Latency} ms |
| Connection Establishment Rate | ${metrics.connectionRate} conn/s |
| Message Success Rate | ${metrics.successRate}% |
| Message Error Rate | ${metrics.errorRate}% |
| Avg Connection Time | ${metrics.avgConnectionTime} ms |`;
    }

    generateObservations(metrics, data) {
        const observations = [];

        // Analyze throughput trends
        if (parseFloat(metrics.throughput) > 1000) {
            observations.push("• High message throughput achieved, indicating good server performance");
        }

        // Analyze latency
        if (parseFloat(metrics.p95Latency) < 100) {
            observations.push("• Low P95 latency indicates consistent response times");
        } else if (parseFloat(metrics.p95Latency) > 500) {
            observations.push("• High P95 latency suggests potential performance bottlenecks");
        }

        // Analyze success rate
        if (parseFloat(metrics.successRate) >= 99.9) {
            observations.push("• Excellent message delivery success rate");
        }

        // Analyze scaling behavior
        const firstRow = data[0];
        const lastRow = data[data.length - 1];
        const latencyIncrease = ((lastRow.average - firstRow.average) / firstRow.average) * 100;

        if (latencyIncrease < 50) {
            observations.push("• Good scaling characteristics - latency increases moderately with load");
        } else {
            observations.push("• Latency increases significantly under high load - consider optimization");
        }

        return observations.length > 0 ? observations.join('\n') : "• No specific observations noted";
    }

    generatePerformanceSummary(metrics) {
        const summary = [];
        summary.push(`**Peak Performance:** ${metrics.maxConnections} concurrent connections`);
        summary.push(`**Throughput:** ${metrics.throughput} messages/second`);
        summary.push(`**Latency (P95):** ${metrics.p95Latency}ms`);
        summary.push(`**Reliability:** ${metrics.successRate}% success rate`);

        return summary.join('  \n');
    }

    // Generate the report
    generateReport() {
        try {
            console.log('🔍 Finding latest benchmark file...');
            const latestFile = this.getLatestCsvFile();
            console.log(`📄 Using file: ${latestFile.name} (modified: ${latestFile.mtime.toISOString()})`);

            console.log('📊 Parsing benchmark data...');
            const {data} = this.parseCsvData(latestFile.path);

            console.log('⚡ Calculating performance metrics...');
            const metrics = this.calculateMetrics(data);

            console.log('📝 Loading report template...');
            const template = fs.readFileSync(this.templatePath, 'utf8');

            console.log('🔧 Generating report content...');
            const reportDate = new Date().toISOString().split('T')[0].replace(/-/g, '');
            const testDate = latestFile.mtime.toISOString().split('T')[0].replace(/-/g, '');

            // Replace template placeholders
            const report = template
                .replace(/{{APP_NAME}}/g, 'Rust WebSocket Server')
                .replace(/{{REPORT_DATE}}/g, reportDate)
                .replace(/{{DATA_SOURCE}}/g, latestFile.name)
                .replace(/{{TEST_DATE}}/g, testDate)
                .replace(/{{SERVER_SPEC}}/g, 'Rust WebSocket Server')
                .replace(/{{MEMORY_SPEC}}/g, 'System Default')
                .replace(/{{OS_SPEC}}/g, process.platform)
                .replace(/{{RUNTIME_SPEC}}/g, 'Rust/Tokio')
                .replace(/{{LOAD_TOOL}}/g, 'Custom Benchmark Client')
                .replace(/{{NETWORK_SPEC}}/g, 'Local Network')
                .replace(/{{TEST_DURATION}}/g, `${Math.max(...data.map(d => parseFloat(d.time)))} seconds`)
                .replace(/{{MAX_CLIENTS}}/g, metrics.maxConnections.toString())
                .replace(/{{TOTAL_MESSAGES}}/g, metrics.totalMessages.toLocaleString())
                .replace(/{{PROTOCOL}}/g, 'WebSocket')
                .replace(/{{CONNECTION_PATTERN}}/g, 'Progressive Load')
                .replace(/{{MESSAGE_SIZE}}/g, 'Variable')
                .replace(/{{PERFORMANCE_METRICS}}/g, this.generatePerformanceMetrics(metrics))
                .replace(/{{DETAILED_RESULTS_TABLE}}/g, metrics.detailedTable)
                .replace(/{{LATENCY_DISTRIBUTION}}/g, metrics.latencyDistribution)
                .replace(/{{OBSERVATIONS}}/g, this.generateObservations(metrics, data))
                .replace(/{{NOTES}}/g, `Generated from ${latestFile.name} on ${reportDate}`)
                .replace(/{{PERFORMANCE_SUMMARY}}/g, this.generatePerformanceSummary(metrics));

            // Ensure output directory exists
            if (!fs.existsSync(this.outputDir)) {
                fs.mkdirSync(this.outputDir, {recursive: true});
            }

            // Write the report
            const outputPath = path.join(this.outputDir, `benchmark-report-${reportDate}.md`);
            fs.writeFileSync(outputPath, report);

            console.log(`✅ Report generated successfully: ${outputPath}`);
            console.log('\n📈 Key Metrics:');
            console.log(`   Max Connections: ${metrics.maxConnections}`);
            console.log(`   Throughput: ${metrics.throughput} msg/s`);
            console.log(`   P95 Latency: ${metrics.p95Latency} ms`);
            console.log(`   Success Rate: ${metrics.successRate}%`);

            return outputPath;
        } catch (error) {
            console.error('❌ Error generating report:', error);
            throw error;
        }
    }
}

// Main execution
if (require.main === module) {
    const reporter = new BenchmarkReporter();
    reporter.generateReport();
}

module.exports = BenchmarkReporter;
