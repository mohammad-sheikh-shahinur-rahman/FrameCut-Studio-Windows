# Contributing to FrameCut Studio

**Engineered & Published by IT AMADERSOMAJ INC.**  
Application ID: `com.itamadersomajinc.framecut` / `app.framecutstudio.editor`

Thank you for contributing to **FrameCut Studio**! FrameCut Studio is an enterprise-grade, 100% on-device Non-Linear Video Editor (NLE) that unifies real-time live preview and final hardware export through a deterministic Unified Rendering Graph.

---

## 1. Ground Rules & Code Standards

Every contribution to FrameCut Studio must adhere to our core architectural principles:

- **100% Local Execution**: Zero cloud dependencies, zero external network telemetry, zero mock UI. Everything must run deterministically on the user's hardware.
- **Hardware Acceleration & Clean Fallbacks**: All GPU shaders, neural models, and audio DSP algorithms must feature verified software fallbacks (e.g. Vulkan / Direct3D 12 → OpenGL ES / CPU).
- **Clean-room LGPL Compliance**: Never introduce GPL-contaminated codecs or dependencies that violate Google Play or commercial distribution policies.
- **Test-Driven Reliability**: Every new feature, transform, filter, or audio processor must include automated unit tests verifying deterministic rendering.

---

## 2. Development Setup

### Desktop (Windows, macOS, Linux)
You need:
- **Rust** 1.93 or newer (see `rust-version` in `src/Cargo.toml`)
- **FFmpeg** 7+ development libraries (BtbN LGPL-shared build)
- **ONNX Runtime** (1.28.0)
- **CMake** and C++ Build Tools (MSVC on Windows, Clang on macOS/Linux)

To run the desktop editor:
```bash
cd src
cargo run --bin framecut --profile quick
```

### Android (Phones, Foldables, Tablets)
You need:
- **Android Studio** Ladybug or newer
- **JDK 17 / 21**
- **Android SDK** API 35 with NDK (r26+)

To run tests and assemble release APK:
```bash
gradle testDebugUnitTest --console=plain
gradle assembleDebug assembleRelease --console=plain
```

---

## 3. Automated Verification Suites

Before opening a pull request, run the automated test suite to ensure zero regressions:

```bash
# Workspace unit tests
cargo test --workspace

# Linter and formatting checks
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

---

## 4. Issues & Feedback

Please submit bug reports, performance metrics, and feature requests directly to our official repository:  
🔗 **[https://github.com/mohammad-sheikh-shahinur-rahman/FrameCut-Studio-Windows/issues](https://github.com/mohammad-sheikh-shahinur-rahman/FrameCut-Studio-Windows/issues)**

---

© 2026 IT AMADERSOMAJ INC. All rights reserved.
