use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_name = "TEXT", help = "System prompt which characterises the program")]
    pub prompt: String,
    #[arg(help = "User prompt which is the input for the program")]
    pub inputs: Vec<String>,

    #[arg(short, long, action, help = "Emits debug messages")]
    pub debug: bool,
}
