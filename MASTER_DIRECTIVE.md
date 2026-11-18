# MASTER DEVELOPMENT DIRECTIVE
## FrameCut Professional — Native Windows Video Editing Studio

> **Status:** ACTIVE & PRODUCTION-GRADE  
> **Target Platform:** Windows 10 & 11 (x86_64 Native Desktop)  
> **Publisher & Maintainer:** IT AMADERSOMAJ INC.  
> **Application Identifier:** `com.itamadersomajinc.framecut` / `app.framecutstudio.editor`  
> **Core Guarantee:** 100% On-Device · Zero Mockups · Zero Cloud Uploads · Zero Paywalls · Zero Watermarks

---

### 1. Project Mission

Transform the complete FrameCut architecture and all approved professional editing capabilities into a fully functional, production-grade Windows Desktop Video Editing Application.

This is not a prototype, UI mockup, demonstration application, or feature simulation.

Every feature must have:
- A real implementation.
- A functional user interface.
- A connected native processing engine.
- Real-time preview where technically feasible.
- Actual rendered output.
- Project persistence.
- Undo/Redo integration where applicable.
- Automated tests.
- Hardware capability detection.
- Graceful software fallback.

Do not claim that a feature is implemented merely because its button, menu, slider, or interface exists.

---

### 2. Windows Technology Architecture

Build a Windows-native architecture with the following technology stack:

**Core Application**
- Native Windows Desktop application.
- C++20 or newer for performance-critical media processing.
- High-performance modular Rust core for safe, thread-safe memory handling.
- Windows App SDK / WinUI 3 & Slint High-DPI Desktop Workspace.
- DirectComposition for advanced desktop composition.
- MVVM / Reactive state architecture for the application interface.

**Media Processing**
- FFmpeg LGPL-compatible integration (`libavcodec`, `libavformat`, `libavutil`, `libswscale`, `libswresample`).
- Windows Media Foundation (MF Source Reader and Sink Writer).
- Native Media Foundation transforms and hardware codec MFTs.
- Custom media probing and metadata extraction (`MediaProber`).
- Custom audio mixing and video rendering pipelines (`concat-media`, `concat-speech`).

**GPU Rendering**
- Direct3D 12 as the primary Windows rendering backend via WGPU 30.
- DirectCompute for compute workloads.
- Vulkan as an optional secondary backend.
- OpenGL and Software Warp rasterizer as compatibility fallbacks.
- Single-pass HLSL and WGSL/GLSL fragment color grading and transition shaders.
- Hardware decoding and encoding through supported Windows APIs (NVENC, Intel QSV, AMD AMF).

**AI Engine**
- Microsoft ONNX Runtime 1.28.0.
- DirectML Execution Provider for GPU/NPU acceleration.
- Windows ML where suitable.
- CPU execution fallback with SIMD (AVX2 / AVX-512 / XNNPACK).
- Local neural model management without mandatory cloud services.

**Storage**
- SQLite database for project metadata and media asset indexing.
- Local JSON project manifests (`framecut.json`).
- Transaction-safe project persistence with atomic write-replace and recovery snapshots.
- Asynchronous file operations and directory migration (`adopt()`).

**Background Processing**
- Native Windows background task scheduler.
- Persistent job queue surviving restarts.
- Thread pool for multi-threaded decoding and rendering.
- Cancellation and crash recovery.
- Live progress reporting.

---

### 3. Ten Native Media Engine Domains

Implement all ten engine domains from the original FrameCut specification using Windows-compatible implementations:

1. **FFmpeg Native Media Engine**:
   - Codec decoding and encoding.
   - Container parsing (MP4, MKV, MOV, WebM, AVI, TS).
   - Elementary stream probing (`ffprobe`).
   - PCM audio resampling (`swresample`).
   - Video filtergraphs (`avfilter`).
   - Software codec fallback (`avcodec`).
   - Clean-room LGPL-compliant distribution strategy.

2. **Timeline and Media Composition (Unified Render Graph)**:
   - Multi-track non-linear timeline.
   - Frame-accurate video and audio composition.
   - Multi-source spatial compositing.
   - Deterministic DAG execution across live preview and final export.
   - Real-time preview at 60 fps.
   - MP4 / M4A / WebM / GIF muxing.

