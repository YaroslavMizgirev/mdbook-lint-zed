use zed_extension_api::{self as zed, LanguageServerId, Result};

struct MdBookLintExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for MdBookLintExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "mdbook-lint" => {
                let command_path = if let Some(path) = &self.cached_binary_path {
                    path.clone()
                } else {
                    let path = self.language_server_binary_path(language_server_id, worktree)?;
                    self.cached_binary_path = Some(path.clone());
                    path
                };

                Ok(zed::Command {
                    command: command_path,
                    args: vec!["lsp".into(), "--stdio".into()],
                    env: Default::default(),
                })
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        if language_server_id.as_ref() != "mdbook-lint" {
            return Ok(None);
        }

        // Check if this is an mdBook project
        let is_mdbook_project = worktree.read_text_file("book.toml").is_ok()
            || worktree.read_text_file("SUMMARY.md").is_ok();

        // Look for mdbook-lint configuration files
        let config_path = [
            ".mdbook-lint.toml",
            ".mdbook-lint.yaml",
            ".mdbook-lint.json",
        ]
        .iter()
        .find(|&&path| worktree.read_text_file(path).is_ok())
        .map(|&path| format!("{}/{}", worktree.root_path(), path));

        Ok(Some(serde_json::json!({
            "mdBookProject": is_mdbook_project,
            "configPath": config_path
        })))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        if language_server_id.as_ref() != "mdbook-lint" {
            return Ok(None);
        }

        // Look for mdbook-lint configuration files
        let config_path = [
            ".mdbook-lint.toml",
            ".mdbook-lint.yaml",
            ".mdbook-lint.json",
        ]
        .iter()
        .find(|&&path| worktree.read_text_file(path).is_ok())
        .map(|&path| format!("{}/{}", worktree.root_path(), path));

        Ok(Some(serde_json::json!({
            "configPath": config_path
        })))
    }
}

impl MdBookLintExtension {
    fn language_server_binary_path(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if language_server_id.as_ref() != "mdbook-lint" {
            return Err("not mdbook-lint language server".into());
        }

        // First, try to find mdbook-lint in PATH
        if let Some(path) = worktree.which("mdbook-lint") {
            return Ok(path);
        }

        // If not in PATH, try common installation locations
        let common_paths = [
            "~/.cargo/bin/mdbook-lint",
            "/usr/local/bin/mdbook-lint",
            "/opt/homebrew/bin/mdbook-lint",
        ];

        for path in &common_paths {
            let expanded_path = shellexpand::tilde(path);
            if std::path::Path::new(expanded_path.as_ref()).exists() {
                return Ok(expanded_path.into_owned());
            }
        }

        // If mdbook-lint is not found, provide installation instructions
        Err("mdbook-lint not found. Please install it with: cargo install mdbook-lint".into())
    }
}

zed::register_extension!(MdBookLintExtension);
