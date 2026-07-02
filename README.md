# Shader Language Server for Zed

A [Zed](https://zed.dev) extension providing shader validation and language
features for **HLSL**, **GLSL** and **WGSL**, powered by
[`shader-language-server`](https://github.com/antaalt/shader-sense) (`antaalt/shader-sense`).

## Features

- Real-time **diagnostics** (`dxc` for HLSL, `glslang` for GLSL, `naga` for WGSL)
- **Completion**, **hover**, **goto definition** and document/workspace **symbols**
- Document **formatting** (clang-format)
- Preprocessor-aware analysis with custom includes, defines, and per-stage defines

Syntax highlighting is provided by bundled Tree-sitter grammars for each language.

## Supported Languages & File Types

| Language | Extensions |
| -------- | ---------- |
| HLSL | `.hlsl` `.hlsli` `.fx` `.fxh` `.ush` `.usf` |
| GLSL | `.glsl` `.vert` `.frag` `.comp` `.geom` `.tesc` `.tese` `.mesh` `.task` |
| WGSL | `.wgsl` |

## Installation

Search for **Shader Language Server** on the Zed extension registry.

### Language Server Binary

The extension resolves the server binary in this order:

1. An explicit path in `lsp.shader-language-server.binary.path`.
2. A `shader-language-server` executable found on `$PATH`.
3. A previously downloaded copy cached by the extension.
4. A fresh download of the latest [antaalt/shader-sense](https://github.com/antaalt/shader-sense) release.

**Prebuilt binaries are available for** Windows x86\_64, Windows aarch64, and Linux x86\_64.
On other platforms, build the server from source and place it on your `$PATH` or set
`lsp.shader-language-server.binary.path`.

## Configuration

All settings live under `lsp.shader-language-server.settings` in your Zed `settings.json`.
The full settings blob is passed to the server at startup via `--config` and also
forwarded in response to `workspace/configuration` requests.

```jsonc
{
  "lsp": {
    "shader-language-server": {
      "settings": {
        // --- Common ---
        "validate": true,
        "symbols": true,
        "symbolDiagnostics": false,
        "severity": "info",       // "none" | "error" | "warning" | "info" | "hint"
        "includes": [],           // additional include search paths
        "defines": {},            // preprocessor defines: { "MY_MACRO": "1" }
        "stageDefine": {          // per-stage preprocessor defines
          "vertex": {},
          "fragment": {},
          "compute": {}
          // geometry | tesselationControl | tesselationEvaluation | mesh | task
          // rayGeneration | closestHit | anyHit | callable | miss | intersect
        },
        "pathRemapping": {},
        "configOverride": "",

        // --- HLSL ---
        "hlsl": {
          "enabled": true,
          "shaderModel": "ShaderModel6_8",
          "version": "V2021",
          "enable16bitTypes": false,
          "spirv": false
        },

        // --- GLSL ---
        "glsl": {
          "enabled": true,
          "targetClient": "Vulkan1_3",
          "spirvVersion": "SPIRV1_6",
          "preamble": ""
        },

        // --- WGSL ---
        "wgsl": {
          "enabled": true
        }
      }
    }
  }
}
```

> The full list of accepted keys and values is in
> [`package.json`](https://github.com/antaalt/shader-validator/blob/main/package.json)
> of the upstream VS Code extension.

### Disabling individual languages

Setting `hlsl.enabled`, `glsl.enabled`, or `wgsl.enabled` to `false` omits the
corresponding `--hlsl` / `--glsl` / `--wgsl` flag from the server invocation:

```jsonc
"settings": { "wgsl": { "enabled": false } }
```

### Binary overrides

Use the `binary` block to override the executable, arguments, or environment:

```jsonc
"lsp": {
  "shader-language-server": {
    "binary": {
      "path": "/path/to/shader-language-server",
      "arguments": ["--stdio", "--hlsl", "--glsl"],
      "env": { "RUST_LOG": "shader_language_server=debug" }
    }
  }
}
```

When `arguments` is set the extension uses it verbatim, skipping the automatic
`--config` and language flags.

## Credits

- Original extension: [Christian Howe](https://github.com/cjhowe-us)
- Language server: [antaalt](https://github.com/antaalt) —
  [shader-sense](https://github.com/antaalt/shader-sense),
  [shader-validator](https://github.com/antaalt/shader-validator).
- Tree-sitter grammars: `tree-sitter-grammars/tree-sitter-glsl`,
  `tree-sitter-grammars/tree-sitter-hlsl`, `szebniok/tree-sitter-wgsl`.

## License

MIT
