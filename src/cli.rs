use std::path::PathBuf;

use async_openai::types::CreateChatCompletionRequestArgs;
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[command(flatten)]
    pub chat: ChatCommandArgs,

    /// Show verbose output
    #[arg(long, short = 'v')]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// default command (can be omitted)
    Chat(ChatCommandArgs),
    Logs,
}

#[derive(Debug, Args)]
pub struct ChatCommandArgs {
    /// ID of the model to use
    #[arg(
        long,
        short = 'm',
        default_value = "gpt-3.5-turbo",
        hide_default_value = true,
        hide_possible_values = true
    )]
    model: Option<Model>,

    /// Sampling temperature
    #[arg(long, short = 't')]
    temperature: Option<f32>,

    /// Probability mass percentage
    #[arg(long, short = 'p')]
    top_p: Option<f32>,

    /// Number of choices to generate for each input message
    #[arg(long, short)]
    n: Option<u8>,

    /// Stream the resultt of the API call
    #[arg(long, short)]
    stream: Option<bool>,

    /// Up to 4 sequences where the API will stop generating further tokens
    #[arg(long)]
    stop: Option<Vec<String>>,

    /// The maximum number of tokens to generate in the chat completion
    #[arg(long, short = 'c')]
    max_tokens: Option<u16>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they
    /// appear in the text so far
    #[arg(long)]
    presence_penalty: Option<f32>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing
    /// frequency in the text so far
    #[arg(long)]
    frequency_penalty: Option<f32>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect
    /// abuse.
    #[arg(long, short = 'u')]
    user: Option<String>,

    /// Input file
    #[arg(long, short = 'i')]
    input: Option<PathBuf>,

    /// Output file
    #[arg(long, short = 'o')]
    output: Option<PathBuf>,

    /// System message
    #[arg(long)]
    pub system: Option<String>,

    #[arg(last(true))]
    pub message: Option<Vec<String>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[value()]
pub enum Model {
    /// alias 3.5
    #[value(name = "gpt-3.5-turbo", alias = "3.5")]
    Gpt35,

    /// alias 4
    #[value(name = "gpt-4", alias = "4")]
    Gpt4,
}

impl ToString for Model {
    fn to_string(&self) -> String {
        match self {
            Model::Gpt35 => "gpt-3.5-turbo".into(),
            Model::Gpt4 => "gpt-4".into(),
        }
    }
}

#[allow(dead_code)]
impl From<&ChatCommandArgs> for CreateChatCompletionRequestArgs {
    fn from(value: &ChatCommandArgs) -> Self {
        let mut builder = CreateChatCompletionRequestArgs::default();
        value.model.map(|m| builder.model(m.to_string()));
        value.temperature.map(|t| builder.temperature(t));
        value.top_p.map(|p| builder.top_p(p));
        value.n.map(|n| builder.n(n));
        value.stream.map(|s| builder.stream(s));
        value.stop.as_ref().map(|s| builder.stop(s));
        value.max_tokens.map(|m| builder.max_tokens(m));
        value.presence_penalty.map(|p| builder.presence_penalty(p));
        value
            .frequency_penalty
            .map(|p| builder.frequency_penalty(p));
        value.user.as_ref().map(|u| builder.user(u));
        builder
    }
}
