class BenchmarkAPI {
    constructor(baseUrl = '') {
        this.baseUrl = baseUrl;
    }

    async startBenchmark(scenario, config) {
        const response = await fetch(`${this.baseUrl}/api/benchmark/start`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({scenario, config})
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.error || 'Failed to start benchmark');
        }

        return response.json();
    }

    async stopBenchmark(benchmarkId) {
        const response = await fetch(`${this.baseUrl}/api/benchmark/stop/${benchmarkId}`, {
            method: 'POST'
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.error || 'Failed to stop benchmark');
        }

        return response.json();
    }

    async getBenchmarkStatus(benchmarkId) {
        const response = await fetch(`${this.baseUrl}/api/benchmark/status/${benchmarkId}`);

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.error || 'Failed to get benchmark status');
        }

        return response.json();
    }

    async listBenchmarks() {
        const response = await fetch(`${this.baseUrl}/api/benchmark/list`);

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.error || 'Failed to list benchmarks');
        }

        return response.json();
    }

    async getSystemStatus() {
        const response = await fetch(`${this.baseUrl}/api/system/status`);

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.error || 'Failed to get system status');
        }

        return response.json();
    }

    async startLogPolling() {
        const response = await fetch(`${this.baseUrl}/api/logs/start`, {
            method: 'POST'
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.error || 'Failed to start log polling');
        }

        return response.json();
    }

    async stopLogPolling() {
        const response = await fetch(`${this.baseUrl}/api/logs/stop`, {
            method: 'POST'
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.error || 'Failed to stop log polling');
        }

        return response.json();
    }

    async pollLogs(lastLogId = 0, timeout = 30000) {
        const response = await fetch(`${this.baseUrl}/api/logs/poll?lastLogId=${lastLogId}&timeout=${timeout}`);

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.error || 'Failed to poll logs');
        }

        return response.json();
    }

    async getLogs(limit = 100, level = null) {
        let url = `${this.baseUrl}/api/logs?limit=${limit}`;
        if (level) {
            url += `&level=${level}`;
        }

        const response = await fetch(url);

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.error || 'Failed to get logs');
        }

        return response.json();
    }

    async clearLogs() {
        const response = await fetch(`${this.baseUrl}/api/logs`, {
            method: 'DELETE'
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.error || 'Failed to clear logs');
        }

        return response.json();
    }
}

// Global API instance
const api = new BenchmarkAPI();
