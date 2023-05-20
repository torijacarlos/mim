use std::{
    env,
    fmt::{self, Result},
    process::exit,
};

#[derive(Debug)]
enum Command {
    List,
    AddFeed,
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
    Yellow,
    Reset,
}

impl fmt::Display for StdoutColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result {
        let value = match self {
            Self::Yellow => "33",
            Self::Reset => "0",
        };
        write!(f, "\x1B[{}m", value)
    }
}

async fn list_feed(mim: &mim::Mim) {
    for feed in mim.feeds.iter() {
        println!("{} {} {}", feed.source, feed.category, feed.value);
        for entry in feed.get_entries().await {
            println!("{:?}", entry,);
        }
    }
}

// todo: remove tokio
// todo: replace reqwest with ureq
// todo: create api for managing data
//     todo: after all of that is done, think about ui or tui or gui or whatever
#[tokio::main]
async fn main() -> mim::MimResult<()> {
    let mut args: Vec<String> = env::args().collect();
    let _: Vec<String> = args.drain(..1).collect();

    if args.len() == 0 {
        println!(
            "{}No command provided. {}Options available are: list, add-feed",
            StdoutColor::Yellow,
            StdoutColor::Reset
        );
        exit(1);
    }

    let mim = mim::Mim::load()?;
    let mut command_str: Vec<String> = args.drain(..1).collect();

    let command: Command = command_str.pop().expect("Invalid input for command").into();

    match command {
        Command::List => {
            list_feed(&mim).await;
        }
        Command::AddFeed => todo!("Implement"),
    }

    mim.save()
}
