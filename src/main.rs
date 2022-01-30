#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;

use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome},
    Request,
};
use rocket_contrib::json::Json;
use serde_derive::Serialize;
use serde_json::Value;
use std::{
    str::FromStr,
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use sysinfo::{DiskExt, ProcessorExt, System, SystemExt};

struct Token {
    token: String,
}

#[derive(Debug)]
enum ApiTokenError {
    Missing,
    Invalid,
}

#[derive(Serialize)]
struct CheckResult {
    name: String,
    label: String,
    status: String,
    notificationMessage: String,
    shortSummary: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ApiTokenError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("oh-dear-health-check-secret");
        match token {
            Some(token) => {
                let wrapped_token = Token {
                    token: token.to_string(),
                };
                if is_valid_token(&wrapped_token) {
                    Outcome::Success(wrapped_token)
                } else {
                    Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid))
                }
            }
            // token does not exist
            None => {
                if token_is_set() {
                    Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing))
                } else {
                    Outcome::Success(Token {
                        token: "".to_string(),
                    })
                }
            }
        }
    }
}

fn is_valid_token(token: &Token) -> bool {
    std::env::var("OHDEAR_TOKEN").map_or(false, |val| val == token.token)
}

fn token_is_set() -> bool {
    std::env::var("OHDEAR_TOKEN").map_or(false, |val| val != "")
}

fn env_with_default<A: FromStr + Copy>(env_name: &str, default: A) -> A {
    std::env::var(env_name).map_or(default, |val| val.parse::<A>().map_or(default, |val| val))
}

#[get("/health", format = "application/json")]
fn health(_token: Token) -> Json<Value> {
    Json(json!({
        "finishedAt": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        "checkResults": [
            disk_check(env_with_default("DISK_FAILURE_THRESHOLD", 90), env_with_default("DISK_WARNING_THRESHOLD", 80)),
            memory_check(env_with_default("MEMORY_FAILURE_THRESHOLD", 80), env_with_default("MEMORY_WARNING_THRESHOLD", 70)),
            cpu_check(env_with_default("CPU_FAILURE_THRESHOLD", 80), env_with_default("CPU_WARNING_THRESHOLD", 70), env_with_default("CPU_TIMESPAN_MS", 500)),
        ],
    }))
}

fn disk_check(failed_threshold: i64, warning_threshold: i64) -> CheckResult {
    let sys = System::new_all();
    let mut total_disk_available: i64 = 0;
    let mut total_disk_space: i64 = 0;
    for disk in sys.disks() {
        total_disk_space += disk.total_space() as i64;
        total_disk_available += disk.available_space() as i64;
    }

    println!("{}/{}B total disk used", total_disk_available, total_disk_space );
    let disk_percentage = 100 - ((total_disk_available as f64 / total_disk_space as f64) * 100.0) as i64;
    CheckResult {
        name: "UsedDiskSpace".to_string(),
        label: "Used Disk Space".to_string(),
        status: if disk_percentage >= failed_threshold {
            "failed".to_string()
        } else if disk_percentage >= warning_threshold {
            "warning".to_string()
        } else {
            "ok".to_string()
        },
        notificationMessage: format!(
            "The disk usage percentage is at ({}% used)",
            disk_percentage
        ),
        shortSummary: format!("{}%", disk_percentage),
    }
}

fn memory_check(failed_threshold: i64, warning_threshold: i64) -> CheckResult {
    let sys = System::new_all();
    println!("{}/{}KB memory", sys.available_memory(), sys.total_memory() );
    let memory_percentage = 100 - ((sys.available_memory() as f64 / sys.total_memory() as f64) * 100.0) as i64;
    CheckResult {
        name: "MemorySpace".to_string(),
        label: "Memory Space".to_string(),
        status: if memory_percentage >= failed_threshold {
            "failed".to_string()
        } else if memory_percentage >= warning_threshold {
            "warning".to_string()
        } else {
            "ok".to_string()
        },
        notificationMessage: format!(
            "The memory usage percentage is at ({}% used)",
            memory_percentage
        ),
        shortSummary: format!("{}%", memory_percentage),
    }
}

fn cpu_check(failed_threshold: i64, warning_threshold: i64, cpu_timespan: i64) -> CheckResult {
    let mut sys = System::new_all();
    let mut total_cpu_load = 0;
    sleep(Duration::from_millis(cpu_timespan as u64));
    sys.refresh_all();
    for processor in sys.processors() {
        total_cpu_load += processor.cpu_usage() as i64;
    }
    
    println!("{} total cpu load over {} cores", total_cpu_load, sys.processors().len() );
    let average_load = total_cpu_load / sys.processors().len() as i64;

    CheckResult {
        name: "Load".to_string(),
        label: "CPU Load".to_string(),
        status: if average_load >= failed_threshold {
            "failed".to_string()
        } else if average_load >= warning_threshold {
            "warning".to_string()
        } else {
            "ok".to_string()
        },
        notificationMessage: format!("The cpu load in the last minute is ({}%)", average_load),
        shortSummary: format!("{}%", average_load),
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![health])
}

fn main() {
    rocket().launch();
}
