use std::{
    env,
    fs::{File, create_dir_all, read_dir},
    io::{ErrorKind, Read, Write},
    path::PathBuf,
    process::exit,
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
    let path = save_notification(Notification {
        message: args.message.clone(),
        level: args.level.clone().unwrap_or("info".to_owned()),
        data: data,
    })?;

    if args.verbose {
        println!("Saved to {}", path.to_str().unwrap());
    }

    Ok(())
}

pub fn next(args: NextArgs) -> Result<()> {
    let mut all_entries: Vec<PathBuf> = read_dir(badger_state_dir()?)?
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap().path())
        .filter(|x| x.is_file())
        .collect();
    all_entries.sort();
    let Some(next_file) = all_entries.get(0) else {
        return Ok(());
    };

    let mut data = String::new();
    {
        let mut file = File::open(next_file)?;
        file.read_to_string(&mut data)?;
    }
    let parsed: Notification = from_str(data.as_str())?;

    let format = args.format.unwrap_or("quiet".to_owned());
    match format.as_str() {
        "quiet" => println!("{}", parsed.message),
        "json" => println!("{}", data),
        _ => bail!(
            "Unrecognized format: `{}`. Expected `quiet` or `json`.",
            format
        ),
    }

    if !args.peek {
        std::fs::remove_file(next_file)?;
    }
    Ok(())
}

pub fn count() -> Result<()> {
    let count = read_dir(badger_state_dir()?)?
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap().path())
        .filter(|x| x.is_file())
        .count();
    println!("{}", count);
    Ok(())
}

pub fn pending() -> Result<()> {
    let result: Vec<()> = read_dir(badger_state_dir()?)?
        .filter(|x| x.is_ok())
        .map(|_| ())
        .take(1)
        .collect();

    if Some(&()) == result.get(0) {
        Ok(())
    } else {
        exit(1);
    }
}

fn badger_state_dir() -> Result<PathBuf> {
    let state_home = env::var("XDG_STATE_HOME").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap();
        format!("{}/.local/state", home)
    });
    let path = PathBuf::from(state_home).join("badger");
    create_dir_all(&path)?;
    Ok(path)
}

fn into_json_value(data: String) -> Value {
    match from_str(data.as_str()) {
        Ok(val) => val,
        Err(_) => Value::String(data),
    }
}

fn save_notification(notification: Notification) -> Result<PathBuf> {
    let state_dir = badger_state_dir()?;
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
