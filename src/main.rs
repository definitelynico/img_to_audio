use macroquad::{input, miniquad::conf, prelude::*};
use rodio::{self, buffer};

fn conf() -> conf::Conf {
    conf::Conf {
        window_title: "audio to image".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let texture: Texture2D = load_texture("images/test_img.png").await.unwrap();
    let mut tex_params: DrawTextureParams = DrawTextureParams::default();

    // Image stuff
    let test_img = load_image("images/test_img.png").await.unwrap();
    let texture_test = Texture2D::from_image(&test_img);
    let test_img_data = test_img.get_image_data();
    let brightness_data = calculate_brightness(test_img_data);

    let freq_data = calculate_frequencies(&brightness_data);
    // println!("b_data: {:?}", freq_data);

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let asd = buffer::SamplesBuffer::new(2, 44100, brightness_data);

    stream_handle.play_raw(asd).unwrap();

    let mut playhead_pos = 0.0;
    let playback_speed = 10.0; // Fix speed to scale with image width!

    let mut is_playing = false;

    loop {
        let delta = get_frame_time();

        let window_height = screen_height();
        let window_width = screen_width();

        tex_params.dest_size = Some(Vec2 {
            x: window_width,
            y: window_height,
        });

        if is_playing {
            if playhead_pos < window_width {
                playhead_pos += window_width * 0.01 * playback_speed * delta;
            } else {
                playhead_pos = 0.0;
            }
        }

        if input::is_key_pressed(KeyCode::Space) {
            is_playing = !is_playing;
        }

        clear_background(GRAY);

        draw_texture_ex(&texture_test, 0.0, 0.0, WHITE, tex_params.to_owned());

        // Playhead
        draw_line(playhead_pos, 0.0, playhead_pos, window_height, 4.0, RED);

        // Debug info stuff
        debug_info(window_width, window_height, playhead_pos);

        next_frame().await;

        // Exit the app
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
    }
}

fn debug_info(window_width: f32, window_height: f32, playhead_pos: f32) {
    let fps_str: String = get_fps().to_string();
    // let delta_str = format!("{}", delta);
    draw_text(&fps_str, 20.0, 20.0, 36.0, LIME);

    let window_status = format!(
        "window width: {} window height: {}",
        window_width, window_height
    );
    draw_text(&window_status, 20.0, 40.0, 24.0, RED);

    let pp = format!("playhead pos: {}", playhead_pos);
    draw_text(&pp, 20.0, 60.0, 24.0, RED);
}

fn calculate_brightness(data: &[[u8; 4]]) -> Vec<f32> {
    let mut brightness_data = Vec::new();

    for pixel in data {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        // Calculate average brightness (ignoring alpha)
        // Scale to [0.0, 1.0]

        // Convert to u8 and append to the result
        brightness_data.push((r + g + b) / 3.0 / 255.0);
    }

    brightness_data
}

fn calculate_frequencies(brightness_data: &[f32]) -> Vec<f32> {
    brightness_data
        .iter()
        .map(|&brightness| {
            // Map brightness to a reasonable frequency range (e.g., 200 Hz to 2000 Hz)
            let min_frequency = 20.0;
            let max_frequency = 20000.0;

            min_frequency + brightness * (max_frequency - min_frequency)
        })
        .collect()
}
