# Contributing to FrameCut Studio

Thank you for your interest in contributing to **FrameCut Studio**, engineered and maintained by **IT AMADERSOMAJ INC.** (`com.itamadersomajinc.framecut`).

---

## 1. Clean Room & Code Standards

FrameCut Studio is an enterprise-grade, clean-room Non-Linear Video Editor written in modern Rust. All contributions must adhere to these foundational principles:

1. **Clean Room Verification**: Never copy code, decompiled routines, shaders, or proprietary text from commercial software (Premiere Pro, DaVinci Resolve, Final Cut Pro, CapCut, etc.). All algorithms must be derived from standard specifications (SMPTE, ITU-R, EBU, W3C) or authored from scratch.
2. **Deterministic Time**: All timeline calculations must use integer `filmcraft_time::Tick` (`254,016,000,000` ticks per second). Never hand-roll float-based timeline position math.
3. **No Panics in Production**: Production crates strictly forbid `panic!`, `unwrap()`, or `expect()`. All errors must propagate via standard `Result<T, E>` types.
4. **Hardware Acceleration with Graceful Fallback**: GPU rendering (Direct3D 12 / Vulkan / Metal) and hardware video codecs (NVENC, Intel QuickSync, AMD AMF) must automatically fall back to pure-Rust / CPU SIMD pipelines when hardware capability is absent.
5. **UI Accessibility & Theme Tokens**: All colors, fonts, margins, and icons must draw from `Tokens` (`crates/ui-egui/src/theme.rs`). No hardcoded pixel coordinates or non-theme hex colors.

---

## 2. Setting Up Your Development Environment

1. Clone the repository:
   ```powershell
   git clone https://github.com/mohammad-sheikh-shahinur-rahman/FrameCut-Studio-Windows.git
   cd FrameCut-Studio-Windows
   ```
2. Verify Rust toolchain:
   ```powershell
   rustc --version
   cargo --version
   ```
3. Run automated layer check:
   ```powershell
   cargo check -p filmcraft
   ```

---

## 3. Pull Request Guidelines

1. **Branch Naming**: Use descriptive branch names: `feature/color-curves`, `fix/timeline-thumbnails`, `perf/waveforms`.
2. **Single Responsibility**: Each pull request should address one specific feature or bug fix.
3. **Verification Checklist**: Before submitting your PR, verify:
   ```powershell
   cargo fmt --check
   cargo clippy -p <crate> --all-targets -- -D warnings
   cargo test -p <crate>
   ```
4. **Documentation**: Any new public API or engine command must include doc comments and integration test coverage.

---

## 4. Code of Conduct

We are committed to providing a welcoming, inclusive, and professional environment for all contributors. Treat every collaborator with respect, focus on technical merit, and uphold the highest standards of software craftsmanship.
