use clap::{Parser, ValueEnum};
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, short = 'n', default_value = "1000")]
    pub max_tokens: usize,

    #[arg(long, short, default_value_t = Model::Gpt35)]
    pub model: Model,

    #[arg(long, short, default_value = "0.7")]
    pub temperature: f32,

    #[arg(long, short)]
    pub verbose: bool,

    #[arg(trailing_var_arg = true)]
    pub input: Vec<String>,
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
