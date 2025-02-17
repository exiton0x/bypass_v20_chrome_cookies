## decrypt_v20_chrome_cookies
🚀 This repository **is a simple Rust project** for retrieving **Chrome v20 cookies.**

## 📌 Features
- Retrieve cookies from **Chrome v20**
- **Simple and lightweight** Rust implementation
- **Easy** to use

## 🛠️ How It Works
This project uses **Rust** to retrieve **Chrome v20 cookies** using chrome debugging. Here are the main steps:
- **Launches the Chrome Debugger** : Start Chrome in headless mode with remote debugging enabled.
```rust
let _ = Command::new(path_chrome)
            .arg(format!("--remote-debugging-port={}", port_debugging))
            .arg("--remote-allow-origins=*")
            .arg("--headless")
            .arg(format!("--user-data-dir={}", path_user))
            .stdout(Stdio::null()) 
            .stderr(Stdio::null())
            .spawn()?;
```

- **Connect to Debugging Interface** : Use ```reqwest``` to retrieve the ```webSocketDebuggerUrl``` for WebSocket communication.
```rust
let resp = reqwest::blocking::get(format!("http://localhost:{}/json", port_debugging))?;
let ws_url = json[0]["webSocketDebuggerUrl"].as_str().unwrap().to_string();
```

## 📥 Installation
Make sure you have **Rust** installed. If not, you can install it via this website [rust](https://www.rust-lang.org/fr/tools/install):       
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then, clone this repository and build the project:
```
git clone https://github.com/exiton0x/decrypt_v20_chrome_cookies
cd decrypt_v20_chrome_cookies
cargo build --release
```

## 🚀 Usage
```
cargo run
```
Or use the compiled binary:
```
target/release/
```

