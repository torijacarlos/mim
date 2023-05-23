use std::{
    env,
    fmt::{self, Result},
    process::exit,
};

use mim::Feed;

#[derive(Debug)]
enum Command {
    List,
    AddFeed,
    RemoveFeed,
    EditFeed,
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        match &value[..] {
            "list" => Command::List,
            "add-feed" => Command::AddFeed,
            _ => unimplemented!("Invalid command"),
        }
    }
}

enum StdoutColor {
    ForegroundBlack,
    ForegroundRed,
    ForegroundGreen,
    ForegroundYellow,
    ForegroundBlue,
    ForegroundMagenta,
    ForegroundCyan,
    ForegroundWhite,
    ForegroundColor256(u8),
    BackgroundBlack,
    BackgroundRed,
    BackgroundGreen,
    BackgroundYellow,
    BackgroundBlue,
    BackgroundMagenta,
    BackgroundCyan,
    BackgroundWhite,
    BackgroundColor256(u8),
    Reset,
}

impl fmt::Display for StdoutColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result {
        let value = match self {
            Self::ForegroundBlack => "30".to_string(),
            Self::ForegroundRed => "31".to_string(),
            Self::ForegroundGreen => "32".to_string(),
            Self::ForegroundYellow => "33".to_string(),
            Self::ForegroundBlue => "34".to_string(),
            Self::ForegroundMagenta => "35".to_string(),
            Self::ForegroundCyan => "36".to_string(),
            Self::ForegroundWhite => "37".to_string(),
            Self::ForegroundColor256(value) => format!("38;5;{}", value),
            Self::BackgroundBlack => "40".to_string(),
            Self::BackgroundRed => "41".to_string(),
            Self::BackgroundGreen => "42".to_string(),
            Self::BackgroundYellow => "43".to_string(),
            Self::BackgroundBlue => "44".to_string(),
            Self::BackgroundMagenta => "45".to_string(),
            Self::BackgroundCyan => "46".to_string(),
            Self::BackgroundWhite => "47".to_string(),
            Self::BackgroundColor256(value) => format!("48;5;{}", value),
            Self::Reset => "0".to_string(),
        };
        write!(f, "\x1B[{}m", value)
    }
}

async fn list_feed(mim: &mim::Mim) {
    for feed in mim.feeds.iter() {
        println!("{} {} {}", feed.id, feed.source, feed.category);
        for entry in feed.get_entries().await {
            println!("- {}: {:?}", entry.title, entry.published);
        }
    }
}

#[derive(Debug)]
enum ArgOption {
    Source,
    Category,
    Identifier,
}

impl From<String> for ArgOption {
    fn from(value: String) -> Self {
        match &value[..] {
            "source" => Self::Source,
            "category" => Self::Category,
            "identifier" => Self::Identifier,
            _ => unimplemented!("Invalid option"),
        }
    }
}

fn parse_options(args: &mut Vec<String>) -> mim::MimResult<Vec<(ArgOption, String)>> {
    let mut values = vec![];
    args.reverse();
    while !args.is_empty() {
        match args.pop() {
            Some(value) if value.contains("--") => {
                let option = value[2..].to_string();
                if let Some(value) = args.pop() {
                    let option: ArgOption = option.into();
                    values.push((option, value));
                }
            }
            Some(value) => {
                values.push((ArgOption::Identifier, value));
            }
            None => (),
        }
    }
    Ok(values)
}

// todo: remove tokio
// todo: replace reqwest with ureq
// todo: after all of that is done, think about ui or tui or gui or whatever
#[tokio::main]
async fn main() -> mim::MimResult<()> {
    let mut args: Vec<String> = env::args().collect();
    let _: Vec<String> = args.drain(..1).collect();

    if args.is_empty() {
        println!(
            "{}No command provided. {}Options available are: list, add-feed",
            StdoutColor::ForegroundYellow,
            StdoutColor::Reset
        );
        exit(1);
    }

    let mut mim = mim::Mim::load()?;
    let mut command_str: Vec<String> = args.drain(..1).collect();

    let command: Command = command_str.pop().expect("Invalid input for command").into();

    match command {
        Command::List => {
            list_feed(&mim).await;
        }
        Command::AddFeed => {
            if let Ok(args) = parse_options(&mut args) {
                println!("{:?}", args);
                let mut feed = Feed::default();
                for (option, value) in args {
                    match option {
                        ArgOption::Source => feed.source = value.into(),
                        ArgOption::Category => feed.category = value.into(),
                        ArgOption::Identifier => feed.id = value,
                    }
                }
                if feed.id.is_empty() {
                    mim.feeds.push(feed);
                }
            } else {
                println!(
                    "{}Invalid options for adding a feed. {}Options available are: --source, --category",
                    StdoutColor::ForegroundYellow,
                    StdoutColor::Reset
                );
            }
        }
        Command::EditFeed => {
            unimplemented!("Command not implemented");
        }
        Command::RemoveFeed => {
            unimplemented!("Command not implemented");
        }
    }

    mim.save()
}
