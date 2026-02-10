# learning-lab

A personal knowledge base for algorithm practice, design patterns, blockchain, cryptography, and systems-level
experimentation across **Java**, **Rust**, and **C/C++**.

---

## Repository Structure

```text
learning-lab/
├── leetcode-solutions/     # LeetCode problem solutions
│   ├── java/               #   Java/Kotlin (Gradle)
│   ├── rust/               #   Rust (Cargo)
│   └── cpp/                #   C/C++ (CMake)
│
├── design-patterns/        # OOP design pattern implementations
│   └── java/               #   Command, Observer, Prototype, State, Type Object
│
├── blockchain/             # Blockchain & smart contracts
│   ├── rust/               #   Block, chain, hashing (Rust)
│   └── smart-contract/     #   Casper network counter contract
│
├── cryptography/           # Cryptography & Ethereum
│   └── rust/               #   EC curves, modular inverse, ETH wallet
│
├── jni/                    # Java Native Interface demos
│   ├── java/               #   Java side (HelloWorld, JListDemo)
│   └── rust/               #   Rust side (cdylib)
│
└── README.md
```

Each topic folder contains language subfolders with their own build system (Gradle, Cargo, or CMake).

---

## Technologies

| Language | Build System | Used In                                           |
|----------|--------------|---------------------------------------------------|
| Java     | Gradle       | LeetCode solutions, design patterns, JNI          |
| Rust     | Cargo        | LeetCode solutions, blockchain, cryptography, JNI |
| C / C++  | CMake        | LeetCode solutions                                |

---

## Quick Start

```bash
# Java LeetCode solutions
cd leetcode-solutions/java && ./gradlew build

# Rust LeetCode solutions
cd leetcode-solutions/rust && cargo build

# C++ solutions
cd leetcode-solutions/cpp && cmake -B build && cmake --build build

# Cryptography
cd cryptography/rust && cargo build

# Smart contract
cd blockchain/smart-contract && make build-contract
```

---

## License

MIT License — see [LICENSE](./LICENSE) for details.
