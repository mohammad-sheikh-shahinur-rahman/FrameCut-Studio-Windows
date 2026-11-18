# FrameCut Studio Native Media Engine

**Engineered & Published by IT AMADERSOMAJ INC.**  
Application ID: `com.itamadersomajinc.framecut` / `app.framecutstudio.editor`  
© 2026 IT AMADERSOMAJ INC. All rights reserved.

The native 10-engine media architecture behind **FrameCut Studio**: 100% local Rust & native platform execution, zero GC, deterministic Unified Rendering Graph.

---

## 10-Engine Architecture & Module Mapping

| Engine # | Engine Domain | Workspace Crates | Implementation Details |
|---|---|---|---|
| **1** | **FFmpeg Native Media Pipeline** | `concat-media` | Probing (`ffprobe`), software/hardware codec decoding & encoding (`libavcodec`), format demuxing (`libavformat`), pixel format conversion (`libswscale`), PCM audio resampling (`libswresample`). |
| **2** | **Unified Render Graph & Compositor** | `concat-render`, `concat-core` | Frame-accurate multi-track timeline preview, `RenderGraph` directed acyclic graph compilation, spatial matrix transforms, track compositing, and layer sequencing. |
| **3** | **GPU Rendering & Shaders (Direct3D 12 / Vulkan / GLES)** | `concat-render`, `concat-effects` | Native Direct3D 12 and Vulkan rendering via WGPU 30. Single-pass WGSL/GLSL shaders for color grading, 8-Band HSL Secondary, 5-Point Tone Curves, Chroma Key, Luma Matte & transitions. |
| **4** | **3D Typography & Volumetric Titles** | `concat-text`, `concat/text-presets` | Volumetric text extrusion (0..20px), 3D perspective pitch (`tiltX`) & yaw (`tiltY`), directional Lambertian bevel shading, and 11 bundled Google Fonts. |
| **5** | **Low-Latency Audio DSP & Mixer** | `concat-speech`, `concat-core` | Real-time 5-Band Parametric EQ (60Hz–14kHz), -0.3 dBFS brickwall peak limiter, 17 studio audio effects, voiceover ducking, and lock-free PCM-16 ring buffers. |
| **6** | **MediaPipe / AI Auto-Cutout & Vision** | `concat-vision` | 100% on-device neural person segmentation (Auto-Cutout), bilateral alpha feathering, DSLR bokeh blur, and planar motion tracking. |
| **7** | **ONNX Neural Video Engine (1.28.0)** | `concat-vision` | Real-time Opset-15 / IR-v8 neural video enhancement, spatial noise reduction, and 2× AI super-resolution upscaling via bundled ONNX Runtime. |
| **8** | **High-Speed Native Media APIs** | `concat-media`, `concat-host` | High-speed PCM waveform extraction, beat-marker detection, and hardware intra-frame video reverse transcoding. |
| **9** | **Local Database & Atomic Manifest Ledger** | `concat-host`, `concat-project` | Structured project metadata (`framecut.json`), atomic state ledger, and 80-step deep undo/redo stack. |
| **10** | **Deferred Task Engine & Master Export** | `concat-export` | Multi-format master export engine: MP4 (H.264/HEVC up to 4K 60fps), Batch 3-Aspect Export (9:16 + 1:1 + 16:9), Animated GIF, Audio-Only M4A, PNG still covers, and hardware proxy workflows. |

---

## Workspace Layout & Dependency Hierarchy

```
concat (Desktop Editor UI) -> concat-speech -> concat-host -> {concat-export, concat-project, concat-media} -> concat-core
                                                           -> concat-render -> concat-core
                                                           -> concat-export -> concat-effects -> concat-project
                                                           -> concat-vision (ONNX Runtime 1.28.0)
```

- `concat-core`: Timeline models, time arithmetic, geometry, blend modes, curves. Standard library only.
- `concat-media`: Direct hardware & FFmpeg media I/O.
- `concat-project`: The project document, 80-step undo/redo stack, timeline tracks, and command execution.
- `concat-render`: WGPU / Direct3D 12 video compositor.
- `concat-effects`: The catalog of 26 film filters, 22 GPU effects, and 14 transitions.
- `concat-text`: Vector text shaping, multi-language font typography, and subtitle (.SRT) parser/serializer.
- `concat-vision`: ONNX neural inference and auto-cutout segmentation.
- `concat-speech`: Whisper speech-to-text, audio DSP effects, and parametric equalization.
- `concat-export`: Multi-threaded frame-by-frame master export and proxy generation pipeline.
- `concat-host`: System integration, sessions, GPU probing, and audio devices.
- `concat`: The complete cross-platform Slint GUI editor.

---

## Build Commands

```sh
# Desktop editor development build (optimized for quick incremental compilation)
cargo build --profile quick -p concat --bin framecut

# Production shipping build (fat LTO, stripped)
cargo build --profile app -p concat --bin framecut

# Run full automated test suite
cargo test --workspace
```

---

© 2026 IT AMADERSOMAJ INC. All rights reserved.
