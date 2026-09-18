use axum::{
    Extension, Json, Router, http::StatusCode, routing::{get, post},
};
use std::{fs::File};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
//use reqwest::Client;

mod config_manager;
use config_manager::config_manager::{get_cfg, decode_base64, save_cfg};

mod structs;
use structs::structs::{ConfigPayload};


type AppState = Arc<Mutex<Option<Child>>>;


#[tokio::main]
async fn main() {
    let shared_state: AppState = Arc::new(Mutex::new(None));

    let app = Router::new()
        .route("/ping", get(home))
        .route("/start_service", post(start_service))
        .route("/stop_service", post(stop_service))
        .layer(Extension(shared_state))
        .nest("/cfg", Router::new()
            .route("/load_cfg_data", post(load_cfg_data))
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> StatusCode {
    StatusCode::OK
}


async fn start_service(Extension(state): Extension<AppState>) -> StatusCode {
    let mut lock = state.lock().unwrap();

    if lock.is_some() {
        return StatusCode::ALREADY_REPORTED;
    }

    let log_file = match File::create("xray_logs.txt") {
        Ok(file) => file,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let err_file = match log_file.try_clone() {
        Ok(file) => file,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let child = Command::new("bin/xray.exe")
        .arg("run")
        .arg("-c")
        .arg("conf/test.json")
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(err_file))
        .spawn();

    match child {
        Ok(process) => {
            *lock = Some(process);
            return StatusCode::CREATED
        }
        Err(e) => {
            println!("{}", e);
            return StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}


async fn stop_service(Extension(state): Extension<AppState>) -> StatusCode {
    let mut lock = state.lock().unwrap();

    if let Some(mut child) = lock.take() {
        match child.kill() {
            Ok(_) => {
                let _ = child.wait();
                return StatusCode::OK
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR
        }
    } else {
        StatusCode::NOT_FOUND
    }
}


async fn load_cfg_data(Json(payload): Json<ConfigPayload>) -> StatusCode {
    let cfg = decode_base64(&get_cfg(&payload.url).await.unwrap().to_string());

    let cfg = match cfg {
        Some(c) => c,
        _ => {return StatusCode::INTERNAL_SERVER_ERROR;}
    };

    let connect_strings: Vec<&str> = cfg.split('\n').collect();

    save_cfg(&connect_strings, "conf/list.json").unwrap_or_else(|e| {
        println!("Failed to save config: {}", e);
    });

    println!("Config data: {:?}", connect_strings);
    
    StatusCode::OK
}