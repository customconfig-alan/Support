# Support – CustomConfig macOS App

Support is a signed and notarised macOS desktop application developed by [CustomConfig](https://www.customconfig.co) to provide clients with seamless access to chat-based support, session scheduling, and urgent issue escalation. Built using [Tauri](https://tauri.app), the app compiles as a universal binary for both Intel and Apple Silicon Macs. It is designed for distribution via MDM or direct download and ensures trusted execution through macOS Gatekeeper.

## Features

- Full-screen embedded live chat powered by [Plain](https://plain.com)
- Personalised support with automatic name and email injection
- Secure authentication using Mac serial number and email address
- Options to schedule a support session or report urgent issues
- Universal binary (arm64 and x86_64)
- Fully signed and notarised for Gatekeeper compatibility
- Minimalist dark UI with CustomConfig branding
- Fast launch and low resource usage thanks to Tauri

## Requirements

Before starting development, ensure the following tools are installed:

- [Node.js (v18 or later)](https://nodejs.org/)
- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Tauri CLI](https://tauri.app/) – install via:

  ```bash
  npm install -g @tauri-apps/cli
  ```

- Xcode Command Line Tools – install via:

  ```bash
  xcode-select --install
  ```

## Installation

Clone the repository and install dependencies:

```bash
git clone https://github.com/your-username/support.git
cd support
npm install
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

## Development

To run the application locally with hot reload:

```bash
npm run tauri dev
```

This will launch the app in development mode, using local builds and displaying the live chat window on successful authentication.

## Building for Production

To create a universal signed and notarised `.app` for macOS:

```bash
npm run tauri build -- --target universal-apple-darwin
```

After a successful build, the following files will be available in:

```
src-tauri/target/universal-apple-darwin/release/bundle/macos/
```

- `Support.app` — the signed app bundle  
- `Support.zip` — a zipped version of the app ready for distribution

## Continuous Integration

The repository includes a GitHub Actions workflow that handles:

- Installing dependencies  
- Building a universal binary  
- Importing and applying the Developer ID Application signing certificate  
- Notarising the zipped app with Apple  
- Uploading the signed `.zip` to the latest GitHub Release

The workflow is located at:

```
.github/workflows/build.yml
```

It runs automatically on every push to the `main` branch.

## GitHub Secrets Required for Signing

To enable signing and notarisation in CI, the following secrets must be set in the repository:

- `APPLE_ID`: Your Apple Developer email address  
- `APPLE_APP_PASSWORD`: An app-specific password for notarisation  
- `APPLE_TEAM_ID`: Your 10-character Apple Developer Team ID  
- `SIGNING_CERTIFICATE`: The contents of your `.p12` certificate, base64 encoded  
- `SIGNING_CERT_PASSWORD`: The password used to export the `.p12` certificate  

## Download

The most recent signed and notarised release is available at:

```
https://github.com/your-username/support/releases/latest/download/Support.zip
```

This file is ready for direct installation or MDM deployment.

## Repository Structure

```
support/
├── .github/
│   └── workflows/
│       └── build.yml
├── src-tauri/
│   ├── icons/
│   ├── tauri.conf.json
│   └── Cargo.toml
├── index.html
├── package.json
├── README.md
```

## License

This project is maintained by CustomConfig Ltd and is not open source. Distribution is limited to verified clients of CustomConfig via direct or managed channels.

## Contact

For more information or to request access, please visit:

[https://www.customconfig.co](https://www.customconfig.co)
