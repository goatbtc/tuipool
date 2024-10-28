This project was created for the SatsHack by Vinteum 2024. The team members are Jose Carlos Cirqueira Junior (Team lead & Developer) e Luciana Ferreira (Developer).

![Captura de tela de 2024-10-28 20-26-09](https://github.com/user-attachments/assets/eec604ce-0c2b-4909-b2d1-625cc5ef22cf)

The problem being solved is the lack of visibility into the real-time status of Bitcoin transactions waiting to be confirmed in the mempool. For users, developers, and services that rely on efficient Bitcoin transaction processing, understanding the state of pending transactions—such as their size, fees, and expected confirmation time—can be critical for optimizing their operations. However, accessing and interpreting mempool data in a user-friendly way is not straightforward.

This project aims to solve that by providing a command-line interface 
(CLI) tool built in Rust that monitors the Bitcoin mempool in real-time, offering users detailed insights into the transactions, fee rates, and overall mempool statistics. It allows users to filter transactions by fee or size, view historical data of confirmed transactions, and receive alerts, all in an efficient and easy-to-use terminal interface. This helps users make informed decisions, especially when choosing transaction fees, and enables better transaction management.
Describe the solution / idea you have in mind
The solution is to build a command-line interface 
(CLI) tool in Rust that monitors the Bitcoin mempool in real-time, providing detailed insights into unconfirmed transactions. This tool will allow users to easily track transaction status, prioritize transactions based on fee rates, and receive real-time statistics and alerts.

## Key features

Real-time Monitoring: Continuously track Bitcoin transactions as they enter and leave the mempool. Display essential details like transaction ID, size, fee rate (sat/vByte), and time spent in the mempool.
We're implementing some AI features too.

- [x] Make you happy
- Blocks
  - [X] GET blocks info
- Fees
  - [X] GET 
  - [ ] AI
- Transactions
  - [ ] GET 
  - [ ] AI
- Mining
  - [ ] GET 
- Difficulty
  - [ ] GET 

Filtering and Search: Enable users to filter transactions based on parameters such as fee rate, transaction size, or value, making it easier to find relevant data.

Statistics and Summary: Show aggregated mempool data, including the total number of transactions, average fee rate, and overall mempool size.

Alerts and Notifications: Set custom alerts to notify users when specific conditions are met, such as low fees or confirmation of a specific transaction.

Interactive TUI: Use a terminal user interface (TUI) for a user-friendly experience, displaying data in organized panels with real-time updates.

This tool helps Bitcoin users, developers, and miners optimize transaction handling and gain valuable insights into network congestion and fee dynamics.

## How to use ## 

Install Rust
and then

```
git clone https://github.com/goatbtc/tuipool.git

cargo clean && cargo build --release && cargo run --release

```

## Call to contributors

Passionate about open-source and Bitcoin? Join us in building a secure, community-driven project licensed under MIT. We welcome contributions from developers, testers, and enthusiasts to make this project a valuable resource for the Bitcoin community. Check out our contribution guide [here](CONTRIBUTING.md) to get started!

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE) or <http://opensource.org/licenses/MIT>)
