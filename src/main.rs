use anyhow::{Context, Result};
use async_openai::{
    types::{ChatCompletionRequestMessageArgs, Role},
    Client,
};
use atty::Stream;
use clap::Parser;
use futures::StreamExt;
use ja::{ChatCompletionArgs, Cli, Command};
use std::io::Write;
use tracing::{info, metadata::LevelFilter, warn};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    init_tracing(&cli);
    info!("cli: {:?}", cli);
    let command = cli.command.unwrap_or(Command::Chat(cli.chat));
    match command {
        Command::Chat(args) => chat(args).await?,
        _ => warn!("unimplemented command"),
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

// TODO refactor this
async fn chat(args: ChatCompletionArgs) -> Result<()> {
    match InputMode::from(&args) {
        InputMode::Cli(message) => cli_mode(message, args).await?,
        InputMode::Pipe => todo!(),
        InputMode::Interactive => interactive_mode(&args).await?,
    }
    Ok(())
}

use dialoguer::{theme::ColorfulTheme, Input, Select};

const CODE_OPTIMIZED: &str = "You are an AI programming assistant.
Follow the user's requirements carefully & to the letter.
Your responses must not be accusing, rude, controversial or defensive.
Your responses should be informative and logical.
You should always adhere to technical information.
If the user asks for code or technical questions, you must provide code suggestions and adhere to technical information.
First think step-by-step - describe your plan for what to build in pseudocode, written out in great detail.
Then output the code in a single code block.
Minimize any other prose.
Keep your answers short and impersonal.
Use Markdown formatting in your answers.
Make sure to include the programming language name at the start of the Markdown code blocks.
Avoid wrapping the whole response in triple backticks.
The user works in an IDE called Visual Studio Code which has a concept for editors with open files, integrated unit test support, an output pane that shows the output of running the code as well as an integrated terminal.
The active document is the source code the user is looking at right now.
You can only give one reply for each conversation turn.
You should always generate short suggestions for the next user turns that are relevant to the conversation and not offensive.
";

async fn interactive_mode(args: &ChatCompletionArgs) -> Result<()> {
    let mut stderr = std::io::stderr();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .item("None")
        .item("Code optimized")
        .item("Custom")
        .default(0)
        .with_prompt("System prompt")
        .interact_opt()?;
    let system_prompt = match selection {
        None => None,
        Some(0) => None,
        Some(1) => Some(CODE_OPTIMIZED.to_string()),
        Some(2) => {
            let input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Custom Prompt")
                .with_initial_text("You are a helpful assistant.")
                .interact_text()?;
            Some(input)
        }
        Some(_) => unreachable!(),
    };
    let mut messages = vec![];
    if let Some(system) = &system_prompt {
        let message = ChatCompletionRequestMessageArgs::default()
            .content(system)
            .role(Role::System)
            .build()
            .context("system")?;
        messages.push(message);
    }
    loop {
        let user_input = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("User Prompt (type exit to exit)")
            .default("exit".into())
            .with_post_completion_text("User Prompt")
            .interact_text()?;
        if user_input == "exit" {
            break;
        }
        let message = ChatCompletionRequestMessageArgs::default()
            .content(user_input)
            .role(Role::User)
            .build()?;
        messages.push(message);
        let request = args
            .to_chat()
            .messages(messages.clone())
            .build()
            .context("request")?;
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
        let message = ChatCompletionRequestMessageArgs::default()
            .role(role)
            .content(content)
            .build()
            .unwrap();
        messages.push(message);
    }
    Ok(())
}

async fn cli_mode(message: String, args: ChatCompletionArgs) -> Result<()> {
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
    let request = args
        .to_chat()
        .messages(messages)
        .build()
        .context("request")?;
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
    fn from(args: &ChatCompletionArgs) -> Self {
        if let Some(ref message) = args.message {
            InputMode::Cli(message.join(" "))
        } else if atty::is(Stream::Stdin) {
            InputMode::Interactive
        } else {
            InputMode::Pipe
        }
    }
}
