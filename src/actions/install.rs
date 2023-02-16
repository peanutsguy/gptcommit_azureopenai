use std::fs::{self};

#[cfg(unix)]
use std::{fs::Permissions, os::unix::prelude::PermissionsExt};

use anyhow::Result;
use colored::Colorize;

use crate::{
    cmd::find_executable,
    git::get_hooks_path,
    help::print_help_openai_api_key,
    settings::{ModelProvider, OpenAISettings, Settings},
};

pub(crate) async fn main(settings: Settings) -> Result<()> {
    println!("{}", "Installing gptcommit hook...".green().bold());

    find_executable("git", "To use gptcommit_azureopenai, you must have git on your PATH")?;
    find_executable("gptcommit_azureopenai", " To use gptcommit_azureopenai, you must have gptcommit_azureopenai on your PATH. Install with `cargo install gptcommit_azureopenai`")?;

    // confirm in git root
    let hooks_path = get_hooks_path()?;

    info!(
        "Found git hooks path for current git repo {}",
        hooks_path.display()
    );
    let prepare_commit_msg_path = hooks_path.join("prepare-commit-msg");
    println!(
        "Installing git hook to {}",
        hooks_path.display().to_string().bold()
    );
    info!("Overwriting file at {}", prepare_commit_msg_path.display());
    fs::write(
        &prepare_commit_msg_path,
        include_str!("../../prepare-commit-msg"),
    )?;
    #[cfg(unix)]
    fs::set_permissions(&prepare_commit_msg_path, Permissions::from_mode(0o755))?;

    println!(
        "{}",
        "gptcommit_azureopenai hook successfully installed!".green().bold(),
    );

    if let Settings {
        model_provider: Some(ModelProvider::OpenAI),
        openai: Some(OpenAISettings { api_key, .. }),
        ..
    } = settings
    {
        if api_key.unwrap_or_else(|| "".into()).is_empty() {
            print_help_openai_api_key();
        }
    }

    Ok(())
}
