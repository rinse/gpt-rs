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
    #[arg(long, default_value_t = 0.3, help = "Controls randomness of the output. Lower values mean more stability")]
    pub temperature: f32,
    #[arg(long, default_value_t = 1.0, help = "Controls diversity via nucleus sampling, not recommended to use with temperature")]
    pub top_p: f32,
    #[arg(long, help = "Controls the maximum number of tokens to generate in the completion")]
    pub max_tokens: Option<u32>,
    #[arg(long, default_value_t = 0.0, help = "Determines how much to penalize new tokens passed on their existing presence so far")]
    pub presence_penalty: f32,
    #[arg(long, default_value_t = 0.0, help = "Determines how much to penalize new tokens based on their existing frequency so far")]
    pub frequency_penalty: f32,
    #[arg(long, default_value_t = 1, help = "The maximum amount of replies")]
    pub reply_count: u32,
    #[arg(long, default_value = "https://api.openai.com/v1/chat/completions", help = "URL of the /v1/chat/completions endpoint")]
    pub api_url: String,
    #[arg(long, default_value_t = 30, help = "Timeout for the http requests sent (seconds)")]
    pub timeout: u64,
}
