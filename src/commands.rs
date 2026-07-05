use std::{
    env,
    fs::{File, create_dir_all, read_dir},
    io::{ErrorKind, Read, Write},
    os::unix::process::ExitStatusExt,
    path::PathBuf,
    process::{Command, Stdio, exit},
    sync::{Arc, atomic},
};

use crate::cli::{NextArgs, PublishArgs, RunArgs};
use crate::signals;
use anyhow::{Result, bail};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Notification {
    message: String,
    level: String,
    data: Option<serde_json::Value>,
}

pub fn publish(args: PublishArgs) -> Result<()> {
    let data = if args.data == Some("-".to_owned()) {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        Some(buf)
    } else {
        args.data
    };

    let data = data.map(into_json_value);
    let path = save_notification(Notification {
        message: args.message,
        level: args.level.unwrap_or("info".to_owned()),
        data,
    })?;

    if args.verbose {
        println!("Saved to {}", path.to_str().expect("unable to unwrap path"));
    }

    Ok(())
}

/// Start a child process to run the given command, forwarding signals to it, and
/// publishing an event if the process exits with a nonzero exit code or terminates
/// by signal.
pub fn run(args: RunArgs) -> Result<()> {
    let child_pid = Arc::new(atomic::AtomicU32::new(0));
    signals::forward_to(child_pid.clone())?;

    let command = args.command;
    let mut child = Command::new(args.shell.unwrap_or("sh".to_owned()))
        .args(["-c", command.as_str()])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // allow background signal thread to start forwarding signals
    child_pid.store(child.id(), atomic::Ordering::Relaxed);
    let result = child.wait()?;
    // disable signal forwarding since the child isn't running anymore
    child_pid.store(0, atomic::Ordering::Relaxed);

    if result.success() {
        return Ok(());
    }

    let message = match result.code() {
        Some(code) => format!("`{command}` exited with code {code}."),
        None => {
            let signal = result.signal().expect("could not unwrap signal");
            format!("`{command}` was terminated with signal {signal}.")
        }
    };
    let metadata = serde_json::json!({
        "command": command,
        "exit_code": result.code(),
        "signal": result.signal(),
    });
    publish(PublishArgs {
        message,
        level: Some("error".to_owned()),
        verbose: false,
        data: Some(metadata.to_string()),
    })?;

    exit(result.code().unwrap_or(1));
}

pub fn next(args: NextArgs) -> Result<()> {
    let mut all_entries: Vec<PathBuf> = read_dir(badger_state_dir()?)?
        .filter(|x| x.is_ok())
        .map(|x| x.expect("unable to unwrap dir entry").path())
        .filter(|x| x.is_file())
        .collect();
    all_entries.sort();
    let Some(next_file) = all_entries.first() else {
        return Ok(());
    };

    let mut data = String::new();
    {
        let mut file = File::open(next_file)?;
        file.read_to_string(&mut data)?;
    }
    let parsed: Notification = serde_json::from_str(data.as_str())?;

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
        .map(|x| x.expect("unable to unwrap dir entry").path())
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

    if Some(&()) == result.first() {
        Ok(())
    } else {
        exit(1);
    }
}

fn badger_state_dir() -> Result<PathBuf> {
    let state_home = env::var("XDG_STATE_HOME").unwrap_or_else(|_| {
        let home = env::var("HOME").expect("no HOME env variable");
        format!("{}/.local/state", home)
    });
    let path = PathBuf::from(state_home).join("badger");
    create_dir_all(&path)?;
    Ok(path)
}

fn into_json_value(data: String) -> serde_json::Value {
    match serde_json::from_str(data.as_str()) {
        Ok(val) => val,
        Err(_) => serde_json::Value::String(data),
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
