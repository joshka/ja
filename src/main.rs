use anyhow::{Context, Result};
use async_openai::{
    types::{
        ChatChoice, ChatChoiceDelta, ChatCompletionRequestMessageArgs,
        ChatCompletionResponseMessage, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
        CreateChatCompletionStreamResponse,
    },
    Client,
};
use clap::{Parser, ValueEnum};
use derive_builder::Builder;
use directories::ProjectDirs;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    fs::{create_dir_all, File},
    io::{stderr, stdout, Write},
    path::PathBuf,
};
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, short = 'n', default_value = "1000")]
    max_tokens: usize,
    #[arg(long, short, default_value_t = Model::Gpt35)]
    model: Model,
    #[arg(long, short, default_value = "0.7")]
    temperature: f32,
    #[arg(long, short)]
    verbose: bool,
    #[arg(trailing_var_arg = true)]
    input: Vec<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[value()]
enum Model {
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

#[derive(Clone, Serialize, Debug, Builder, Deserialize, PartialEq)]
struct Interaction {
    request: CreateChatCompletionRequest,
    response: CreateChatCompletionResponse,
}

fn from_chat_stream(value: CreateChatCompletionStreamResponse) -> CreateChatCompletionResponse {
    CreateChatCompletionResponse {
        id: value.id.unwrap(),
        object: value.object,
        created: value.created,
        model: value.model,
        choices: value
            .choices
            .into_iter()
            .map(|c| from_choice_delta(c))
            .collect(),
        usage: value.usage,
    }
}

fn from_choice_delta(value: ChatChoiceDelta) -> ChatChoice {
    let message = ChatCompletionResponseMessage {
        content: String::default(),
        role: value.delta.role.unwrap_or_default(),
    };
    ChatChoice {
        index: value.index,
        message,
        finish_reason: value.finish_reason,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let time_stamp = OffsetDateTime::now_utc().format(&Iso8601::DEFAULT)?;
    let cache_dir = ensure_cache_dir_exists()?;

    let path = cache_dir.join(format!("{time_stamp}.json"));
    if cli.verbose {
        writeln!(stderr(), "writing to {path:?}")?;
    }

    let request = create_request(cli.model, cli.temperature, cli.input.join(" "))?;
    let mut stream = Client::new().chat().create_stream(request.clone()).await?;
    let mut response = None;
    let mut lock = stdout().lock();
    while let Some(result) = stream.next().await {
        match result {
            Ok(stream_response) => {
                let response =
                    response.get_or_insert_with(|| from_chat_stream(stream_response.clone()));
                let stream_choice = stream_response.choices.get(0).unwrap();
                let mut response_choice = response.choices.get_mut(0).unwrap();
                if let Some(ref content) = stream_choice.delta.content {
                    write!(lock, "{}", content)?;
                    response_choice.message.content.push_str(content);
                }
                if let Some(ref role) = stream_choice.delta.role {
                    write!(lock, "{}: ", role)?;
                    response_choice.message.role = role.clone();
                }
            }
            Err(err) => {
                writeln!(lock, "error: {err}")?;
            }
        }
        stdout().flush()?;
    }

    let interaction = Interaction {
        request,
        response: response.unwrap(),
    };
    let mut file = File::create(path).context("create response file")?;
    serde_json::to_writer_pretty(&file, &interaction).context("writing file")?;
    file.flush().context("flush")?;

    writeln!(lock)?;
    stdout().flush()?;
    Ok(())
}

fn ensure_cache_dir_exists() -> Result<PathBuf> {
    let projects = ProjectDirs::from("net", "joshka", "ja").context("No valid home directory")?;
    let cache_dir = projects.cache_dir();
    create_dir_all(&cache_dir).context("creating cache dir")?;
    Ok(cache_dir.to_path_buf())
}

fn create_request(
    model: Model,
    temperature: f32,
    content: String,
) -> Result<CreateChatCompletionRequest> {
    let message = ChatCompletionRequestMessageArgs::default()
        .content(content)
        .build()
        .context("message")?;
    CreateChatCompletionRequestArgs::default()
        .model(model.to_string())
        .messages(vec![message])
        .temperature(temperature)
        .build()
        .context("request")
}
