use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = 6379)]
    pub port: usize,

    #[arg(short, long)]
    pub replicaof: Option<String>,

    #[arg(long)]
    pub dir: Option<String>,

    #[arg(long)]
    pub dbfilename: Option<String>
}
