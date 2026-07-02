# Shader Language Server for Zed
> A Zed extension providing the Shader Language Server

## Installation

Install the Shader Language Server extension through Zed. Then, set up Shader
Language Server if you don't already have it.

**(Recommended) Using Cargo Binstall**:

Fast; this downloads a Shader Language Server release binary.

First, install cargo-binstall if you don't have it:

```
### For Linux/MacOS
curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

### For Windows (PowerShell)
Set-ExecutionPolicy Unrestricted -Scope Process;
iex (iwr "https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.ps1").Content;
```

Then, install the Shader Language Server:

```
cargo binstall shader_language_server
```

**Using Cargo Binstall**:

Slow; this compiles Shader Language Server from scratch.

```
cargo install shader_language_server
```

**Using Mise**:

Fast; useful if you already use Mise-en-place.

```
mise use -g cargo:shader-language-server
```
