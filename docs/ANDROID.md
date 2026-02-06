# Ring Slint Android Guide

Build Android applications with Ring Slint.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Environment Setup](#environment-setup)
- [Building for Android](#building-for-android)
- [Project Configuration](#project-configuration)
- [Asset Management](#asset-management)
- [Platform Differences](#platform-differences)
- [Troubleshooting](#troubleshooting)

---

## Overview

Ring Slint supports Android with the following characteristics:

| Feature | Android Support |
|---------|-----------------|
| **Rendering** | GPU-accelerated via Skia (OpenGL ES / Vulkan) |
| **Min SDK** | 21 (Android 5.0 Lollipop) |
| **Target SDK** | 35 |
| **Architectures** | arm64-v8a, armeabi-v7a, x86_64 |

### Platform Limitations

The following desktop features are **not available** on Android:

- File dialogs (`fileOpen`, `fileSave`, `folderOpen`)
- Desktop notifications (`notify`)
- Global hotkeys (`hotkeyRegister`)
- System tray (`trayCreate`)
- Window drag (`windowDrag`)
- Always-on-top (`windowSetAlwaysOnTop`)
- Window icon (`windowSetIcon`)

---

## Prerequisites

### Required Tools

1. **Android NDK** (r25 or later, r27 recommended)
   - Download from [Android NDK Downloads](https://developer.android.com/ndk/downloads)
   
2. **Rust with Android targets**
   ```sh
   rustup target add aarch64-linux-android
   rustup target add armv7-linux-androideabi
   rustup target add x86_64-linux-android
   ```

3. **cargo-apk**
   ```sh
   cargo install cargo-apk
   ```

4. **Java JDK** (for APK signing)
   - JDK 11 or later recommended

---

## Environment Setup

### Set NDK Path

Add to your shell configuration:

**Bash (~/.bashrc):**
```bash
export ANDROID_NDK_ROOT="/path/to/android-sdk/ndk/27.0.0"
export PATH="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"
```

**Fish (~/.config/fish/config.fish):**
```fish
set -gx ANDROID_NDK_ROOT "/path/to/android-sdk/ndk/27.0.0"
set -gx PATH "$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin" $PATH
```

**Windows (PowerShell):**
```powershell
$env:ANDROID_NDK_ROOT = "C:\Android\sdk\ndk\27.0.0"
```

### Verify Installation

```sh
# Check NDK version
cat $ANDROID_NDK_ROOT/source.properties

# Verify Rust targets
rustup target list --installed | grep android
```

---

## Application Entry Point

On Android, Ring Slint automatically loads and executes `main.ring` from the assets directory. This file is **required** and serves as the entry point for your application.

### Required Files

Your `assets/` directory must contain:

| File | Required | Description |
|------|----------|-------------|
| `main.ring` | **Yes** | Application entry point (loaded and executed automatically) |
| `slint.ring` | **Yes** | Ring Slint bindings (copy from `src/slint.ring`) |
| `*.slint` | Yes | Your UI definition files |
| Other assets | Optional | Images, fonts, etc. |

### How It Works

1. Android app starts → `android_main()` is called
2. Slint Android backend is initialized
3. Assets are extracted to internal storage
4. `main.ring` is loaded from assets and executed
5. Your Ring code runs (loads UI, starts event loop)

### Example main.ring

```ring
load "slint.ring"

oApp = new SlintApp {
    loadUI("app.slint")
    show()
    run()
}
```

> **Note:** The `load "slint.ring"` statement loads the Ring Slint bindings. The `.slint` files are loaded relative to the extracted assets directory.

---

## Building for Android

### Debug Build

```sh
cd src/rust_src
cargo apk build
```

### Release Build

```sh
cd src/rust_src
cargo apk build --release
```

The APK will be generated at:
```
target/release/apk/ring_slint.apk
```

### Install on Device

```sh
# Install via ADB
adb install target/release/apk/ring_slint.apk

# Or build and run directly
cargo apk run --release
```

---

## Project Configuration

### Cargo.toml Android Settings

The Android configuration is in `src/rust_src/Cargo.toml`:

```toml
[package.metadata.android]
package = "dev.ring.slint"
build_targets = ["aarch64-linux-android", "armv7-linux-androideabi", "x86_64-linux-android"]
assets = "assets"

[package.metadata.android.sdk]
min_sdk_version = 21
target_sdk_version = 35

[package.metadata.android.application]
label = "Ring Slint App"

[package.metadata.android.signing.release]
path = "/path/to/your/keystore.jks"
keystore_password = "your_password"
key_alias = "your_alias"
key_password = "your_key_password"
```

### Configuration Options

| Option | Description |
|--------|-------------|
| `package` | Android package name (e.g., `com.example.myapp`) |
| `build_targets` | Target architectures to build |
| `assets` | Directory containing assets (`.slint` files, images) |
| `min_sdk_version` | Minimum Android API level |
| `target_sdk_version` | Target Android API level |
| `label` | App name shown in launcher |

### Release Signing

For release builds, you need a signing keystore:

1. **Create a keystore** (if you don't have one):
   ```sh
   keytool -genkey -v -keystore my-release-key.jks -keyalg RSA -keysize 2048 -validity 10000 -alias my-alias
   ```

2. **Configure signing** in `Cargo.toml`:
   ```toml
   [package.metadata.android.signing.release]
   path = "/path/to/my-release-key.jks"
   keystore_password = "your_keystore_password"
   key_alias = "my-alias"
   key_password = "your_key_password"
   ```

> **Security Note:** Don't commit keystore passwords to version control. Use environment variables or a separate config file.

---

## Asset Management

### Asset Directory Structure

Place your `main.ring`, `.slint` files, and other assets in the `assets` directory:

```
src/rust_src/
├── assets/
│   ├── main.ring          # REQUIRED - App entry point
│   ├── slint.ring         # REQUIRED - Copy from src/slint.ring
│   ├── app.slint          # Your UI definition
│   ├── components/
│   │   └── button.slint
│   └── images/
│       └── logo.png
├── src/
│   └── ...
└── Cargo.toml
```

> **Important:** Both `main.ring` and `slint.ring` are required in the `assets/` directory. Copy `slint.ring` from the project's `src/` directory. `main.ring` is automatically loaded and executed when the Android app starts.

### Loading Assets

On Android, asset paths are automatically resolved relative to the assets directory:

**assets/main.ring:**
```ring
load "slint.ring"

oApp = new SlintApp {
    loadUI("app.slint")  // Loads from assets/app.slint
    show()
    run()
}
```

### Slint Imports

Imports in `.slint` files also resolve from assets:

```slint
// main.slint
import { MyButton } from "components/button.slint";

export component App inherits Window {
    MyButton { }
}
```

### Images in Slint

Reference images using `@image-url`:

```slint
export component App inherits Window {
    Image {
        source: @image-url("images/logo.png");
    }
}
```

---

## Platform Differences

### Conditional Code

Check platform at runtime if needed:

```ring
// In Slint, you can use @platform
// In Ring, check for feature availability
```

### Available Features on Android

| Category | Available | Not Available |
|----------|-----------|---------------|
| **UI** | All Slint components | - |
| **Properties** | `set`, `get`, `setBool` | - |
| **Callbacks** | All callback methods | - |
| **Timers** | All timer methods | - |
| **Models** | All model methods | - |
| **Styles** | All styles | - |
| **Window** | Basic window management | `windowDrag`, `windowSetAlwaysOnTop`, `windowSetIcon` |
| **Dialogs** | - | All file/message dialogs |
| **System** | - | Notifications, Hotkeys, Tray |
| **Clipboard** | - | Clipboard operations |

### Handling Unsupported Features

The desktop-only methods will fail silently or return empty values on Android. Design your app to handle this:

```ring
// Don't rely on desktop features on Android
// Use Slint's built-in dialogs instead of native ones
```

### Android-Specific UI Considerations

1. **Touch-friendly targets**: Make buttons at least 48x48 dp
2. **Responsive layouts**: Use relative sizing
3. **Safe areas**: Account for notches and navigation bars
4. **Back button**: Handle Android back navigation

---

## Troubleshooting

### Common Issues

#### NDK Not Found

```
error: could not find Android NDK
```

**Solution:** Set the `ANDROID_NDK_ROOT` environment variable:
```sh
export ANDROID_NDK_ROOT="/path/to/ndk"
```

#### Missing Rust Targets

```
error: target 'aarch64-linux-android' not found
```

**Solution:** Install Android targets:
```sh
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
```

#### Linker Errors

```
error: linker `aarch64-linux-android-clang` not found
```

**Solution:** Ensure NDK toolchain is in PATH:
```sh
export PATH="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"
```

#### APK Installation Fails

```
INSTALL_FAILED_UPDATE_INCOMPATIBLE
```

**Solution:** Uninstall the existing app:
```sh
adb uninstall dev.ring.slint
```

#### App Crashes on Start

Check logcat for errors:
```sh
adb logcat -s RustStdoutStderr:D ring_slint:D
```

### Debugging Tips

1. **Enable logging** in your Ring code:
   ```ring
   ? "Debug: Starting app"
   ```

2. **View logs** with ADB:
   ```sh
   adb logcat | grep -i ring
   ```

3. **Check APK contents**:
   ```sh
   unzip -l target/release/apk/ring_slint.apk
   ```

4. **Verify assets** are included:
   ```sh
   unzip -l target/release/apk/ring_slint.apk | grep assets
   ```

---

## Example: Android App

Here's a complete example that works on both desktop and Android:

**assets/app.slint:**
```slint
import { Button, VerticalBox } from "std-widgets.slint";

export component App inherits Window {
    title: "Cross-Platform App";
    
    in-out property <int> counter: 0;
    callback increment();
    
    VerticalBox {
        alignment: center;
        padding: 20px;
        
        Text {
            text: "Count: " + counter;
            font-size: 32px;
            horizontal-alignment: center;
        }
        
        Button {
            text: "Increment";
            clicked => { increment(); }
        }
    }
}
```

**main.ring:**
```ring
load "slint.ring"

nCount = 0

oApp = new SlintApp {
    loadUI("app.slint")
    setCallback("increment", :onIncrement)
    show()
    run()
}

func onIncrement
    nCount++
    oApp.set("counter", nCount)
```

Build and run:
```sh
cd src/rust_src
cargo apk run --release
```

---

## Resources

- [Slint Android Documentation](https://slint.dev/docs/rust/slint/android/)
- [Android NDK Guide](https://developer.android.com/ndk/guides)
- [cargo-apk Documentation](https://github.com/rust-mobile/cargo-apk)
- [Ring Slint Examples](../examples/)
