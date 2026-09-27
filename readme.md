# tauri-deepseek-harness

A standalone desktop application for [DeepSeek Harness](https://github.com/deepseek-ai/deepseek-harness), built with [Tauri](https://tauri.app/).

![1](readme_files/1.png)

## Features

- **Native Experience**: Interact with DeepSeek Harness directly from your desktop — no browser needed.
- **Fast and Lightweight**: Built with Tauri, the app has a small footprint and starts up quickly.
- **Cross-Platform**: Available for Windows, macOS, and Linux.

## Prerequisites

- [Node.js](https://nodejs.org/) (v18 or later) with npm
- A DeepSeek API key (see [step 1](#1-get-an-api-key) below)

## Install DeepSeek Harness

DeepSeek Harness (`dsh`) provides the web interface that this desktop app loads. Install it first.

### 1. Get an API Key

Visit the [DeepSeek Platform](https://platform.deepseek.com/api_keys), sign in (or create an account), and generate an API key.

### 2. (Optional) Use a Faster npm Registry

If you are in China, switch npm to the npmmirror registry for faster downloads:

```bash
npm config set registry https://registry.npmmirror.com
```

### 3. Install DeepSeek Harness

Install the CLI globally:

```bash
npm install -g @deepseek-ai/dsh@latest
```

### 4. Start the Web Interface

```bash
dsh web
```

The GUI will be served at `http://127.0.0.1:3080`. Keep this terminal running while using the desktop app.

## Install the Desktop App

### Pre-built Binaries

Download the pre-built binary for your platform from the [Releases](https://github.com/litongjava/tauri-deepseek-harness/releases) page.

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/litongjava/tauri-deepseek-harness
   ```
2. Navigate to the project directory:
   ```bash
   cd tauri-deepseek-harness
   ```
3. Install the Tauri CLI:
   ```bash
   cargo install tauri-cli
   ```
4. Build the application:
   ```bash
   cargo tauri build
   ```

## Usage

1. Start DeepSeek Harness with `dsh web` (see above).
2. Launch the desktop app, paste the complete URL printed by `dsh web` (including `?token=...`) into the connection window, and click Connect Harness.
3. Enter your API key in the interface and start chatting.

### Web Token Authentication

The app now shows an address entry window on every launch, replacing the previous `DSH_WEB_URL` environment variable workflow. Rebuild and reinstall older desktop binaries to use this feature.

Paste the current complete address, such as `http://127.0.0.1:3080/?token=YOUR_CURRENT_TOKEN`. Custom ports and HTTP/HTTPS addresses are supported.

Keep `dsh web` running. If the token changes or you see `dsh web authentication required`, close the Harness window to return to the connection window and paste the new URL. Close the connection window to exit the app.

The input is cleared after opening the Harness window. The launcher does not save addresses or tokens to a configuration file. Authenticating in your browser does not authenticate the desktop WebView.

Use `cargo tauri dev` for development or `cargo tauri build` to create an installer.

## Cache

Cache folder:

- **Windows**: `C:\Users\<username>\AppData\Local\com.litongjava.tauri.deepseek.harness`

## Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT License](LICENSE)
