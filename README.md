This project was created for the SatsHack by Vinteum 2024. The team members are Jose Carlos Cirqueira Junior (Team lead & Developer) e Luciana Ferreira (Developer).

The problem being solved is the lack of visibility into the real-time status of Bitcoin transactions waiting to be confirmed in the mempool. For users, developers, and services that rely on efficient Bitcoin transaction processing, understanding the state of pending transactions—such as their size, fees, and expected confirmation time—can be critical for optimizing their operations. However, accessing and interpreting mempool data in a user-friendly way is not straightforward.

This project aims to solve that by providing a command-line interface 
(CLI) tool built in Rust that monitors the Bitcoin mempool in real-time, offering users detailed insights into the transactions, fee rates, and overall mempool statistics. It allows users to filter transactions by fee or size, view historical data of confirmed transactions, and receive alerts, all in an efficient and easy-to-use terminal interface. This helps users make informed decisions, especially when choosing transaction fees, and enables better transaction management.
Describe the solution / idea you have in mind
The solution is to build a command-line interface 
(CLI) tool in Rust that monitors the Bitcoin mempool in real-time, providing detailed insights into unconfirmed transactions. This tool will allow users to easily track transaction status, prioritize transactions based on fee rates, and receive real-time statistics and alerts.

Key Features:
Real-time Monitoring: Continuously track Bitcoin transactions as they enter and leave the mempool. Display essential details like transaction ID, size, fee rate (sat/vByte), and time spent in the mempool.

Filtering and Search: Enable users to filter transactions based on parameters such as fee rate, transaction size, or value, making it easier to find relevant data.

Statistics and Summary: Show aggregated mempool data, including the total number of transactions, average fee rate, and overall mempool size.

Alerts and Notifications: Set custom alerts to notify users when specific conditions are met, such as low fees or confirmation of a specific transaction.

Interactive TUI: Use a terminal user interface (TUI) for a user-friendly experience, displaying data in organized panels with real-time updates.

This tool helps Bitcoin users, developers, and miners optimize transaction handling and gain valuable insights into network congestion and fee dynamics.
