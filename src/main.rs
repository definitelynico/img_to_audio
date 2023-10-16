use macroquad::{input, miniquad::conf, prelude::*};
use rodio::{self, Source};

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
    let mut tex_params: DrawTextureParams = DrawTextureParams::default();

    // Image stuff
    let test_img = load_image("images/partitur.png").await.unwrap();
    let texture_test = Texture2D::from_image(&test_img);
    let test_img_data = test_img.get_image_data();
    let static_brightness_data: &'static [f32] = static_calculate_brightness(test_img_data).await;

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let static_sample_buffer =
        rodio::static_buffer::StaticSamplesBuffer::new(2, 44100, static_brightness_data);
    let buffer_duration = static_sample_buffer.total_duration().unwrap();
    println!("buffer_duration: {:?}", buffer_duration);

    let mut playhead_pos = 0.0;

    ////////////////////// TESTING //////////////////////
    let buffer_duration_secs = buffer_duration.as_secs_f32(); // Convert to seconds
    let playback_speed = screen_width() / buffer_duration_secs;
    /////////////////////////////////////////////////////

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
            let increment = playback_speed * delta;

            if playhead_pos < window_width {
                playhead_pos += increment;
            } else {
                is_playing = false;
                playhead_pos = 0.0;
            }
        }

        if input::is_key_pressed(KeyCode::Space) {
            is_playing = !is_playing;

            stream_handle
                .play_raw(static_sample_buffer.clone().convert_samples())
                .unwrap();
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

async fn static_calculate_brightness(data: &[[u8; 4]]) -> &'static [f32] {
    let mut brightness_data = Vec::new();

    for pixel in data {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        brightness_data.push((r + g + b) / 3.0 / 255.0);
    }

    // Convert the Vec<f32> into a static slice
    let static_slice: &'static [f32] = Box::leak(brightness_data.into_boxed_slice());

    static_slice
}
