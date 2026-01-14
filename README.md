# 💊 Meditrack

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Built With Rust](https://img.shields.io/badge/Built%20With-Rust-orange?logo=rust)](https://www.rust-lang.org/)

> **Simple. Private. Blazingly Fast.**
> A high-performance, terminal-based health tracking tool built in **Rust**.

Meditrack solves the problem of "app bloat." Instead of opening a slow mobile app, logging into an account, and handing over your data, Meditrack lets you manage your health protocols in milliseconds directly from your command line.

---

## ✨ Key Features

* **🚀 Blazingly Fast:** Written in Rust for near-zero startup time and minimal resource usage.
* **🔒 Absolute Privacy:** 100% local storage. Your medication history never leaves your machine.
* **📂 Simple Data Format:** Uses human-readable JSON storage, making backups and custom scripting easy.
* **🎯 Zero Distractions:** No notifications, no ads, and no tracking just the utility you need.

---

## 🚀 Quick Start

### 1. Installation

#### **Binary Download (Recommended)**
Grab the latest statically linked binary for your system from the [Releases](https://github.com/alanirawad-sketch/Meditrack/releases) page.

```bash
# Move the binary to your path (Linux/macOS example)
chmod +x meditrack
sudo mv meditrack /usr/local/bin/

🦀 Building from Source

git clone [https://github.com/alanirawad-sketch/Meditrack.git](https://github.com/alanirawad-sketch/Meditrack.git)
cd Meditrack
cargo build --release

⌨️ Basic Commands

[Action]	              [Command]
Add Medicine	          meditrack add "Aspirin" "500mg" "08:00"
View Schedule	          meditrack list
Mark as Taken             meditrack take <ID>
Help	                  meditrack help


🛠 Usage Examples

- Adding a daily supplement:
  meditrack add "Vitamin D3" "5000IU" "09:00"
- Checking your daily progress:
  meditrack list
- Marking as taken:
  meditrack take 1

🎯 Target Audience

Software Engineers: Stay in the zone. Track your health without switching windows.
Biohackers: Manage complex supplement stacks with precision.
Privacy Purists: 100% offline and under your total control.

💰 Monetization Strategy

Free Tier: 100% access to the core CLI tool and local tracking.
Pro Tier(€5/mo): Encrypted cloud backup and PDF report generation.

📁 Repository Structure

Meditrack/
├── src/                # Source code
├── docs/               # Documentation
├── .github/            # GitHub workflows (CI/CD)
├── .cargo/             # Cargo configuration
├── Cargo.toml          # Rust project config
├── LICENSE             # License file
├── README.md           # Project overview
└── medications.txt     # Sample data

🤝 Contributing

1. Fork the repository.
2. Create your branch (git checkout -b feature/NewFeature).
3. Commit changes (git commit -m 'Add NewFeature').
4. Push to the branch (git push origin feature/NewFeature).
5. Open a Pull Request.

📜 License

Distributed under the MIT License.



Maintained with ❤️ by the Meditrack Project Team.



