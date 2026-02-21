## 🔁 SCENARIO GROUP 1: **Broadcast to All Users**

These test how your server performs when **sending a message to all connected clients.**

---

### 1.1 – **Mass Broadcast (High Volume)**

* **Setup:** 1,000 clients connected.
* **Action:** Sends 1,000,000 broadcast messages.
    * 1KB/message
    * 100KB messages
* **Goal:** Observe how long each client consume 1,000,000 messages.

**Metrics:**

* Broadcast delivery latency
* Message loss rate
* CPU and memory usage

### 1.2 – **Incremental Broadcast Load**

| Total Messages (1) | Number of Clients (2) | Messages per Client (3)=(1)/(2) |
|--------------------|-----------------------|---------------------------------|
| 10M                | 10                    | 1,000,000                       | 
| 10M                | 100                   | 100,000                         | 
| 10M                | 1,000                 | 10,000                          |
| 10M                | 10,000                | 1,000                           |
| 1M                 | 10                    | 100,000                         |
| 1M                 | 100                   | 10,000                          |
| 1M                 | 1,000                 | 1,000                           |
| 1M                 | 4,000                 | 300                             |

---

## 👤 SCENARIO GROUP 2: **Route Message to Specific Users**

These scenarios test direct messaging, user mapping, and routing logic under load.

---

### 2.1 – **One-to-One Message Routing (High Frequency)**

* **Setup:** 1,000 clients, each with a unique ID.
* **Action:** Server sends 1,000,000 direct messages to each client.
* **Goal:** Test lookup performance and routing precision.

**Metrics:**

* Message delivery latency per user
* Error rate (e.g., incorrect user, dropped message)

---

### 2.2 – **Concurrent User-to-User Messaging**

* **Setup:** 2,000 clients simulate chat between user pairs.
* **Action:** Each client sends a message every second to a specific user.
* **Goal:** Emulate chat server behavior.

---

### 2.3 – **Hot User Scenario**

* **Setup:** 1 "celebrity" user, 5,000 others.
* **Action:** All 5,000 users send direct messages to the celebrity user within 10 seconds.
* **Goal:** Test queue handling for high-targeted routing.

---

### 2.4 – **Disconnected User Handling**

* **Setup:** 1,000 online users, 1,000 disconnected.
* **Action:** Attempt direct messages to both connected and disconnected users.
* **Goal:** Measure fallback behavior, error logging, and queueing for later delivery (if applicable).

---

### 2.5 – **Authentication-Scoped Routing**

* **Setup:** 2,000 clients with JWT-based identity.
* **Action:** Only authenticated clients can receive/send direct messages.
* **Goal:** Ensure correct auth checks under load and secure routing enforcement.

---

## 📊 BONUS: Mixed Load Test

### 3.1 – **Hybrid Scenario (Real-World Simulation)**

* **Setup:**

    * 7,000 clients.
    * 60% receive broadcast.
    * 30% send direct messages.
    * 10% idle but connected.
* **Action:** Mixed broadcast + direct messages every second.
* **Goal:** Observe real-world usage pattern and system bottlenecks.

---

## 🔧 Tools to Simulate Load

Use tools like:

* **Locust** (custom with WebSocket client)
* **Artillery.io** (supports WS)
* **k6** with WebSocket module
* Custom Rust/Go/Node.js test clients with Tokio/tungstenite/etc.

---

If you need code samples or a framework-specific test runner (e.g., for Rust using `tokio-tungstenite`), let me know — I
can provide a tailored setup.
