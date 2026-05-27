use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "portpeek",
    about = "View network information about processes",
    version = "0.1.0"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List {
        #[arg(long)]
        tcp: bool,

        #[arg(long)]
        udp: bool,

        #[arg(short, long)]
        all: bool,
    },
    Search {
        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long, value_parser=validate_port)]
        port: Option<u16>,

        #[arg(long)]
        path: Option<String>,

        #[arg(long, value_parser=validate_pid)]
        pid: Option<u32>,

        #[arg(short, long)]
        local: bool,

        #[arg(short, long)]
        remote: bool,
    },
}
fn validate_port (s: &str) -> Result<u16, String> {
    let port = s.parse::<u16>();
    
    if port == Ok(0) {
        return Err(String::from("Port number must be a nonzero value."))
    } else {
        match port {
            Ok(p) => Ok(p),
            Err(_) => Err(String::from("Port number must be between 0 and 65535.")),
        }
    }
}
fn validate_pid (s: &str) -> Result<u32, String> {
    let pid = s.parse::<u32>();
    match pid {
        Ok(p) => Ok(p),
        Err(_) => Err(String::from("PID must range between 0 and 4294967296.")),
    }
}
pub fn parse() -> Args {
    let args = Args::parse();
    args
}
