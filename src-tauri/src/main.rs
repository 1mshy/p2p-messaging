// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api_structs;

use reqwest::{Error};
use serde_json::{Value};
use crate::api_structs::Dustbin;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn message(name: &str) -> String {
    format!("You are about to send the message: {}", name)
}

#[tauri::command]
async fn request_ip() -> String {
    let err_str: String = "Error getting ip".to_string();
    let client = reqwest::Client::new();
    let response = match client.get("http://localhost:5555/ip")
        .send().await {
        Ok(response) => response,
        Err(_) => return err_str
    };
    if response.status().is_success() {
        let person: Dustbin = match response.json().await {
            Ok(ip) => ip,
            Err(_) => return err_str
        }; // Deserialize JSON response
        return person.origin;
    }
    return err_str;
}

#[tauri::command]
async fn register() -> String {
    let err_str: String = "Error getting ip".to_string();
    let client = reqwest::Client::new();
    let response = match client.post("http://localhost:5555/register")
        .json(&serde_json::json!({
            "uuid": "32984320948ejf30jf"
        }))
        .send().await {
        Ok(response) => response,
        Err(_) => return err_str
    };
    if response.status().is_success() {
        let person: Dustbin = match response.json().await {
            Ok(ip) => ip,
            Err(_) => return err_str
        }; // Deserialize JSON response
        return person.origin;
    }
    return err_str;
}

///
///
/// # Arguments
///
/// * `payload`: &Value
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```
///     post(&json!({"one": "things"}));
/// ```
#[allow(dead_code)] // makes the compiler not spit out warnings bc we aren't using it yet
#[tokio::main]
async fn post(payload: &Value) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let response = client.post("https://httpbin.org/post")
        .json(payload)
        .send()
        .await?;
    println!("Status for post request: {}", response.status());

    Ok(())
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, message, request_ip, register])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
