use raylib::prelude::*;

// Inspired by this nifty assignment: http://nifty.stanford.edu/2025/wayne-music-visualizer/specification.html
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments. Usage: {} [filename] [# of groups]", &args[0]);
        std::process::exit(1);
    }
    let filename: &String = &args[1];
    let groups: usize = match args[2].parse::<usize>() {
        Ok(v) => {
            if v == 0 {
                eprintln!("Invalid digit 0. Number of groups must be more than or equal to 1.");
                std::process::exit(2);
            }
            v
        },
        Err(_) => {
            eprintln!("Invalid digit {}. Number of groups must be more than or equal to 1.", args[2]);
            std::process::exit(2);
        }
    };
    let chunks = match map_chunks(filename, &groups) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed parsing the .wav file: {}", e.to_string());
            std::process::exit(3);
        }
    };

    run_window(filename, &chunks);
    Ok(())
}

/// Maps samples of .wav file into floats in range -1.0 .= 1.0. Divides samples to #`group_count` chunks.
/// If `group_count` is bigger than sample count, it sets the `group_count` to sample count.
fn map_chunks(wav_file_name: &String, group_count: &usize) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let mut reader = hound::WavReader::open(wav_file_name)?;
    let shift = 24 - reader.spec().bits_per_sample; // We're mapping for i24, even if the wav uses 16 bit samples.

    let samples: Vec<i32> = reader.samples::<i32>()
        .map(|s| s.unwrap())
        .collect();

    let chunk_size = if samples.len()/group_count > 0 { samples.len()/group_count } else { 1 };
    let chunks: Vec<f32> = samples
        .chunks_exact(chunk_size)
        .map(|chunk|
            chunk.into_iter().max().unwrap()
        )
        .map(|&value| (value << shift) as f32 / 8_388_607.0)
        .collect();
    return Ok(chunks);
}

struct ChunkLine {
    start_pos: Vector2,
    end_pos: Vector2,
    color: Color,
}

fn run_window(filename: &String, chunks: &Vec<f32>) {
    const DEFAULT_SCREEN_WIDTH: i32 = 1200;
    const DEFAULT_SCREEN_HEIGHT: i32 = 300;
    let (mut rl, thread) = raylib::init()
        .size(DEFAULT_SCREEN_WIDTH, DEFAULT_SCREEN_HEIGHT)
        .title("music visualizer")
        .build();
    rl.set_target_fps(60);

    // This entire bit could be refactored to run on window resize to make the wave adapt to a new resolution.
    let chunk_width = DEFAULT_SCREEN_WIDTH as f32 / chunks.len() as f32;
    let chunks_as_lines: Vec<ChunkLine> = chunks
        .iter()
        .enumerate()
        .map(|(i, chunk)| {
            ChunkLine {
                start_pos: Vector2::new(0.0 + i as f32 * chunk_width, (*chunk + 1.0) * DEFAULT_SCREEN_HEIGHT as f32/2.0),
                end_pos: Vector2::new(0.0 + i as f32 * chunk_width, DEFAULT_SCREEN_HEIGHT as f32 - (*chunk + 1.0) * DEFAULT_SCREEN_HEIGHT as f32/2.0),
                color: Color::RED, // Could be calculated per index to do something like a gradient.
            }
        })
        .collect();

    let audio = raylib::core::audio::RaylibAudio::init_audio_device().unwrap();
    let music = audio.new_music(&filename.as_str()).unwrap();
    music.play_stream();
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if music.is_stream_playing() {
                music.pause_stream();
            }
            else {
                music.play_stream();
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
            music.seek_stream((music.get_time_played()-5.0).clamp(0.0, music.get_time_length()));
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            music.seek_stream((music.get_time_played()+5.0).clamp(0.0, music.get_time_length()));
        }


        let music_percent = music.get_time_played()/music.get_time_length();
        let last_chunk_to_draw = (music_percent * chunks_as_lines.len() as f32) as usize;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for (idx, chunk) in chunks_as_lines.iter().enumerate() {
            d.draw_line_ex(chunk.start_pos, chunk.end_pos, chunk_width, if idx <= last_chunk_to_draw { chunk.color } else { Color::new(50, 50, 50, 255) });
        }
        music.update_stream();
    }
}