3. **GPU Rendering Engine (Direct3D 12 + Vulkan + GLSL/WGSL)**:
   - Direct3D 12 primary swapchain backend.
   - Vulkan compatibility backend.
   - OpenGL and Warp software fallbacks.
   - WGSL / HLSL custom shader execution.
   - Single-pass GPU color grading and 3D LUT evaluation.
   - Real-time GPU transitions and motion blur.
   - Real-time hardware capability detection.

4. **3D Typography & Volumetric Title Engine**:
   - Filament-grade 3D perspective pitch (`tiltX`) and yaw (`tiltY`).
   - True volumetric multi-layer 3D text depth extrusion (0..20px).
   - Directional Lambertian lighting and bevel specular highlights.
   - Vector path glyph rasterization and high-DPI scaling.
   - Real-time preview and exact export parity.

5. **Professional Low-Latency Audio DSP & Mixer**:
   - Lock-free in-place 32-bit floating point ring buffers.
   - 5-Band Parametric Graphic EQ (60Hz, 230Hz, 910Hz, 3.6kHz, 12kHz biquads).
   - -0.3 dBFS Master Brickwall Peak Limiter preventing digital clipping.
   - 17 Studio AudioFX (Bass Boost, Compressor, Reverb, Delay, Bitcrush, Pitch Shift, etc.).
   - LUFS integrated and momentary loudness metering + true-peak metering.
   - Multi-track PCM WAV bounce and live mic voiceover ducking.

6. **AI Vision & Auto-Cutout Engine**:
   - 100% offline neural person segmentation (SelfieSegmentation / Portrait matting).
   - Bilateral edge feathering and alpha matting.
   - DSLR bokeh depth-of-field background blur.
   - Automated planar subject motion tracking (`OFFSET_X` / `OFFSET_Y` keyframes).
   - Zero cloud uploads and zero remote network requests.

7. **ONNX Neural Video Engine**:
   - Real-time Opset-15 / IR-v8 convolutional neural video enhancement.
   - Spatial noise reduction and frame detail recovery.
   - 2× AI super-resolution upscaling (`Resize` + `Conv` tensor pipeline).
   - AI auto-enhance histogram and contrast analysis.
   - DirectML NPU/GPU acceleration with graceful CPU SIMD fallback.

8. **Native Windows Media Capture & Probing**:
   - Studio camera video and photo capture into project media bin.
   - Hardware-assisted video recording with zero frame drops.
   - High-speed PCM audio waveform extraction and beat peak detection.
   - Hardware intra-frame video reverse transcoder (`GopTranscoder`).
   - MediaFoundation and DirectShow hardware muxing.

9. **Project Database & Atomic State Ledger**:
   - Structured project metadata indexing and clip duration caching.
   - Atomic JSON state ledger (`framecut.json`) with `.bak` recovery snapshots.
   - 80-step deep historical undo/redo transaction stack.
   - Reactive UI StateFlow observers for zero-latency workspace sync.

10. **Background Task Engine & Master Export**:
    - Deferred exports surviving app restarts.
    - Automated 540p / 720p hardware proxy generation.
    - Batch ONNX neural enhancement queue.
    - Persistent task queue with threadpool worker management.
    - Task cancellation and crash recovery.
    - Live progress reporting and frame render metrics.

---

### 4. Complete Professional Editing Toolset Registry (80 Tools)

#### A. Timeline & Sequence Editing (10 Tools)
- **Split Clip at Playhead**: Frame-accurate split into two contiguous clips, preserving speed curves and effect chains.
- **Trim In / Trim Out**: Sub-frame interactive drag-handles with dynamic scrubbing.
- **Slip Source Window**: Source media offset adjustment without altering timeline boundaries.
- **Magnetic Ripple Editing**: Automated gap compaction and track ripple on delete and trim.
- **Duplicate Clips**: Instant cloning of clips with identical transformation and effect states.
- **Swap Adjacent Clips**: Seamless positional interchange between adjacent clips.
- **Merge Split Clips**: Lossless healing of split segments originating from the same source file.
- **Multiple Sequences**: Support for independent sequences and nested timelines.
- **Track Controls**: Independent Track Mute, Solo, Lock, and Custom Renaming.
- **Chapter & Beat Markers**: Interactive color-coded markers with keyboard jump navigation.
- **80-Step Undo / Redo**: Deep transaction stack covering all timeline edits and inspector actions.

