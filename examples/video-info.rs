/// Simple video file analyzer
///
/// Usage: cargo run --example video-info -- <video-file>
///
/// Shows:
/// - File metadata (duration, bitrate, format)
/// - Stream information (codecs, resolution, fps)
/// - Frame count estimation
/// - First frame decoding test

use ffmpeg_next as ffmpeg;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get filename from args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <video-file>", args[0]);
        eprintln!("\nExample: {} sample.mp4", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    println!("=== FFmpeg Video Analyzer ===\n");
    println!("File: {}\n", input_file);

    // Initialize FFmpeg
    ffmpeg::init()?;

    // Open input file
    let ictx = ffmpeg::format::input(&input_file)?;

    // === FILE METADATA ===
    println!("📄 FILE METADATA");
    println!("  Format: {}", ictx.format().name());
    println!("  Format (long): {}", ictx.format().description());

    let duration = ictx.duration();
    if duration > 0 {
        let secs = duration as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE);
        println!("  Duration: {:.2}s ({:.2} min)", secs, secs / 60.0);
    }

    let bitrate = ictx.bit_rate();
    if bitrate > 0 {
        println!("  Bitrate: {:.2} Mbps", bitrate as f64 / 1_000_000.0);
    }

    // === METADATA TAGS ===
    let metadata = ictx.metadata();
    if metadata.iter().count() > 0 {
        println!("\n  Tags:");
        for (key, value) in metadata.iter() {
            println!("    {}: {}", key, value);
        }
    }

    // === STREAMS INFO ===
    println!("\n📺 STREAMS ({} total)", ictx.nb_streams());

    let mut video_stream_index = None;

    for stream in ictx.streams() {
        let codec_params = stream.parameters();
        let media_type = codec_params.medium();

        println!("\n  Stream #{}", stream.index());
        println!("    Type: {:?}", media_type);
        println!("    Codec: {:?}", codec_params.id());
        println!("    Time base: {}/{}", stream.time_base().numerator(), stream.time_base().denominator());

        let fps = stream.avg_frame_rate();
        if fps.numerator() > 0 {
            println!("    FPS: {:.2}", fps.numerator() as f64 / fps.denominator() as f64);
        }

        match media_type {
            ffmpeg::media::Type::Video => {
                video_stream_index = Some(stream.index());

                // Video-specific info
                if let Ok(video) = ffmpeg::codec::context::Context::from_parameters(codec_params) {
                    let video = video.decoder().video()?;
                    println!("    Resolution: {}x{}", video.width(), video.height());
                    println!("    Pixel format: {:?}", video.format());

                    let aspect = video.aspect_ratio();
                    if aspect.numerator() > 0 {
                        println!("    Aspect ratio: {}/{} ({:.2})",
                            aspect.numerator(), aspect.denominator(),
                            aspect.numerator() as f64 / aspect.denominator() as f64);
                    }
                }
            }
            ffmpeg::media::Type::Audio => {
                if let Ok(audio) = ffmpeg::codec::context::Context::from_parameters(codec_params) {
                    let audio = audio.decoder().audio()?;
                    println!("    Sample rate: {} Hz", audio.rate());
                    println!("    Channels: {}", audio.channels());
                    println!("    Format: {:?}", audio.format());
                }
            }
            ffmpeg::media::Type::Subtitle => {
                println!("    (Subtitle stream)");
            }
            _ => {}
        }

        // Stream metadata
        let stream_meta = stream.metadata();
        if stream_meta.iter().count() > 0 {
            println!("    Metadata:");
            for (key, value) in stream_meta.iter() {
                println!("      {}: {}", key, value);
            }
        }
    }

    // === FRAME COUNT ESTIMATION ===
    if let Some(stream_idx) = video_stream_index {
        let stream = ictx.stream(stream_idx).unwrap();

        let duration = stream.duration();
        let fps = stream.avg_frame_rate();

        if duration > 0 && fps.numerator() > 0 {
            let tb = stream.time_base();
            let duration_secs = duration as f64 * tb.numerator() as f64 / tb.denominator() as f64;
            let frame_rate = fps.numerator() as f64 / fps.denominator() as f64;
            let estimated_frames = (duration_secs * frame_rate) as u64;

            println!("\n📊 FRAME INFO");
            println!("  Estimated frames: ~{}", estimated_frames);
        }
    }

    // === DECODE FIRST FRAME ===
    if let Some(stream_idx) = video_stream_index {
        println!("\n🎬 FIRST FRAME TEST");

        let input = ictx.stream(stream_idx).unwrap();
        let codec_params = input.parameters();

        let mut decoder = ffmpeg::codec::context::Context::from_parameters(codec_params)?
            .decoder()
            .video()?;

        let mut ictx = ffmpeg::format::input(&input_file)?;
        let mut frame_decoded = false;

        for (stream, packet) in ictx.packets() {
            if stream.index() == stream_idx {
                decoder.send_packet(&packet)?;

                let mut decoded = ffmpeg::util::frame::video::Video::empty();
                if decoder.receive_frame(&mut decoded).is_ok() {
                    println!("  ✓ Successfully decoded first frame!");
                    println!("    Width: {}", decoded.width());
                    println!("    Height: {}", decoded.height());
                    println!("    Format: {:?}", decoded.format());
                    println!("    PTS: {:?}", decoded.pts());
                    println!("    Plane count: {}", decoded.planes());

                    for i in 0..decoded.planes() {
                        println!("    Plane {}: stride = {}, size = {} bytes",
                            i,
                            decoded.stride(i),
                            decoded.data(i).len()
                        );
                    }

                    frame_decoded = true;
                    break;
                }
            }
        }

        if !frame_decoded {
            println!("  ✗ Failed to decode first frame");
        }
    } else {
        println!("\n⚠ No video stream found");
    }

    println!("\n✅ Analysis complete!");

    Ok(())
}
