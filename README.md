# Royal Kludge Configurator

[![Built with Tauri](https://img.shields.io/badge/Built%20with-Tauri-FFC131?logo=tauri&labelColor=1C1C1E)](https://tauri.app)
[![Nuxt UI](https://img.shields.io/badge/Made%20with-Nuxt%20UI-00DC82?logo=nuxt&labelColor=020420)](https://ui.nuxt.com)

A modern, cross-platform desktop application for configuring Royal Kludge keyboards. Built with Tauri, Nuxt, and Rust.

![Royal Kludge Configurator](screenshots/main.png)

## Features

### 🎨 Lighting Configuration

- **All Built-in Light Modes**: Support for all RGB and single-color lighting modes including:
  - Custom, Steady, Breathing, Neon
  - Neon Stream, Ripples Shining, Rotating Windmill
  - Sine Wave, Rainbow Roulette, Stars Twinkle
  - Layer Upon Layer, Rich And Honored, Marquee Effect
  - Rotating Storm, Serpentine Horse Race, Retro Snake
  - Diagonal Transformation, Ambilight, Streamer
  - Flash Away, Shadow Disappear, and more
- **Per-Key RGB Customization**: Click on individual keys to assign custom colors (RGB keyboards only)
- **Brightness Control**: Adjustable brightness levels (0-5)
- **Animation Speed**: Control animation speed (1-5)
- **Sleep Timer**: Configure auto-sleep timing (1-5)
- **Color Picker**: Full RGB color selection with hex, RGB values
- **Random Colors**: Toggle random color generation for compatible modes

### ⌨️ Key Mapping

- **Visual Key Mapping Editor**: Interactive keyboard visualization
- **Remap Any Key**: Click on keys to remap them to different functions
- **Searchable Key Selection**: Easy-to-use searchable dropdown for key codes
- **Real-time Preview**: See mapped keys highlighted on the keyboard image

### 💾 Profile Management

- **Multiple Profiles**: Create, save, and manage multiple configuration profiles
- **Profile Persistence**: All profiles saved locally with SQLite database
- **Auto-Load**: Automatically loads the last selected profile on startup
- **Default Profile**: Automatic default profile creation for new keyboards
- **Profile Switching**: Quick switching between profiles with visual indicators

### 🔧 Technical Features

- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Auto-Updates**: Built-in updater for seamless updates
- **Local Storage**: All configurations stored locally using SQLite
- **Keyboard Detection**: Automatic detection of connected Royal Kludge keyboards
- **Visual Feedback**: Toast notifications for all actions
- **Modern UI**: Clean, responsive interface built with Nuxt UI

## Screenshots

### Main Screen

![Main Screen](screenshots/main.png)
_Keyboard selection and overview_

### Lighting Configuration

![Lighting Configuration](screenshots/lighting.png)
_Configure lighting modes, colors, and effects_

### Per-Key RGB Customization

![Per-Key RGB](screenshots/per-key-rgb.png)
_Customize individual key colors on RGB keyboards_

### Key Mapping Editor

![Key Mapping](screenshots/key-mapping.png)
_Visually remap keys with interactive keyboard layout_

### Profile Management

![Profiles](screenshots/profiles.png)
_Create and manage multiple configuration profiles_

## Installation

### Download

Download the latest release from the [Releases page](https://github.com/Ripwords/rk-configurator/releases).

### Supported Platforms

- **Windows**: `.exe` installer
- **macOS**: `.app` bundle (Intel and Apple Silicon)
- **Linux**: `.AppImage`

## Usage

1. **Connect Your Keyboard**: Plug in your Royal Kludge keyboard
2. **Launch the App**: Open Royal Kludge Configurator
3. **Select Keyboard**: Choose your keyboard from the list
4. **Configure Settings**:
   - Adjust lighting modes and colors
   - Remap keys as needed
   - Create profiles for different configurations
5. **Save**: Click "Save to Keyboard" to apply changes

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable)
- [Node.js](https://nodejs.org/) (v18 or later)
- [Bun](https://bun.sh/) (package manager)
- Platform-specific dependencies:
  - **Linux**: `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Microsoft Visual C++ Build Tools

### Setup

```bash
# Clone the repository
git clone https://github.com/Ripwords/rk-configurator.git
cd rk-configurator

# Install dependencies
bun install

# Run in development mode
bun run dev

# Build for production
bun run build
```

### Project Structure

```
rk837/
├── app/                    # Nuxt frontend
│   ├── components/        # Vue components
│   ├── composables/       # Composables (keyboard, database)
│   └── pages/             # Application pages
├── src-tauri/             # Tauri backend (Rust)
│   ├── src/               # Rust source code
│   ├── keyboards/         # Keyboard configuration files
│   └── migrations/        # Database migrations
└── scripts/                # Build and utility scripts
```

## Credits

This project is inspired by and builds upon the excellent work of **[Rangoli](https://github.com/rnayabed/rangoli)** by [Debayan Sutradhar (rnayabed)](https://github.com/rnayabed).

Rangoli is a free, open-source, lightweight, cross-platform software for Royal Kludge keyboards written in C++/Qt. This Tauri-based implementation reimplements the protocol reverse-engineered by the Rangoli project.

### Key Contributions from Rangoli

- **Protocol Reverse Engineering**: The HID communication protocol was reverse-engineered by analyzing USB packets from the official RK Software
- **Keyboard Configuration Files**: Keyboard layouts and configurations are based on Rangoli's keyboard database
- **Feature Reference**: Lighting modes, key mapping, and per-key RGB features are based on Rangoli's implementation

Special thanks to:

- [@rnayabed](https://github.com/rnayabed) - Original Rangoli developer
- [@cben](https://github.com/cben) - Rangoli contributor
- [@hramrach](https://github.com/hramrach) - Rangoli contributor

## Disclaimer

**Royal Kludge Configurator is NOT official software. It is a community project.**

There is no guarantee that it will work with your keyboard. The protocol was reverse-engineered and may not work with all Royal Kludge keyboard models. Use at your own risk.

## License

This project is licensed under the same license as Rangoli. Please refer to the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

If you encounter any issues or have feature requests, please [open an issue](https://github.com/Ripwords/rk-configurator/issues).

## Acknowledgments

- [Rangoli](https://github.com/rnayabed/rangoli) - Original inspiration and protocol reference
- [Tauri](https://tauri.app) - Framework for building desktop applications
- [Nuxt UI](https://ui.nuxt.com) - UI component library
- [hidapi](https://github.com/libusb/hidapi) - HID device communication library
