```
# 🛠️  dbctl

A flexible, user-friendly, and type-safe CLI + TUI tool written in Rust to **create, run, and manage Dockerized databases** (PostgreSQL, Redis, MariaDB, and more).

Built with love for learners and power users alike. 💖

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-stable-orange.svg)
![Docker](https://img.shields.io/badge/docker-required-blue.svg)

## 🎯 Overview


**dbctl** simplifies database development and testing by providing:

- **Fast creation** of Docker containers for popular databases
- **Easy management** via both CLI and interactive TUI modes

- **Type-safe** input collection and validation
- **Powerful insights** with stats, logs, and connection details
- **Configuration persistence** with JSON-based profiles

### Supported Databases


- 🐘 **PostgreSQL** - Full-featured, robust relational database
- 🔄 **Redis** - In-memory data structure store
- 🐬 **MariaDB** - Community-developed fork of MySQL
- ➕ Extensible architecture for adding more database types


---


## 🚀 Quick Start


```bash
# Install from crates.io
cargo install dbctl

# Create a PostgreSQL database
dbctl create postgres --name mypg --user admin --password secret --port 5432

# Launch the interactive TUI wizard

dbctl wizard
```


---

## 📅 Development Roadmap

| Week | Focus                             | Goals |
|------|-----------------------------------|-------|
| 1    | ✅ Setup + CLI/TUI Basics         | Rust, project layout, `clap`, `ratatui` |
| 2    | 🛢️ PostgreSQL support             | Database trait, default configs |
| 3    | 🐳 Docker integration              | Launch & manage containers |
| 4    | ⚙️ CLI Command: Create            | Launch Postgres via CLI |
| 5    | 🎨 TUI Wizard                     | Select DB & enter info visually |

| 6    | 🧩 Add Redis, MariaDB             | Implement additional backends |

| 7    | 📊 Stats & Logging                | Show container stats, logs, inspect |
| 8    | 🧪 Error Handling & Polish        | `thiserror`, tracing, UX |

---

## ✅ Feature Checklist

### 🏗️ Core Project Setup

- [ ] Project Initialization with `cargo new`
- [ ] Modular Folder Structure
- [ ] Logging with `tracing`
- [ ] Error handling with `thiserror`

### ⚙️ CLI Interface (`clap`)

- [ ] Create database (PostgreSQL)
- [ ] Create database (Redis)
- [ ] Create database (MariaDB)
- [ ] Inspect running containers
- [ ] Show container logs
- [ ] Delete database containers

### 🎨 TUI Wizard (`ratatui`)

- [ ] Select database type
- [ ] Input config values with validation
- [ ] Preview config before launching
- [ ] Display creation results and info
- [ ] Add theme/colors and improved UX

### 🐳 Docker API (`bollard`)

- [ ] Start Postgres container
- [ ] Start Redis container
- [ ] Start MariaDB container

- [ ] View stats/logs of container
- [ ] Return container ID + connection URL
- [ ] Stop and remove containers


### 📄 JSON Config Support

- [ ] Load config from file

- [ ] Save config to file
- [ ] Update config (edit mode)

---

## 📂 Project Architecture


```
dbctl/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs             # Entry point
│   ├── cli/                # CLI command handling
│   │   └── mod.rs
│   ├── tui/                # Terminal UI components
│   │   └── mod.rs
│   ├── db/                 # Database implementations
│   │   ├── mod.rs
│   │   ├── postgres.rs
│   │   ├── redis.rs
│   │   └── mariadb.rs
│   ├── docker/             # Docker interaction

│   │   └── engine.rs

│   ├── config/             # Configuration handling
│   │   └── models.rs
│   ├── output.rs           # Output formatting
│   ├── utils.rs            # Utility functions
│   └── error.rs            # Error types

```


---

## 💻 Usage Examples


### Command-Line Interface

```bash

# Create a PostgreSQL database
$ dbctl create postgres \
  --name mypg \
  --user admin \

  --password secret \

  --port 5432


✅ Database 'mypg' started in Docker
🔗 URL: postgres://admin:secret@localhost:5432/mypg
🆔 Container ID: 17afc8c9d16


# List running database containers
$ dbctl list

# View container logs
$ dbctl logs mypg


# Stop and remove a container
$ dbctl remove mypg
```

### Terminal User Interface

The TUI provides an interactive wizard for configuring and launching databases:

```
+------------------------------------------+
|    Select a Database to Launch           |
|  > PostgreSQL                            |

|    Redis                                 |
|    MariaDB                               |
+------------------------------------------+

+------------------------------------------+
|  🧾 PostgreSQL Setup                     |

|  Name      : mypg                        |
|  Username  : admin                       |
|  Password  : ******                      |

|  Host      : localhost                   |
|  Port      : 5432                        |
|  SSL?      : [ No ]                      |
+------------------------------------------+
|        [ Create Database ]               |

+------------------------------------------+

✅ Success! Container Started

🔗 URL: postgres://admin:secret@localhost:5432/mypg
🆔 Container ID: 17afc8c9d16

```

---


## 🔧 Database Configuration


Configurations can be saved as JSON profiles for reuse:


```json
{
  "type": "postgres",

  "name": "mypg",

  "user": "admin",
  "password": "secret",
  "host": "localhost",

  "port": 5432,
  "db_name": "mypg",
  "ssl": false
}
```


Load saved configurations:

```bash

$ dbctl create --from-file postgres-dev.json

```


---

## 🛠️ Installation

### Prerequisites


- Rust toolchain (stable)

- Docker installed and running

### From Source

```bash

# Clone the repository
git clone https://github.com/yourusername/dbctl.git
cd dbctl


# Build and install
cargo install --path .
```

---

## 📚 Contributing


Contributions are welcome! Please feel free to submit a Pull Request.


1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---


## 📄 License


This project is licensed under the MIT License - see the LICENSE file for details.
```