#### B. Transform, 3D Geometry & Compositing (6 Tools)
- **2D Transform**: Position X/Y, Scale, Rotation Z, Flip H/V, Stretch X/Y, and Opacity.
- **3D Perspective Projection**: Pitch (`tiltX`) and Yaw (`tiltY`) 3D camera projection.
- **4-Edge Sub-Pixel Crop**: Independent sub-pixel trimming on Left, Top, Right, and Bottom edges.
- **9 Picture-in-Picture Presets**: Fullscreen, Top-Left, Top-Right, Bottom-Left, Bottom-Right, Split Left, Split Right, Split Top, Split Bottom.
- **9 Compositing Blend Modes**: Normal, Multiply, Screen, Overlay, Darken, Lighten, Add (Color Dodge), Soft Light, Hard Light.
- **Multi-Property Keyframing**: Keyframe curves across Position, Scale, Rotation, Opacity, and Volume (Linear, Ease-In, Ease-Out, Ease-In-Out).

#### C. Color Grading, Curves, Wheels & Scopes (6 Tools)
- **12-Channel Primary Grading**: Exposure, Contrast, Saturation, Vibrance, Highlights, Shadows, Whites, Blacks, Temp, Tint, Sharpen, Vignette.
- **3-Way Color Wheels**: Independent chromaticity control over Lift (Shadows), Gamma (Midtones), and Gain (Highlights).
- **5-Point RGB Tone Curves**: Master, Red, Green, and Blue curves evaluated in real-time GLSL/WGSL shaders.
- **8-Hue HSL Secondary Qualifier**: Hue, Saturation, and Lightness qualifiers for Red, Orange, Yellow, Green, Aqua, Blue, Purple, Magenta.
- **3D .CUBE LUT Importer**: Trilinear and diagonal 3D LUT parser with 0..100% blend strength.
- **Live Scopes**: Real-time RGB Histogram and 16-Column Luma Waveform / IRE Parade Scope.

#### D. 26 Cinema Film & Log Filters (All Implemented)
`Rec.709 Log Normalization`, `Kodak Portra 400`, `CineStill 800T`, `Blade Runner Teal & Orange`, `Matrix Emerald`, `Warm Sunset`, `Cool Mist`, `Vintage 70s`, `Cyberpunk Neon`, `Noir High-Contrast`, `Golden Hour`, `Pastel Dream`, `Crimson Punch`, `Forest Pine`, `Desert Sand`, `Deep Blue`, `Vivid Pop`, `Low Sat Drama`, `Sepia Classic`, `Monochrome Silver`, `Soft Portrait`, `Acid Lime`, `Solarize Gold`, `Infra Glow`, `Amber Glow`, `Urban Steel`.

#### E. 22 GPU Video Effects (All Implemented)
`Gaussian Blur`, `Zoom Blur`, `Motion Blur`, `Sharpen`, `Bloom Pulse`, `Halation Glow`, `Chromatic Aberration`, `RGB Split`, `VHS Tape Glitch`, `CRT Scanlines`, `Neon Edge Glow`, `Posterize`, `Mirror Split`, `Duotone Gradient`, `Thermal Heatmap`, `Pixelate Mosaic`, `Film Grain`, `Camera Shake`, `Impact Flash`, `Beat Pulse Shake`, `Vignette Feather`, `Light Leak Flare`.

#### F. 14 Scene Transitions (All Implemented)
`Fade to Black`, `Cross Dissolve`, `Fade to White`, `Iris Circle`, `Linear Wipe`, `Light Leak Flash`, `Blur Dissolve`, `Zoom Blur Transition`, `Push Slide`, `Whip-Pan`, `Zoom Transition`, `Spin Rotation`, `Glitch Transition`, `Color Melt`.

#### G. AI Vision, Keying & Masks (5 Tools)
- **AI Auto-Cutout**: 100% offline neural person segmentation with bilateral feathering.
- **AI Subject Motion Tracking**: Centroid neural tracking baked into timeline keyframes.
- **Chroma Key**: Precision color picker with Similarity, Smoothness, and Spill Suppression.
- **Subject Matte Isolation**: Luma Key, Saturation Key, Portrait Focus, and Vignette Matte.
- **6 Geometric Masks**: Rectangle, Circle, Pill, Heart, Linear Split, and Mirror Band.

