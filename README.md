# GenesisVault: A Secure and Private Ledger via Federated Byzantine Fault Tolerance

GenesisVault is a research project exploring the intersection of Federated Byzantine Fault Tolerance (FBFT), succinct non-interactive zero-knowledge proofs (zk-SNARKs), and privacy-preserving transaction technologies in a Rust-based environment. The project aims to develop a robust, distributed ledger system capable of resisting Byzantine failures while simultaneously providing strong guarantees for data privacy and on-chain state verification. This is achieved through a novel architecture that leverages FBFT for consensus, enabling operation even with a percentage of malicious or faulty nodes, coupled with zk-SNARKs to prove transaction validity and state transitions without revealing sensitive information.

The core objective of GenesisVault is to provide a foundation for building decentralized applications (dApps) requiring both high reliability and confidentiality. Unlike traditional blockchain systems, GenesisVaults FBFT mechanism allows for faster transaction finality and reduced energy consumption as it doesnt rely on proof-of-work or proof-of-stake consensus. Moreover, the integration of zk-SNARKs allows users to transact and interact with the ledger while preserving their anonymity and the confidentiality of transaction details. This capability is particularly crucial for applications involving sensitive data such as financial transactions, healthcare records, or supply chain management.

By combining these technologies, GenesisVault offers a unique solution for creating secure and private distributed systems. The system achieves resistance to faulty or malicious actors, provides high levels of privacy for users and their data, and enables verifiable computation on private data. This makes it a powerful tool for building a wide range of dApps that require both trust and confidentiality. The research behind GenesisVault also aims to contribute to the broader understanding of how these technologies can be effectively integrated and optimized for real-world applications.

## Key Features

*   **Federated Byzantine Fault Tolerance (FBFT) Consensus:** Implements a modified version of the Practical Byzantine Fault Tolerance (pBFT) algorithm, allowing the network to reach consensus even when a certain percentage of nodes are malicious or faulty. The system is designed for a federated setting, where a limited number of known and trusted nodes participate in consensus.
*   **zk-SNARKs for Privacy-Preserving Transactions:** Integrates the `bellman` crate for generating and verifying zk-SNARKs. Transactions are encoded as circuits, enabling the generation of proofs that attest to the validity of the transaction without revealing the underlying data or logic.
*   **On-Chain State Verification:** zk-SNARKs are used to prove the correctness of state transitions. This allows the network to verify the integrity of the ledger's state without needing to re-execute all previous transactions, significantly improving scalability.
*   **Confidential Asset Transfers:** The system supports the creation and transfer of confidential assets, where the amounts and recipients of transactions are hidden from public view using cryptographic commitments and zero-knowledge proofs.
*   **Rust-based Implementation:** Written entirely in Rust, ensuring memory safety, high performance, and cross-platform compatibility. The use of Rust's strong type system and ownership model contributes to the project's overall security and reliability.
*   **Modular Architecture:** Designed with a modular architecture to facilitate future extensions and improvements. Components such as the consensus mechanism, zk-SNARK integration, and transaction processing pipeline are designed to be loosely coupled, allowing for easier experimentation with different algorithms and techniques.
*   **Optimized Proof Generation and Verification:** Focuses on optimizing the performance of zk-SNARK proof generation and verification, leveraging techniques such as parallelization and GPU acceleration to minimize latency.

## Technology Stack

*   **Rust:** The primary programming language used for the entire project. Rust provides memory safety, performance, and concurrency features crucial for building a reliable and efficient distributed system.
*   **tokio:** An asynchronous runtime for Rust, used for handling network communication and concurrent operations within the FBFT consensus algorithm.
*   **bellman:** A Rust crate providing tools for generating and verifying zk-SNARKs. It's used to implement the zero-knowledge proving system for privacy-preserving transactions and on-chain state verification.
*   **rand:** The Rust random number generator crate, used for generating cryptographic keys and other random data required for the system's security protocols.
*   **serde:** A Rust crate for serialization and deserialization, used for encoding and decoding data structures for network communication and storage.

## Installation

1.  **Install Rust:** Ensure you have the Rust toolchain installed. You can download and install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2.  **Clone the Repository:**
    git clone [https://github.com/jjfhwang/GenesisVault.git](https://github.com/jjfhwang/GenesisVault.git)
    cd GenesisVault

3.  **Build the Project:**
    cargo build --release

    This command will build the project in release mode, optimizing for performance.

4.  **Install Dependencies:** Certain dependencies might require system-level installation, depending on your operating system. For example, `bellman` might require `libsnark`. Refer to `bellman`'s documentation for more information.

## Configuration

GenesisVault uses environment variables for configuration. Create a `.env` file in the root directory of the project. The following variables can be configured:

*   `NODE_ID`: The unique identifier for the node.
*   `PEER_ADDRESSES`: A comma-separated list of addresses for other nodes in the network (e.g., `127.0.0.1:8001,127.0.0.1:8002`).
*   `PRIVATE_KEY`: The node's private key (used for signing messages).
*   `CONSENSUS_THRESHOLD`: The minimum number of votes required for consensus (e.g., 2f+1, where f is the number of faulty nodes).

Example `.env` file:

NODE_ID=1
PEER_ADDRESSES=127.0.0.1:8001,127.0.0.1:8002,127.0.0.1:8003
PRIVATE_KEY=0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789
CONSENSUS_THRESHOLD=2

## Usage

To run the GenesisVault node:

./target/release/genesisvault

This will start the node and begin participating in the FBFT consensus process. The node will listen for incoming transactions and state updates.

For sending transactions, you would typically interact with the node via an API (currently under development). The API will allow you to:

*   Submit transactions: The transaction format includes the type of transaction (e.g., asset transfer), the sender, recipient, amount (if applicable), and a zk-SNARK proof.
*   Query the ledger state: Retrieve the current state of the ledger, including account balances and other relevant data.
*   Monitor the network: Observe the consensus process and the status of transactions.

Example transaction structure (in Rust):

struct Transaction {
    transaction_type: String,
    sender: String,
    recipient: String,
    amount: u64,
    proof: Vec<u8>, // zk-SNARK proof
}

## Contributing

We welcome contributions to GenesisVault! Please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Write clear and concise commit messages.
4.  Submit a pull request with a detailed description of your changes.
5.  Ensure all tests pass before submitting the pull request.
6.  Adhere to the Rust coding style guidelines.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/jjfhwang/GenesisVault/blob/main/LICENSE) file for details.

## Acknowledgements

We would like to acknowledge the contributions of the Rust community and the developers of the `bellman` and `tokio` crates. Their work has been invaluable in building this project.