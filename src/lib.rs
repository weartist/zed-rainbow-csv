use zed_extension_api::{self as zed, Result};

struct RainbowCSV;

impl zed::Extension for RainbowCSV {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _config: zed::LanguageServerConfig,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Err("Not implemented".into())
    }
}

zed::register_extension!(RainbowCSV);
