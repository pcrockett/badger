use std::{env, fs::File, io::Write, path::PathBuf};

use crate::cli::{NextArgs, PublishArgs};
use anyhow::{Result, bail};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_str};

#[derive(Serialize, Deserialize)]
struct Notification {
    message: String,
    level: String,
    data: Option<Value>,
}

pub fn publish(args: PublishArgs) -> Result<()> {
    let data = match args.data.clone() {
        Some(val) => Some(into_json_value(val)),
        None => None,
    };
    save_notification(Notification {
        message: args.message.clone(),
        level: args.level.clone().unwrap_or("info".to_owned()),
        data: data,
    })?;

    if args.verbose {
        println!(
            "subcommand:publish message:{} level:{} verbose:{} data:{}",
            args.message,
            args.level.unwrap_or("info".to_owned()),
            args.verbose,
            args.data.unwrap_or("".to_owned())
        );
    }

    Ok(())
}

pub fn next(args: NextArgs) -> Result<()> {
    println!(
        "subcommand:next peek:{} format:{}",
        args.peek,
        args.format.unwrap_or("quiet".to_owned())
    );
    Ok(())
}

pub fn count() -> Result<()> {
    println!("subcommand:count");
    Ok(())
}

pub fn pending() -> Result<()> {
    println!("subcommand:pending");
    Ok(())
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

fn save_notification(notification: Notification) -> Result<PathBuf> {
    let state_dir = badger_state_dir();
    let timestamp = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, false);

    for index in 0..=999 {
        let path = state_dir.join(format!("{}.json", slug(&timestamp, index)));
        let Ok(mut file) = File::create_new(&path) else {
            // could be any number of problems.
            //
            // potentially hitting a race condition, someone else beat us to the
            // punch. try the next index.
            continue;
        };
        let serialized = serde_json::to_string_pretty(&notification)?;
        file.write_all(serialized.as_bytes())?;
        file.flush()?;
        return Ok(path);
    }
    bail!("unable to save notification with timestamp {timestamp}")
}

fn slug(timestamp: &String, index: u16) -> String {
    let timestamp = env::var("BADGER_TIMESTAMP").unwrap_or_else(|_| timestamp.clone());
    format!("{timestamp}_{index:03}")
}
