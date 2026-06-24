use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, long_about = None)]
#[command(name = "badger")]
#[command(about = "Publish and view notifications in your terminal")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Publish a notification
    Publish {
        message: String,

        #[arg(short, long)]
        level: Option<String>,

        #[arg(short, long)]
        verbose: bool,

        #[arg(short, long)]
        data: Option<String>,
    },

    /// Display the next notification in the list
    Next {
        #[arg(short, long)]
        peek: bool,

        #[arg(short, long)]
        format: Option<String>,
    },

    /// Get notification count
    Count {},

    /// Determine by exit code if notifications are pending
    Pending {},
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Publish {
            message,
            level,
            verbose,
            data,
        }) => {
            println!(
                "subcommand:publish message:{} level:{} verbose:{} data:{}",
                message,
                level.unwrap_or("info".to_owned()),
                verbose,
                data.unwrap_or("".to_owned())
            )
        }
        Some(Commands::Next { peek, format }) => {
            println!(
                "subcommand:next peek:{} format:{}",
                peek,
                format.unwrap_or("quiet".to_owned())
            )
        }
        Some(Commands::Count {}) => {
            println!("subcommand:count")
        }
        Some(Commands::Pending {}) => {
            println!("subcommand:pending")
        }
        None => {
            println!("subcommand:none")
        }
    }
}
