use anyhow::{bail, Context, Result};
use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};
use atty::Stream;

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use futures::StreamExt;
use ja::cli::{ChatCommandArgs, Cli, Command, Model};
use std::io::Write;
use strum::VariantNames;
use tracing::{info, metadata::LevelFilter};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    init_tracing(&cli);
    info!("cli: {:?}", cli);
    let command = cli.command.unwrap_or(Command::Chat(cli.chat));
    match command {
        Command::Chat(args) => chat(args).await?,
        _ => bail!("command not implemented"),
    }

    Ok(())
}

fn init_tracing(cli: &Cli) {
    let level = if cli.verbose {
        LevelFilter::INFO
    } else {
        LevelFilter::WARN
    };
    tracing_subscriber::fmt().with_max_level(level).init();
}

async fn chat(args: ChatCommandArgs) -> Result<()> {
    match InputMode::from(&args) {
        InputMode::Cli(message) => cli_mode(message, &args).await?,
        InputMode::Pipe => todo!(),
        InputMode::Interactive => interactive_mode(&args).await?,
    }
    Ok(())
}

const CODE_PROMPT: &str = include_str!("./assets/code-prompt.md");
const EXPERTS_PROMPT: &str = include_str!("./assets/experts-interview.md");
const PROMPT_ENGINEER_PROMPT: &str = include_str!("./assets/prompt-engineer.md");
const CODE_REVIEW_PROMPT: &str = include_str!("./assets/code-review.md");

async fn interactive_mode(args: &ChatCommandArgs) -> Result<()> {
    let mut stderr = std::io::stderr();
    let mut messages = vec![];
    let Some(model) = get_model(args.model.unwrap_or_default())? else {
        return Ok(());
    };
    if let Some(system_prompt) = get_system_prompt()? {
        messages.push(
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(system_prompt)
                .build()
                .unwrap(),
        );
    }
    let mut chat_builder: CreateChatCompletionRequestArgs = args.into();
    chat_builder.model(model.to_string());
    loop {
        let user_input = get_user_input()?;
        if user_input == "exit" {
            break;
        }
        messages.push(
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(user_input)
                .build()?,
        );
        let request = chat_builder.messages(messages.clone()).build()?;
        let mut response = Client::new().chat().create_stream(request).await?;
        let mut role = Role::Assistant;
        let mut content = String::new();
        while let Some(response) = response.next().await {
            let response = response?;
            let choice = response.choices.get(0).unwrap();
            if let Some(r) = &choice.delta.role {
                writeln!(stderr, "{}:", role)?;
                role = r.clone();
                stderr.flush()?;
            }
            if let Some(token) = &choice.delta.content {
                write!(stderr, "{}", token)?;
                content.push_str(token);
                stderr.flush()?;
            }
        }
        writeln!(stderr)?;
        stderr.flush()?;
        messages.push(
            ChatCompletionRequestMessageArgs::default()
                .role(role)
                .content(content)
                .build()?,
        );
    }
    Ok(())
}

#[allow(dead_code)]
fn add_system_message(
    messages: &mut Vec<ChatCompletionRequestMessage>,
    system: &String,
) -> Result<()> {
    messages.push(
        ChatCompletionRequestMessageArgs::default()
            .role(Role::System)
            .content(system)
            .build()?,
    );
    Ok(())
}

fn get_model(default: Model) -> anyhow::Result<Option<Model>> {
    match Select::with_theme(&ColorfulTheme::default())
        .items(Model::VARIANTS)
        .default(default as usize)
        .with_prompt("Model (Escape to exit)")
        .interact_opt()?
    {
        None => Ok(None),
        Some(selection) => Ok(Model::from_repr(selection)),
    }
}

fn get_system_prompt() -> anyhow::Result<Option<String>> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .item("Default Prompt (You are a helpful assistant.)")
        .item("Code rules prompt")
        .item("Experts prompt")
        .item("Prompt Engineer")
        .item("Code Review")
        .item("Custom")
        .default(0)
        .with_prompt("System prompt (Escape to exit)")
        .interact_opt()?;
    let system_prompt = match selection {
        None => None,
        Some(0) => None,
        Some(1) => Some(CODE_PROMPT.to_string()),
        Some(2) => Some(EXPERTS_PROMPT.to_string()),
        Some(3) => Some(PROMPT_ENGINEER_PROMPT.to_string()),
        Some(4) => Some(CODE_REVIEW_PROMPT.to_string()),
        Some(5) => {
            let input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Custom Prompt")
                .interact_text()?;
            Some(input)
        }
        Some(_) => unreachable!(),
    };
    Ok(system_prompt)
}

fn get_user_input() -> Result<String, anyhow::Error> {
    let user_input = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("User Prompt (type exit to exit)")
        .default("exit".into())
        .with_post_completion_text("User Prompt")
        .interact_text()?;
    Ok(user_input)
}

async fn cli_mode(message: String, args: &ChatCommandArgs) -> Result<()> {
    let mut stdout = std::io::stdout();
    info!("message: {}", message);
    let mut messages = vec![];
    args.system.as_ref().map(|system| {
        let message = ChatCompletionRequestMessageArgs::default()
            .content(system)
            .role(Role::System)
            .build()
            .context("system")
            .unwrap();
        messages.push(message);
    });
    messages.push(
        ChatCompletionRequestMessageArgs::default()
            .content(message)
            .build()
            .context("message")?,
    );
    let mut chat_builder: CreateChatCompletionRequestArgs = args.into();
    let request = chat_builder.messages(messages).build().context("request")?;
    info!("request: {:?}", request);
    let mut stream = Client::new().chat().create_stream(request).await?;
    while let Some(response) = stream.next().await {
        match response {
            Ok(response) => {
                let choice = response.choices.get(0).unwrap();
                if let Some(role) = &choice.delta.role {
                    writeln!(stdout, "{}:", role)?;
                    stdout.flush()?;
                }
                if let Some(content) = &choice.delta.content {
                    write!(stdout, "{}", content)?;
                    stdout.flush()?;
                }
                // TODO markdown / syntax
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
    writeln!(stdout)?;
    stdout.flush()?;
    Ok(())
}

enum InputMode {
    Cli(String),
    Pipe,
    Interactive,
}

impl InputMode {
    fn from(args: &ChatCommandArgs) -> Self {
        if let Some(ref message) = args.message {
            InputMode::Cli(message.join(" "))
        } else if atty::is(Stream::Stdin) {
            InputMode::Interactive
        } else {
            InputMode::Pipe
        }
    }
}
