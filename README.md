# FrameCut Studio — Professional Native Desktop Video Editing Architecture

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Platform](https://img.shields.io/badge/platform-Windows%2010%20%2F%2011%20(x86__64)-blue.svg)]()
[![Architecture](https://img.shields.io/badge/architecture-Pure%20Rust%20%2B%20WGPU%20Direct3D12-orange.svg)]()
[![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue.svg)](./LICENSE-MIT)
[![Engineered By](https://img.shields.io/badge/publisher-IT%20AMADERSOMAJ%20INC.-indigo.svg)]()
[![Package ID](https://img.shields.io/badge/app%20id-com.itamadersomajinc.framecut-success.svg)]()

> **100% On-Device · Zero Mockups · Zero Cloud Uploads · Zero Watermarks · Zero Subscriptions**  
> **Engineered & Published by IT AMADERSOMAJ INC.**  
> **Application Identifier:** `com.itamadersomajinc.framecut` / `app.framecutstudio.editor`

---

## 1. Executive Summary & Core Mission

**FrameCut Studio** is a production-grade, 100% local, high-performance Non-Linear Video Editing Studio (NLE) engineered natively for Windows (and cross-platform desktops). Built from clean-room architecture in pure Rust, modern Direct3D 12 / Vulkan / Metal GPU shaders via WGPU, and hardware-accelerated media decoders/encoders, FrameCut Studio unifies live preview, multi-track timeline editing, professional 3-way color grading, audio DSP mixing, and final hardware export through a deterministic Unified Rendering Pipeline.

Every subsystem in FrameCut Studio is backed by real mathematical transforms, DSP audio filtering, GPU shader execution, integer-tick edit math (`254,016,000,000` ticks/second for exact synchronization across any frame rate), and hardware codec acceleration—with **zero mock UI, zero simulated buttons, zero cloud dependencies, and 100% offline reliability**.

---

## 2. 10-Engine Native Media Architecture & Integration Matrix

FrameCut Studio evaluates, integrates, and coordinates 10 specialized native and platform media engine domains without pipeline conflicts or duplicate decode/encode passes:

| # | Media Engine Domain | Concrete Role in FrameCut Studio | Native / Platform Implementation | Hardware vs Software Fallback |
|---|---|---|---|---|
| **1** | **Native Media Pipelines** | Elementary stream demuxing (MP4, MKV, MOV, TS), software codec fallback, PCM sample-rate conversion, and high-precision filtering | High-performance container parsers (`filmcraft-isobmff`, `filmcraft-matroska`), `symphonia` audio decoding, and DirectShow / Media Foundation / native FFmpeg fallbacks | Hardware DXVA2 / D3D11 / NVDEC / QSV primary with automatic fallback to high-throughput CPU SIMD decoders |
| **2** | **Unified Render Graph & Host Compositor** | Frame-accurate multi-player timeline preview, multi-track composition, layer blending, and transform trees | `filmcraft-render`, `filmcraft-frame`, and WGPU native desktop swapchain | Hardware surface-to-surface encoding with automatic profile/level negotiation and H.265 → H.264 fallback |
| **3** | **GPU Rendering Engine** | Real-time color transforms, multi-layer alpha blending, 2D/3D title rasterization, and custom GLSL/WGSL post-processing effects | Direct3D 12, Vulkan, and Metal pipelines via WGPU (`filmcraft-gpu`), compiled shader kernels | Direct3D 12 primary, falling back to Direct3D 11 / Vulkan, with CPU software blit fallback via Rayon SIMD |
| **4** | **Professional Color Grading Suite (Lumetri)** | 3-Way Color Wheels (Lift / Gamma / Gain), RGB Splines, Hue-vs-Hue/Sat/Luma curves, HSL secondary isolation, and 3D LUTs (.cube) | `filmcraft-color` with IEEE 754 float32 precision, ACEScg / Rec. 709 / Rec. 2020 / PQ / HLG color space transforms | Compute shader acceleration with 64x64x64 tetrahedral LUT interpolation fallback |
| **5** | **Real-Time Video Scopes** | Live visual analysis: Waveform (Luma / RGB Parade), Vectorscope, Histogram, and CIE Chromaticity Diagram | `filmcraft-scopes` multi-threaded buffer analysis with GPU compute shader acceleration | Direct GPU texture readback with multi-threaded SIMD CPU fallback |
| **6** | **Audio DSP & 64-Bit Mixing Engine** | Multi-track audio mixing, 5-Band Parametric Biquad EQ, Lookahead Peak Limiter, Compressor, Noise Gate, and Voiceover | `filmcraft-audio-dsp`, `cpal` low-latency WASAPI / CoreAudio backend, 64-bit double precision audio bus | Hardware audio endpoints with multi-buffer underflow protection |
| **7** | **Offline AI Subtitles & Voice Synthesis** | On-device automatic speech recognition (Whisper model) and natural text-to-speech voice generation | `filmcraft-speech` (Whisper CPU/Vulkan ONNX runtime) and `filmcraft-tts` (Kokoro neural voice model) | Fully local CPU AVX2 / Vulkan execution with zero network dependency |
| **8** | **High-Throughput Hardware Export Engine** | Surface-to-surface MP4, MOV, ProRes, MKV export with exact color/transform parity with the live preview monitor | `filmcraft-export` with Direct3D 11 / NVENC / Intel QuickSync / AMD AMF hardware encoder bindings | Hardware NVENC / QSV / AMF primary with multi-threaded rav1e / openh264 / svt-av1 fallback |
| **9** | **Project Persistence & Atomic Transaction Ledger** | Project manifest serialization, clip indexing, undo/redo history trees, and automatic crash recovery | `filmcraft-project`, `filmcraft-engine`, atomic transaction journals (`.fcproj` / `project.json`) | Memory-mapped read with double-buffered temp atomic rename write |
| **10** | **Asynchronous Task & Render Worker Pool** | Non-blocking proxy video generation, background thumbnail caching, audio waveform extraction, and batch export queue | Cross-platform multi-threaded worker threadpools (`rayon`, `tokio`, custom dedicated render queues) | Automatic core detection and CPU affinity scaling |

---

## 3. Dedicated Workflow Workspaces

FrameCut Studio features 6 prominent one-click studio workspaces located right on the top header bar:

1. **Import (`Mode::Import`)**: High-throughput media browser, asset ingest, thumbnail grid, and metadata prober.
2. **Edit (`Mode::Edit - Editing`)**: Primary multi-track NLE workspace with Program Monitor, Source Monitor, Multi-Track Timeline, Essential Graphics, and Tool Palette.
3. **Color Grading (`Mode::Edit - Color`)**: Dedicated DaVinci / Premiere-style color finishing studio with **3-Way Color Wheels** (Lift / Gamma / Gain), **Tone Curves**, **HSL Secondary Keying**, **Input / Look 3D LUTs**, and **Real-Time Video Scopes** (Waveform, RGB Parade, Vectorscope).
4. **Effects (`Mode::Edit - Effects`)**: Complete visual effects laboratory with spatial transforms, drop shadows, blend modes, motion blur, and video filter cascades.
5. **Audio (`Mode::Edit - Audio`)**: Audio Track Mixer, Essential Sound, Biquad Parametric EQ, Dynamics, and master bus metering.
6. **Export (`Mode::Export`)**: Multi-format hardware export suite with resolution presets (4K UHD, 1080p, 720p, 9:16 Shorts/Reels), CRF/bitrate control, and background rendering queue.

---

## 4. Multi-Track Timeline & Responsive UI Features

- **Pill Badge Clip Names**: Every audio and video clip on the timeline renders its true asset name inside a high-contrast, semi-transparent pill badge, guaranteeing complete legibility over continuous video thumbnail filmstrips and audio waveforms.
- **Dynamic Track Auto-Discovery**: Video tracks (V1, V2, V3...), Audio tracks (A1, A2, A3...), Subtitle tracks, and Adjustment layers with individual Lock, Mute, Solo, and Visibility switches.
- **Intelligent Playhead Color Grading**: Moving the playhead across the timeline automatically targets the topmost visible video clip for instant color grading adjustments without requiring tedious manual track item re-selection.
- **Full Undo / Redo System**: Unlimited-depth transaction history stack protecting edits, splits, trims, effect additions, and volume adjustments.
- **Auto-Save & Crash-Recovery Journal**: Continuous background journaling prevents data loss in the event of hardware power interruptions.

---

## 5. Technology Stack & Directory Structure

```
FrameCut Studio/
├── apps/
│   ├── filmcraft/              # Main desktop application shell (framecut executable)
│   ├── filmcraft-cli/          # Headless automation CLI & MCP integration server
│   └── filmcraft-web/          # WebAssembly browser companion
├── crates/
│   ├── audio-dsp/              # Biquad EQ, Peak Limiter, Noise Gate, Compressor DSP
│   ├── bitstream/              # Bitstream decoding & parsing utilities
│   ├── captions/               # SRT, VTT, and embedded subtitle parser & renderer
│   ├── color/                  # Color grading mathematics, LUTs, ACEScg, Rec.709
│   ├── edit/                   # Timeline ripple, roll, slip, slide, trim algorithms
│   ├── engine/                 # Central session state machine & command bus
│   ├── export/                 # Hardware encoder orchestration (H.264, HEVC, AV1)
│   ├── frame/                  # Video frame buffer layouts (RGBA8, RGBA16F, YUV420P)
│   ├── geom/                   # 2D & 3D coordinate transformations and matrix math
│   ├── gpu/                    # Direct3D 12 / Vulkan / Metal WGPU pipelines
│   ├── interchange/            # Final Cut XML, EDL, and AAF project interchange
│   ├── isobmff/                # MP4 / QuickTime MOV container parser & demuxer
│   ├── matroska/               # MKV / WebM container parser & demuxer
│   ├── media/                  # Hardware video decoding & frame presentation
│   ├── platform/               # OS native windowing, file associations, and dialogs
│   ├── project/                # Project manifest serialization (.fcproj)
│   ├── render/                 # Compositor graph, layer blending, and opacity trees
│   ├── scopes/                 # Luma waveform, RGB parade, and vectorscope math
│   ├── speech/                 # Local Whisper ASR neural model integration
│   ├── text/                   # HarfBuzz / FreeType text shaper & 3D title engine
│   ├── time/                   # Exact integer Tick timeline clock (254,016,000,000/sec)
│   ├── tts/                    # Kokoro neural voice speech synthesizer
│   └── ui-egui/                # Responsive professional desktop interface
├── packaging/                  # Inno Setup Windows installer & MSIX packaging scripts
└── xtask/                      # Build automation & verification tasks
```

---

## 6. Building & Running FrameCut Studio

### Prerequisites
- **Rust Toolchain**: Rust 1.95+ or stable `2024` edition.
- **Windows Build Tools**: Visual Studio 2022 / C++ Build Tools with Windows 10/11 SDK.
- **Git**: Installed and on system PATH.

### Quick Build (Development)
```powershell
# Check project crates
cargo check -p filmcraft

# Build fast local binary
cargo build --profile quick -p filmcraft --bin framecut

# Run FrameCut Studio
cargo run --profile quick -p filmcraft --bin framecut
```

### Production Release Build
```powershell
# Compile optimized native Windows binary
cargo build --release -p filmcraft --bin framecut

# The compiled executable is generated at:
# target/release/framecut.exe
```

---

## 7. Quality Assurance & Automated Testing

FrameCut Studio enforces strict architectural boundaries:
- **No panics in production code**: All error paths return structured `Result<T, E>`.
- **Pure Rust core**: Zero unverified FFI boundaries or dangerous memory operations.
- **Layer hierarchy validation**: Run `cargo xtask layers` to ensure lower tiers (L0–L4) remain independent of GUI abstractions.

To run the automated suite:
```powershell
# Run workspace test suite
cargo test --workspace

# Check formatting
cargo fmt --check

# Strict Clippy lint check
cargo clippy --workspace --all-targets -- -D warnings
```

---

## 8. License & Intellectual Property

FrameCut Studio is published by **IT AMADERSOMAJ INC.** (`com.itamadersomajinc.framecut`) and licensed under the dual **MIT OR Apache-2.0** license.

All assets, icons, fonts, and codecs conform to clean-room engineering standards. See [LICENSING.md](./LICENSING.md) and [ATTRIBUTION.md](./ATTRIBUTION.md) for full licensing and third-party notices.