#### H. 3D Typography, Animated Titles & Captions (6 Tools)
- **11 Bundled Professional Fonts**: Hanken Grotesk, Inter, Space Grotesk, Montserrat, Bebas Neue, Anton, Bangers, DM Serif Display, Pacifico, Permanent Marker, Pixelify Sans.
- **8 Styled Text Presets**: Bold Title, Lower Third, Neon Glow, Cinematic Subtitle, Retro 3D, Badge Stamp, Minimalist, Comic Punch.
- **3D Volumetric Extrusion**: True multi-layer 3D depth extrusion (0..20px) with directional bevel shading.
- **Speech-Synced Waveform Captions**: Word-by-word Karaoke subtitle highlighting.
- **.SRT Subtitle Codec**: SubRip .SRT import, export, and auto-splitting.
- **Studio Watermark Stamp**: Persistent branding and copyright overlay.

#### I. 13 Parametric Vector Shapes (All Implemented)
`Square`, `Circle`, `Triangle`, `Polygon (3..12 sides)`, `Star (3..12 points)`, `Heart`, `Parallelogram`, `Trapezoid`, `Line`, `Single Arrowhead`, `Double Arrowhead`, `Speech Bubble`, `Callout Banner`.

#### J. Speed & Time Remapping (3 Tools)
- **Constant Speed & 6 Presets**: 0.1x to 16x speed plus Montage, Hero, Bullet, Jump Cut, Flash In, Flash Out.
- **5-Point Custom Speed Curve**: Interactive bezier speed ramping editor.
- **Reverse & Freeze Frame**: Intra-frame video reverse transcoder and instant freeze frame insertion.

#### K. Professional Audio Studio (3 Tools)
- **17 Audio DSP Effects**: Bass Boost, Treble Boost, Noise Gate, Vocal Isolator, Studio Compressor, Pitch Shift, Chipmunk, Deep Voice, Reverb, Echo / Delay, Chorus, Flanger, Phaser, Overdrive Distortion, 8-Bit Bitcrush, Vintage Telephone, AM Radio, Vinyl Warmth, Studio Room.
- **5-Band Parametric Graphic EQ & Limiter**: 60Hz, 230Hz, 910Hz, 3.6kHz, 12kHz biquads with -0.3 dBFS brickwall peak limiting.
- **Voiceover, Ducking & Silence Remover**: Live mic capture, automatic background ducking, and silence truncation.

#### L. Capture, Proxy & Master Export (3 Tools)
- **Studio Camera Capture**: Direct video and photo capture into the media bin.
- **Hardware Proxy & .ZIP Archives**: 540p proxy workflow and lossless .ZIP project archive backups.
- **5-Mode Master Export Engine**: MP4 (H.264/HEVC up to 4K 60fps), Batch 3-Aspect Export (9:16 + 1:1 + 16:9), Animated GIF, Audio-Only M4A, PNG Still & Cover Thumbnail.

---

### 5. Professional Windows User Experience

- **Professional Dark Interface**: Clean dark theme (`#121214` workspace, `#1a1a1e` panels, `#26262b` cards).
- **Responsive Dockable Workspace**: 4-panel layout (Media Bin, Preview Monitor, Inspector, Multi-track Timeline) with 1-click **Reset Workspace Layout** via `View` menu and header switcher.
- **Direct Clip Addition**: Hover `+` (Add to Timeline) on media cards, `+ Add to Timeline` in preview monitor chip, automatic source preview dismissal upon placement.
- **Responsive Inspector & Forms**: Adaptive badges, icon-based alignment controls, and scrolling tab strips ensuring zero text truncation on narrow widths.
- **Comprehensive Keymap**: Intuitive keyboard shortcuts (Space, J/K/L, Split, Ripple Delete, Zoom In/Out, Undo/Redo).

---

### 6. Performance & Hardware Adaptation

The **Hardware Capability Auditor** probes host environment capabilities at startup:
- CPU Architecture and SIMD support (SSE4.2, AVX, AVX2, FMA).
- System RAM and VRAM availability.
- GPU feature levels (Direct3D 12 Feature Level 12_1 / Vulkan 1.3).
- Hardware encoder availability (H.264, HEVC, AV1).
- DirectML and ONNX Runtime execution provider routing.

**Graceful Degradation Hierarchy**:
1. Dedicated GPU / Direct3D 12 hardware surface swapchain.
2. Integrated GPU acceleration.
3. Compatible secondary graphics API (Vulkan / OpenGL ES).
4. Highly optimized CPU / SIMD software fallback (Warp rasterizer + XNNPACK).

