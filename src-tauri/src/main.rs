// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod httpbin;

use reqwest::{Error, Response};
use serde::Serialize;
use serde_json::{json, Value};
use crate::httpbin::Dustbin;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn message(name: &str) -> String {
    // post(&json!({"one": "things"}));
    format!("You are about to send the message: {}", name)
}

#[tauri::command]
async fn request_ip() -> String {
    let ERR_STR: String = "Error getting ip".to_string();
    let client = reqwest::Client::new();
    let response = match client.post("https://httpbin.org/post")
        .send().await {
        Ok(response) => response,
        Err(e) => return ERR_STR
    };

    if response.status().is_success() {
        let person: Dustbin = match response.json().await {
            Ok(ip) => ip,
            Err(e) => return ERR_STR
        }; // Deserialize JSON response
        return person.origin;
    }
    return ERR_STR;
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
        .invoke_handler(tauri::generate_handler![greet, message, request_ip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
