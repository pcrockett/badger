use std::{
    env,
    fs::{File, read_dir},
    io::{ErrorKind, Write},
    os::unix::fs::DirEntryExt,
    path::PathBuf,
};

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
    let all_entries = read_dir(badger_state_dir())?;
    for entry in all_entries {
        let path = entry?.path();
        if path.is_file() {
            println!("TODO: read {}", path.to_str().unwrap());
            return Ok(());
        }
    }
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
    let timestamp = env::var("BADGER_TIMESTAMP")
        .unwrap_or_else(|_| Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, false));

    for index in 0..=999 {
        let path = state_dir.join(format!("{}_{:03}.json", timestamp, index));
        let create_result = File::create_new(&path);
        if let Ok(file) = create_result {
            write(notification, file)?;
            return Ok(path);
        }

        let error = create_result.unwrap_err();
        if error.kind() != ErrorKind::AlreadyExists {
            bail!(error);
        }
    }
    bail!("unable to save notification with timestamp `{timestamp}`")
}

fn write(notification: Notification, mut file: File) -> Result<()> {
    let serialized = serde_json::to_string_pretty(&notification)?;
    file.write_all(serialized.as_bytes())?;
    file.flush()?;
    Ok(())
}
