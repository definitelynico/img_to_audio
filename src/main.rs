use macroquad::{input, miniquad::conf, prelude::*};

fn conf() -> conf::Conf {
    conf::Conf {
        window_title: "audio to image".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let texture = load_texture("images/test_img.png").await.unwrap();

    let mut playhead_pos = 0.0;
    let playback_speed = 100.0; // Fix speed to scale with image width!

    let mut is_playing = false;

    loop {
        let delta = get_frame_time();

        let window_height = screen_height();
        let window_width = screen_width();

        if is_playing {
            if playhead_pos < window_width {
                playhead_pos += delta * playback_speed;
            } else {
                playhead_pos = 0.0;
            }
        }

        if input::is_key_pressed(KeyCode::Space) {
            is_playing = !is_playing;
        }

        clear_background(GRAY);

        // draw_texture_ex(
        //     &texture,
        //     0.0,
        //     0.0,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(window_width, window_height)),
        //         ..Default::default()
        //     },
        // );

        draw_rectangle(
            20.0,
            20.0,
            window_height - 40.0,
            window_height - 40.0,
            GREEN,
        );

        // Draw line from top to bottom
        draw_line(playhead_pos, 0.0, playhead_pos, window_height, 4.0, RED);

        debug_info(window_width, window_height, playhead_pos);

        next_frame().await;

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
