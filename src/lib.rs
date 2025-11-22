use std::collections::HashMap;

use zed_extension_api as zed;

const CARGO_PATH_PART: &str = "/.cargo";
const USR_LOCAL_PATH: &str = "/usr/local";
const LIB_PART: &str = "/lib";
const BIN_PART: &str = "/bin";
const LS_PART: &str = "/shader-language-server";
const DYLD_PATH_VAR: &str = "DYLD_LIBRARY_PATH";
const LD_PATH_VAR: &str = "LD_LIBRARY_PATH";
const USER_VAR: &str = "USER";
const HOME_VAR: &str = "HOME";
const CARGO_HOME_VAR: &str = "CARGO_HOME";

struct HlslLsExtension;

impl zed::Extension for HlslLsExtension {
  fn new() -> Self {
    HlslLsExtension
  }

  fn language_server_command(
    &mut self,
    _language_server_id: &zed::LanguageServerId,
    worktree: &zed::Worktree,
  ) -> Result<zed::Command, String> {
    let env: HashMap<String, String> = HashMap::from_iter(worktree.shell_env().into_iter());
    let vulkan_sdk_path = env
      .get("VULKAN_SDK")
      .map(|path| path.to_string())
      .unwrap_or(USR_LOCAL_PATH.to_string());
    let lib_path = vulkan_sdk_path.to_string() + LIB_PART;
    let shader_ls_path = worktree
      .which("shader-language-server")
      .ok_or(())
      .or_else(|_| {
        let cargo_home_path = env.get(CARGO_HOME_VAR).cloned().ok_or(()).or_else(|_| {
          let user = env
            .get(USER_VAR)
            .cloned()
            .ok_or(())
            .map_err(|err| format!("Failed to get USER: {:?}", err))?;
          let home_path = env
            .get(HOME_VAR)
            .cloned()
            .ok_or(())
            .unwrap_or(format!("/Users/{}", user));
          Ok::<String, String>(home_path + "/" + CARGO_PATH_PART)
        })?;
        Ok::<String, String>(String::from(cargo_home_path + BIN_PART + LS_PART))
      })?;

    Ok(zed::Command {
      command: shader_ls_path,
      args: vec![],
      env: vec![
        (DYLD_PATH_VAR.to_string(), lib_path.clone()),
        (LD_PATH_VAR.to_string(), lib_path),
      ],
    })
  }
}

zed::register_extension!(HlslLsExtension);
