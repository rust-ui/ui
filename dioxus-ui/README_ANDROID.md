# Android Setup (Mac Apple Silicon)

## Prerequisites

### 1. Android Studio
Install from https://developer.android.com/studio. Open it once to let it install the SDK.

### 2. NDK
In Android Studio → SDK Manager → SDK Tools → check **NDK (Side by side)** → Apply.

### 3. Environment variables
Add to `~/.zshrc`:
```zsh
export ANDROID_HOME="$HOME/Library/Android/sdk"
export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"
```
Then `source ~/.zshrc`.

### 4. Rust target
```bash
rustup target add aarch64-linux-android
```

### 5. AVD (emulator)
In Android Studio → Device Manager → Create Virtual Device.
Pick any phone, select **API 35+** with **arm64-v8a** image (required on Apple Silicon).

## Run

```bash
dx serve --platform android
```

The emulator must be running before serving (start it from Android Studio or `emulator -avd <name>`).
