mod cmp;
mod conditions;
mod echo;
mod eval;
mod executor;
mod lang_parser;
mod regex;
mod utils;
mod variables;

use executor::{ExeError, Executor};
use lang_parser::{LangParser, ParseErr};
use std::{env, fs, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
enum TopLevelErr {
    #[error("Missing file path: `{0}`")]
    MissingFilePath(String),
    #[error("`{0}` not found")]
    FileNotFound(String),
    #[error("IO Error: `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseErr(#[from] ParseErr),
    #[error("Executor error: {0}")]
    ExeError(#[from] ExeError),
}

fn main() -> Result<(), TopLevelErr> {
    let file_path = env::args().nth(1).ok_or(TopLevelErr::MissingFilePath(
        "example: mybash ./src/main.mb".into(),
    ))?;

    let path = Path::new(&file_path);

    if !path.exists() {
        return Err(TopLevelErr::FileNotFound(file_path));
    }

    let content = fs::read_to_string(path)?;
    let result = content.parse::<LangParser>()?;
    let mut exe = Executor::new(&result.experssions);
    exe.execute()?;

    Ok(())
}
