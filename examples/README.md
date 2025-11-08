# FFmpeg Examples

## video-info

Simple command-line tool to analyze video files and test FFmpeg functionality.

### Features

- **File metadata**: Format, duration, bitrate
- **Stream information**: Codecs, resolution, FPS, audio channels
- **Frame count estimation**
- **First frame decoding test**: Validates decoder functionality

### Usage

```powershell
# Build
cargo build --example video-info --release

# Run
.\target\release\examples\video_info.exe <video-file>
```

### Example Output

```
=== FFmpeg Video Analyzer ===

File: sample.mp4

📄 FILE METADATA
  Format: mov,mp4,m4a,3gp,3g2,mj2
  Format (long): QuickTime / MOV
  Duration: 10.00s (0.17 min)
  Bitrate: 2.50 Mbps

📺 STREAMS (2 total)

  Stream #0
    Type: Video
    Codec: H264
    Time base: 1/12800
    FPS: 30.00
    Resolution: 1920x1080
    Pixel format: Yuv420P(TvRange)

  Stream #1
    Type: Audio
    Codec: AAC
    Sample rate: 48000 Hz
    Channels: 2
    Format: Fltp

📊 FRAME INFO
  Estimated frames: ~300

🎬 FIRST FRAME TEST
  ✓ Successfully decoded first frame!
    Width: 1920
    Height: 1080
    Format: Yuv420P(TvRange)
    PTS: Some(0)
    Plane count: 3
    Plane 0: stride = 1920, size = 2073600 bytes
    Plane 1: stride = 960, size = 518400 bytes
    Plane 2: stride = 960, size = 518400 bytes

✅ Analysis complete!
```

## Supported Formats (FFmpeg 8.0)

### Video Codecs

| Codec | Decode | Encode | Description |
|-------|--------|--------|-------------|
| **H.264 (AVC)** | ✅ | ✅ | Most common, excellent compatibility |
| **H.265 (HEVC)** | ✅ | ✅ | Better compression than H.264 |
| **VP9** | ✅ | ✅ | Google's codec, WebM |
| **AV1** | ✅ | ✅ | Next-gen, best compression |
| **MPEG-4** | ✅ | ✅ | Legacy format |
| **VP8** | ✅ | ✅ | WebM, YouTube |
| **ProRes** | ✅ | ✅ | Professional editing |
| **DNxHD** | ✅ | ✅ | Professional editing |

### Container Formats

| Format | Demux | Mux | Extension | Usage |
|--------|-------|-----|-----------|-------|
| **MP4** | ✅ | ✅ | .mp4, .m4v | Most common, web/mobile |
| **MKV (Matroska)** | ✅ | ✅ | .mkv | Feature-rich container |
| **AVI** | ✅ | ✅ | .avi | Legacy Windows |
| **MOV (QuickTime)** | ✅ | ✅ | .mov | Apple ecosystem |
| **WebM** | ✅ | ✅ | .webm | Web streaming |
| **FLV** | ✅ | ✅ | .flv | Flash video (legacy) |
| **MPEG-TS** | ✅ | ✅ | .ts, .m2ts | Broadcasting |
| **OGG** | ✅ | ✅ | .ogv | Open format |

### Audio Codecs

| Codec | Decode | Encode | Description |
|-------|--------|--------|-------------|
| **AAC** | ✅ | ✅ | Standard for MP4/M4A |
| **MP3** | ✅ | ✅ | Universal compatibility |
| **Opus** | ✅ | ✅ | Best quality at low bitrates |
| **Vorbis** | ✅ | ✅ | OGG format |
| **FLAC** | ✅ | ✅ | Lossless compression |
| **WAV/PCM** | ✅ | ✅ | Uncompressed |
| **AC-3** | ✅ | ✅ | Dolby Digital |
| **DTS** | ✅ | ❌ | Surround sound |

### Image Formats

| Format | Description |
|--------|-------------|
| **JPEG** | Standard photos |
| **PNG** | Lossless, transparency |
| **BMP** | Uncompressed bitmap |
| **GIF** | Animated images |
| **TIFF** | Professional imaging |
| **WebP** | Google's format |

## Hardware Acceleration

FFmpeg 8.0 supports hardware acceleration via:

- **NVIDIA (NVENC/NVDEC)**: H.264, H.265, AV1
- **Intel Quick Sync (QSV)**: H.264, H.265, VP9
- **AMD AMF**: H.264, H.265
- **VAAPI** (Linux): Various codecs
- **VideoToolbox** (macOS): H.264, H.265

*Note: Hardware acceleration requires specific FFmpeg build configuration.*

## Build Info

This example works with FFmpeg 3.4 - 8.0, built via:

```powershell
# Install FFmpeg via vcpkg (static linking)
vcpkg install ffmpeg:x64-windows-static-md

# Build examples
cargo build --examples --release
```

The binary includes FFmpeg statically linked (~20-60 MB), no DLLs needed.