---

### 7. Media Engine Reliability

- **Frame-Accurate Synchronization**: Strict PTS timestamp alignment between audio sinks and video frames.
- **Variable Frame Rate Normalization**: Graceful handling of smartphone VFR footage.
- **Atomic Project Persistence**: Saves write to temporary manifests before swapping, preventing corruptions.
- **Auto-Recovery**: Recovers unsaved edits from transaction-safe `.bak` snapshots.
- **Crash-Resilient Background Jobs**: Work queue survives unexpected interruptions.

---

### 8. Licensing and Distribution

- **Clean-room LGPL Compliance**: Full compliance with LGPL v2.1/v3.0. Zero GPL contamination.
- **Unencumbered Codec Architecture**: Uses pre-licensed hardware encoders/decoders provided by Windows/GPU vendors.
- **100% Offline & Private**: Zero remote telemetry, zero analytics tracking, zero mandatory accounts.

---

### 9. Automated Quality Assurance

Rigorous test suites cover all 80 tools and 10 engine domains:
- Timeline manipulation and ripple editing.
- 80-step undo/redo state restoration.
- Color matrix and GLSL shader math verification.
- Parametric EQ biquad frequency response.
- Deterministic Unified Render Graph DAG execution.
- MP4 and HEVC export container validation.

---

### 10. Production Acceptance Criteria (10-Point Standard)

Every feature in FrameCut Professional meets:
1. **Real Native Implementation**: Zero mock stubs or placeholder buttons.
2. **Interactive UI Integration**: Connected to live sliders, wheels, curves, and inspector panels.
3. **Engine Binding**: Directly drives the unified compositor and export pipeline.
4. **Real-Time Preview**: Frame-accurate, hardware-accelerated playback at 60 fps.
5. **Exact Export Parity**: Identical mathematical color, transform, and audio output in final render.
6. **Transaction-Safe Persistence**: Stored atomically in project manifests (`framecut.json`).
7. **80-Step Undo/Redo**: Full state snapshotting for every parameter and timeline adjustment.
8. **Automated Verification**: Covered by automated unit and integration test suites.
9. **Hardware Probing**: Real-time detection of Direct3D 12, NVENC/QSV, DirectML, and CPU instruction sets.
10. **Graceful Degradation**: Multi-tier software/CPU fallbacks when hardware capabilities are absent.

---

### 11. Execution Roadmap & Phase Sign-Off

- [x] **PHASE 1 — Repository & Dependency Audit**: LGPL compliance, FFmpeg 8.1, ONNX Runtime 1.28.0.
- [x] **PHASE 2 — Native Windows Foundation**: Direct3D 12 render graph, atomic project ledger, standalone distribution staging.
- [x] **PHASE 3 — Timeline & Multi-Track Editing**: 10 timeline tools, 80-step undo/redo, sub-pixel trimming, magnetic ripple.
- [x] **PHASE 4 — Color Grading & Visual Shaders**: 26 cinema filters, 22 GPU effects, 14 transitions, 3-way color wheels, 5-point tone curves.
- [x] **PHASE 5 — Audio Studio**: 5-band parametric EQ, -0.3 dBFS brickwall peak limiter, 17 audio DSP effects, voiceover ducking.
- [x] **PHASE 6 — AI Neural Studio**: Offline person auto-cutout, planar motion tracking, ONNX neural video engine.
- [x] **PHASE 7 — Master Export & Capture**: 5-mode master export, 4K 60fps MP4/HEVC, batch 9:16/1:1/16:9, proxy generation.
- [x] **PHASE 8 — Automated Verification**: 31 automated test suites passing with zero errors.
- [x] **PHASE 9 — Windows Release Packaging**: Inno Setup (`framecut.iss`), WiX (`framecut.wxs`), standalone portable folder, Git repository synchronized.

---

### 12. Final Deliverables & Absolute Development Rules

- Native Windows Desktop application (`framecut.exe`).
- Production standalone distribution directory (`dist/FrameCut Studio`).
- Zero mockup, zero placeholder, zero cloud dependency policy enforced.
- Enterprise-grade reliability engineered for creator workflows.

---

**Engineered & Published by IT AMADERSOMAJ INC.**  
© 2026 IT AMADERSOMAJ INC. All rights reserved.
