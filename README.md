# OSIS-web

Source code for website osis-episjh.org (planned domain)

## Run

Go to releases.
download zip

go to targets.
run backend.exe (allow firewall)

## Build from source

### Dependencies

- Rust
- Cargo
- Trunk
- Cargo watch(optional, for hot reloading)
- Git

### 1. Install repo

```bat
git clone https://github.com//OSIS-episjh/OSIS-web
cd OSIS-web
```

### 2. Build frontend

```bat
cd frontend
trunk build // use trunk watch for hot realoding
cd ..
```

### 3. Build and run backend

```bat
echo DATABASE_URL=postgres://user:password@localhost:port/osis_web > .env 
cd backend
cargo run // use cargo watch for hot reloading
```

Go to <https://localhost>
