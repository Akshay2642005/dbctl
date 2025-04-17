# 🛠️ db-manager-cli
## ✅ Feature Checklist

### 🏗️ Core Project Setup

- [x] Project Initialization with `cargo new`
- [x] Modular Folder Structure
- [x] Logging with `tracing`
- [x] Error handling with `thiserror`

### ⚙️ CLI Interface (`clap`)

- [x] Create database (PostgreSQL)
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

- [x] Start Postgres container
- [ ] Start Redis container
- [ ] Start MariaDB container

- [ ] View stats/logs of container
- [ ] Return container ID + connection URL
- [ ] Stop and remove containers


### 📄 JSON Config Support

- [ ] Load config from file

- [ ] Save config to file
- [ ] Update config (edit mode)
