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
    let playback_speed = 10.0;

    let mut is_playing = false;

    loop {
        let delta = get_frame_time();
        let delta_str = format!("{}", delta);

        let window_height = screen_height();
        let window_width = screen_width();

        if is_playing {
            playhead_pos += playback_speed * delta;
        }

        if input::is_key_pressed(KeyCode::Space) {
            is_playing = !is_playing;
        }

        clear_background(GRAY);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(window_width, window_height)),
                ..Default::default()
            },
        );

        // Draw line from top to bottom
        draw_line(playhead_pos, 0.0, playhead_pos, window_height, 4.0, RED);

        draw_text(&delta_str, 20.0, 20.0, 20.0, LIME);

        next_frame().await
    }
}
