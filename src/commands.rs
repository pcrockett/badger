use crate::cli::{NextArgs, PublishArgs};

pub fn publish(args: PublishArgs) -> i32 {
    println!(
        "subcommand:publish message:{} level:{} verbose:{} data:{}",
        args.message,
        args.level.unwrap_or("info".to_owned()),
        args.verbose,
        args.data.unwrap_or("".to_owned())
    );
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
