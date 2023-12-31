#![windows_subsystem = "windows"]

use macroquad::{input, miniquad::conf, prelude::*};
use rodio::{self, static_buffer::StaticSamplesBuffer, Sink, Source};

fn conf() -> conf::Conf {
    conf::Conf {
        window_title: "ynok - img2audio".to_owned(),
        window_width: 600,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut show_debug_info: bool = false;
    let mut tex_params: DrawTextureParams = DrawTextureParams::default();

    // Image stuff
    let mut test_img = image_load_dialog().await;
    let mut texture_test = Texture2D::from_image(&test_img);
    let mut test_img_data = test_img.get_image_data();
    let mut static_brightness_data: &'static [f32] =
        static_calculate_brightness(test_img_data).await;

    // Audio stuff
    let sample_rate: u32 = 44100;
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let mut static_sample_buffer =
        rodio::static_buffer::StaticSamplesBuffer::new(1, sample_rate, static_brightness_data);
    // let mut filtered_audio = static_sample_buffer.buffered().high_pass(30); // Filter out very low frequencies
    let mut buffer_duration = static_sample_buffer.total_duration().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Data stuff
    let window_height = screen_height();
    let window_width = screen_width();
    let mut buffer_duration_secs = buffer_duration.as_secs_f32(); // Convert to seconds
    let mut playback_speed = window_width / buffer_duration_secs;
    let mut playhead_pos = 0.0;
    let mut is_playing = false;

    loop {
        let delta = get_frame_time();

        tex_params.dest_size = Some(Vec2 {
            x: window_width,
            y: window_height,
        });

        if is_playing {
            let increment = playback_speed * sink.speed() * delta;

            if playhead_pos < window_width {
                playhead_pos += increment;
            } else {
                is_playing = false;
                playhead_pos = 0.0;
            }
        }

        if input::is_key_pressed(KeyCode::I) {
            sink.stop();
            is_playing = false;
            playhead_pos = 0.0;

            test_img = image_load_dialog().await;
            texture_test = Texture2D::from_image(&test_img);
            test_img_data = test_img.get_image_data();
            static_brightness_data = static_calculate_brightness(test_img_data).await;
            static_sample_buffer = rodio::static_buffer::StaticSamplesBuffer::new(
                1,
                sample_rate,
                static_brightness_data,
            );
            buffer_duration = static_sample_buffer.total_duration().unwrap();
            buffer_duration_secs = buffer_duration.as_secs_f32(); // Convert to seconds
            playback_speed = window_width / buffer_duration_secs
        }

        if input::is_key_pressed(KeyCode::Space) {
            is_playing = !is_playing;

            if sink.empty() {
                playhead_pos = 0.0;
                sink.append(static_sample_buffer.clone());
            } else {
                sink.stop();
                playhead_pos = 0.0;
            }
        }

        if input::is_key_pressed(KeyCode::Right) {
            let current_speed = sink.speed();
            if current_speed < 4.0 {
                sink.set_speed(current_speed + 0.25);
            }
        }

        if input::is_key_pressed(KeyCode::Left) {
            let current_speed = sink.speed();
            if current_speed > 0.25 {
                sink.set_speed(current_speed - 0.25);
            }
        }

        if input::is_key_pressed(KeyCode::D) {
            show_debug_info = !show_debug_info;
        }

        clear_background(GRAY);

        draw_texture_ex(&texture_test, 0.0, 0.0, WHITE, tex_params.to_owned());

        // Playhead
        draw_line(playhead_pos, 0.0, playhead_pos, window_height, 4.0, RED);

        // Debug info stuff
        if show_debug_info {
            debug_info(window_width, window_height, playhead_pos, &sink);
        }

        next_frame().await;

        if input::is_key_pressed(KeyCode::S) {
            sink.stop();
            is_playing = false;
            playhead_pos = 0.0;
            save_to_file(static_sample_buffer.clone());
        }

        // Exit the app
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
    }
}

fn debug_info(window_width: f32, window_height: f32, playhead_pos: f32, sink: &Sink) {
    let font_size = 36.0;
    let fps_str: String = format!("fps: {}", get_fps());
    draw_rectangle(0.0, 0.0, 300.0, 100.0, BLACK);
    draw_text(&fps_str, 20.0, 25.0, font_size, GREEN);

    let window_status = format!("res: {}x{}", window_width, window_height);
    draw_text(&window_status, 20.0, 45.0, font_size, BLUE);

    let pp = format!("ph pos: {:.2}", playhead_pos);
    draw_text(&pp, 20.0, 65.0, font_size, PURPLE);
    let sr_str = format!("speed: {}", sink.speed());
    draw_text(&sr_str, 20.0, 85.0, font_size, PINK);
}

async fn static_calculate_brightness(data: &[[u8; 4]]) -> &'static [f32] {
    let mut brightness_data = Vec::new();

    for pixel in data {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        //Scale brightness data to be between -1.0 and 1.0
        brightness_data.push(((r + g + b) / 3.0 / 255.0) * 2.0 - 1.0);
    }

    // Convert the Vec<f32> into a static slice
    let static_slice: &'static [f32] = Box::leak(brightness_data.into_boxed_slice());

    static_slice
}

fn save_to_file(buffer: StaticSamplesBuffer<f32>) {
    let pp = match rfd::FileDialog::new()
        .add_filter("Waveform Audio File", &["wav"])
        .save_file()
    {
        Some(path) => path.to_string_lossy().to_string(),
        None => return,
    };

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // Export .wav from static sample buffer
    let mut writer = hound::WavWriter::create(pp, spec).unwrap();
    for sample in buffer.into_iter() {
        writer
            .write_sample((sample * i16::MAX as f32) as i16)
            .unwrap();
    }
}

async fn image_load_dialog() -> Image {
    let mut img = Image::empty();

    match rfd::FileDialog::new()
        .add_filter("Images", &["png"])
        .pick_file()
    {
        Some(path) => {
            let path_str = path.to_str().unwrap();
            img = load_image(path_str).await.unwrap();
        }
        None => {
            println!("No file selected");
        }
    };

    img
}
