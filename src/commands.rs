use std::{env, path::PathBuf};

use crate::cli::{NextArgs, PublishArgs};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, from_str};

#[derive(Serialize, Deserialize)]
struct Notification {
    message: String,
    level: String,
    data: Option<Value>,
}

pub fn publish(args: PublishArgs) -> i32 {
    let data = match args.data.clone() {
        Some(val) => Some(into_json_value(val)),
        None => None,
    };
    save_notification(Notification {
        message: args.message.clone(),
        level: args.level.clone().unwrap_or("info".to_owned()),
        data: data,
    });

    if args.verbose {
        println!(
            "subcommand:publish message:{} level:{} verbose:{} data:{}",
            args.message,
            args.level.unwrap_or("info".to_owned()),
            args.verbose,
            args.data.unwrap_or("".to_owned())
        );
    }
    0
}

pub fn next(args: NextArgs) -> i32 {
    println!(
        "subcommand:next peek:{} format:{}",
        args.peek,
        args.format.unwrap_or("quiet".to_owned())
    );
    0
}

pub fn count() -> i32 {
    println!("subcommand:count");
    0
}

pub fn pending() -> i32 {
    println!("subcommand:pending");
    0
}

fn badger_state_dir() -> PathBuf {
    let state_home = env::var("XDG_STATE_HOME").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap();
        format!("{}/.local/state", home)
    });
    PathBuf::from(state_home).join("badger")
}

fn into_json_value(data: String) -> Value {
    match from_str(data.as_str()) {
        Ok(val) => val,
        Err(_) => Value::String(data),
    }
}

fn save_notification(notification: Notification) -> String {
    let state_dir = badger_state_dir();
    panic!("Not implemented yet")
}
