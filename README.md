# 🚀 Ralert 🚀

![Project Status](https://img.shields.io/badge/status-in_diapers-yellow?style=for-the-badge)
![Built With](https://img.shields.io/badge/built_with-Rust-orange?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)

An incredibly lightweight and fast health monitor (micro-NOC), written 100% in Rust! 🦀

---

## 👋 Hello! What is Ralert?

Welcome to the very beginning of **Ralert**!

The grand vision for this project is to create a **NOC (Network Operations Center)** that consumes the fewest resources possible (RAM and CPU). It's perfect for running on resource-constrained servers (like a Raspberry Pi or a small VPS) that are already busy serving other applications.

## 🚧 We Are Under Construction! 🚧

Patience! This project has just been born. 👶

Right now, **Ralert** is in its "Hello, World" phase. Literally, the only thing it does is print a greeting to the terminal!

But don't be fooled by its current simplicity. Every great journey begins with a single step, and this is ours.

## 🗺️ The Roadmap (What's Coming!)

The plan is to turn this simple "Hello, World" into a powerful tool. Here are the planned features:

* **[ ] 💓 Health Checks:** Super-lightweight checks (Ping, TCP Ports, HTTP Endpoints).
* **[ ] 📊 Simple Dashboard:** A TUI (in the terminal) or a minimal Web UI to see the status (🟢/🔴).
* **[ ] 🗃️ Lightweight Database:** Using **SQLite** to store events without consuming RAM.
* **[ ] 🔔 Alerts:** Webhook integration (Discord, Slack, Telegram) when something fails.
* **[ ] ⚙️ System Metrics:** Basic monitoring of the host's own CPU, RAM, and Disk.

## 🛠️ Tech Stack (Planned)

* **Language:** [Rust](https://www.rust-lang.org/)
* **Async Runtime:** [Tokio](https://tokio.rs/)
* **Web Server (UI):** [Axum](https://github.com/tokio-rs/axum)
* **Database:** [SQLx](https://github.com/launchbadge/sqlx) with SQLite

## 🏁 How to Get Started (The current "Hello, World")

Want to see the magic in action? It's easy!

1.  Clone this repository:
    ```bash
    git clone https://github.com/pxnditxyr/ralert
    cd ralert
    ```

2.  Run it with Cargo:
    ```bash
    cargo run
    ```

3.  Enjoy the greeting!
    ```
    Hello to Ralert!!! 🚀 by Pxndxs 🐼
    ```

And that's all for now! 🎉

## 🤝 Want to Help?

Soon! Once the basic structure is in place, contributions will be more than welcome. If you have ideas, feel free to open an *Issue*!

## 📝 License

This project is licensed under the MIT License.

## 📜 Author

**Ralert** was created by [**Pxndxs**](https://github.com/pxnditxyr).

[**Oficial Creator Website**](https://pxndxs.com)
