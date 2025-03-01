mod cli;

use std::{result::Result, str::FromStr};
use anyhow::Context;
use chatgpt::{config::ModelConfiguration, prelude::{ChatGPT, ChatMessage}};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("RUST_LOG", "info");
    let args = cli::Args::try_parse()?;
    if args.debug {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    log::debug!("prompt: <{}>", args.prompt);
    let api_key = std::env::var("CHATGPT_API_KEY")
        .context("An environment variable CHATGPT_API_KEY is required.")?;
    log::debug!("api_key: <***>");
    let input = get_input(&args.inputs)
        .map(Ok)
        .unwrap_or_else(get_input_from_stdin)
        .context("Failed to read inputs.")?;
    log::debug!("input: <{}>", input);
    let history = vec![
        ChatMessage {
            role: chatgpt::types::Role::System,
            content: args.prompt,
        },
        ChatMessage {
            role: chatgpt::types::Role::User,
            content: input,
        },
    ];
    let client = ChatGPT::new_with_config(
        api_key,
        ModelConfiguration {
            engine: chatgpt::config::ChatGPTEngine::Custom("gpt-4o-mini"), 
            temperature: args.temperature,
            top_p: args.top_p,
            max_tokens: args.max_tokens,
            presence_penalty: args.presence_penalty,
            frequency_penalty: args.frequency_penalty,
            reply_count: args.reply_count,
            api_url: url::Url::from_str(args.api_url.as_str())
                .context("API_URL must be a URL")?,
            timeout: std::time::Duration::from_secs(args.timeout),
        }
    )?;
    let response = client.send_history(&history)
        .await
        .context("Failed to fetch a response from ChatGPT.")?;
    log::debug!("model: {}", response.model);
    let chat_content= response.message();
    let content = &chat_content.content;
    println!("{}", content);
    Ok(())
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
