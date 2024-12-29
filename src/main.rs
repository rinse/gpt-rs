mod cli;

use std::result::Result;
use anyhow::Context;
use chatgpt::{config::ModelConfiguration, prelude::{ChatGPT, ChatMessage}};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = cli::Args::try_parse()?;
    let is_debug = args.debug;
    log_debug(is_debug, || format!("prompt: <{}>", args.prompt));
    let api_key = std::env::var("CHATGPT_API_KEY")
        .context("An environment variable CHATGPT_API_KEY is required.")?;
    log_debug(is_debug, || "api_key: <***>".to_owned());
    let input = get_input(&args.inputs)
        .map(Ok)
        .unwrap_or_else(get_input_from_stdin)
        .context("Failed to read inputs.")?;
    log_debug(is_debug, || format!("input: <{}>", input));
    let history = vec![
        ChatMessage {
            role: chatgpt::types::Role::System,
            content: args.prompt,
        },
        ChatMessage {
            role: chatgpt::types::Role::User,
            content: input,
        }
    ];
    let client = ChatGPT::new_with_config(
        api_key,
        ModelConfiguration {
            engine: chatgpt::config::ChatGPTEngine::Custom("gpt-4o-mini"), 
            ..ModelConfiguration::default()
        }
    )?;
    let response = client.send_history(&history)
        .await
        .context("Failed to fetch a response from ChatGPT.")?;
    log_debug(is_debug, || format!("model: {}", response.model));
    let chat_content= response.message();
    let content = &chat_content.content;
    println!("{}", content);
    Ok(())
}

fn log_debug(is_debug: bool, message: impl FnOnce() -> String) {
    if is_debug {
        eprintln!("gpt-rs [DEBUG] - {}", message());
    }
}

fn get_input_from_stdin() -> Result<String, std::io::Error> {
    std::io::stdin().lines()
        .reduce(|acc, e| Ok(format!("{}\n{}", acc?, e?)))
        .unwrap_or_else(|| Ok("".to_owned()))
}

fn get_input(inputs: &[String]) -> Option<String> {
    let s = inputs.join("\n");
    if s.is_empty() || s == "-" { None } else { Some(s) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_input() {
        let inputs = ["Hello", "World"].map(|e| e.to_owned()).to_vec();
        let actual = get_input(&inputs);
        assert_eq!(actual, Some("Hello\nWorld".to_owned()));
    }

    #[test]
    fn test_get_input_empty() {
        let actual = get_input(&vec![]);
        assert_eq!(actual, None);
    }

    #[test]
    fn test_get_input_explicit_empty() {
        let actual = get_input(&vec!["-".to_owned()]);
        assert_eq!(actual, None);
    }
}
