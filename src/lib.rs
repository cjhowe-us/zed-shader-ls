use std::fs;

use serde_json::Value;
use zed_extension_api::{self as zed, LanguageServerId, Result};

const SERVER_REPO: &str = "antaalt/shader-sense";
const SERVER_NAME: &str = "shader-language-server";
const LSP_SETTINGS_KEY: &str = "shader-language-server";

struct ShaderLsExtension {
    cached_binary_path: Option<String>,
}

impl ShaderLsExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Ok(settings) = zed::settings::LspSettings::for_worktree(LSP_SETTINGS_KEY, worktree) {
            if let Some(binary) = settings.binary {
                if let Some(path) = binary.path {
                    return Ok(path);
                }
            }
        }

        if let Some(path) = worktree.which(SERVER_NAME) {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        let (os, arch) = zed::current_platform();
        let asset_name = match (os, arch) {
            (zed::Os::Windows, zed::Architecture::X8664) => {
                "shader-language-server-x86_64-pc-windows-msvc.zip"
            }
            (zed::Os::Windows, zed::Architecture::Aarch64) => {
                "shader-language-server-aarch64-pc-windows-msvc.zip"
            }
            (zed::Os::Linux, zed::Architecture::X8664) => {
                "shader-language-server-x86_64-unknown-linux-gnu.zip"
            }
            (os, arch) => {
                return Err(format!(
                    "shader-language-server has no prebuilt binary for {os:?}/{arch:?}. \
                    Build from source and put it in your PATH, or set \
                    `lsp.shader-language-server.binary.path` in your Zed settings."
                ));
            }
        };

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            SERVER_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| {
                format!(
                    "no asset named `{asset_name}` found in release {}",
                    release.version
                )
            })?;

        let version_dir = format!("shader-language-server-{}", release.version);
        let binary_path = match os {
            zed::Os::Windows => format!("{version_dir}/{SERVER_NAME}.exe"),
            _ => format!("{version_dir}/{SERVER_NAME}"),
        };

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|err| format!("failed to download shader-language-server: {err}"))?;

            zed::make_file_executable(&binary_path)?;

            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name = name.to_string_lossy();
                    if name.starts_with("shader-language-server-") && name != version_dir {
                        fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for ShaderLsExtension {
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
        let command = self.language_server_binary_path(language_server_id, worktree)?;

        let lsp_settings =
            zed::settings::LspSettings::for_worktree(LSP_SETTINGS_KEY, worktree).ok();

        // If the user supplies explicit arguments, use them verbatim.
        if let Some(arguments) = lsp_settings
            .as_ref()
            .and_then(|s| s.binary.as_ref())
            .and_then(|b| b.arguments.as_ref())
        {
            let env = lsp_settings
                .as_ref()
                .and_then(|s| s.binary.as_ref())
                .and_then(|b| b.env.as_ref())
                .map(|e| e.clone().into_iter().collect())
                .unwrap_or_default();
            return Ok(zed::Command {
                command,
                args: arguments.clone(),
                env,
            });
        }

        let config: Value = lsp_settings
            .as_ref()
            .and_then(|s| s.settings.clone())
            .unwrap_or_else(|| serde_json::json!({}));

        // Pass the full settings blob as --config so the server has it at startup,
        // before the first workspace/configuration round-trip.
        let config_json = serde_json::to_string(&config).unwrap_or_else(|_| "{}".to_string());
        let mut args = vec!["--stdio".to_string(), "--config".to_string(), config_json];

        // Each language can be disabled via `settings.hlsl.enabled` etc.
        for lang in ["hlsl", "glsl", "wgsl"] {
            let enabled = config
                .get(lang)
                .and_then(|v| v.get("enabled"))
                .and_then(Value::as_bool)
                .unwrap_or(true);
            if enabled {
                args.push(format!("--{lang}"));
            }
        }

        let env = lsp_settings
            .and_then(|s| s.binary)
            .and_then(|b| b.env)
            .map(|e| e.into_iter().collect())
            .unwrap_or_default();

        Ok(zed::Command { command, args, env })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(
            zed::settings::LspSettings::for_worktree(LSP_SETTINGS_KEY, worktree)
                .ok()
                .and_then(|settings| settings.initialization_options),
        )
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(
            zed::settings::LspSettings::for_worktree(LSP_SETTINGS_KEY, worktree)
                .ok()
                .and_then(|settings| settings.settings),
        )
    }
}

zed::register_extension!(ShaderLsExtension);
