#![windows_subsystem = "windows"]

pub mod assets;
pub mod language;
pub mod level;
pub mod player;

use assets::*;

pub enum GameState {
    Playing,
    LevelTransition {
        level: usize,
        timer: f32,
        loaded: bool,
    },
}

impl GameState {
    pub fn transition(level: usize) -> Self {
        Self::LevelTransition {
            level,
            timer: 0.0,
            loaded: false,
        }
    }
}

fn main() -> Result<()> {
    let (mut rl, thread) = raylib::init().size(800, 800).title("Break a loop!").build();

    let mut assets = Assets::load(&mut rl, &thread).context("Failed to load assets!")?;

    let (mut level, mut player) = level::Level::load(0).context("Failed to load first level!")?;
    let mut state = GameState::Playing;
    let level_transition_time = 0.5;

    {
        let mut button_scale = 1.0;
        let mut last_hovered = false;
        loop {
            if rl.window_should_close() {
                return Ok(());
            }

            let center = rvec2(rl.get_screen_width(), rl.get_screen_height() - 100) / 2.0;
            let button_position = center
                - rvec2(
                    assets.play_button_texture.width,
                    assets.play_button_texture.height,
                ) / 2.0
                    * button_scale;

            let button_hovered = rrect(
                button_position.x,
                button_position.y,
                assets.play_button_texture.width as f32 * button_scale,
                assets.play_button_texture.height as f32 * button_scale,
            )
            .check_collision_point_rec(rl.get_mouse_position());

            if button_hovered && !last_hovered {
                assets.audio.play_sound(&assets.play_button_hover_sound);
            }

            last_hovered = button_hovered;

            if button_hovered && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                assets.audio.play_sound(&assets.play_button_click_sound);
                break;
            }

            button_scale += (if button_hovered { 1.3 } else { 1.0 } - button_scale)
                * (1.0 - 0.5_f32.powf(rl.get_frame_time() / 0.3));

            let mut d = rl.begin_drawing(&thread);
            d.draw_texture(&assets.title_screen_texture, 0, 0, Color::WHITE);
            d.draw_texture_ex(
                &assets.play_button_texture,
                button_position,
                0.0,
                button_scale,
                Color::WHITE,
            )
        }
    }

    let start = std::time::Instant::now();

    fn format_time(time: std::time::Duration) -> String {
        format!(
            "{:0>2}:{:0>2}:{:0>5.2}",
            time.as_secs() / 3600,
            (time.as_secs() / 60) % 60,
            time.as_secs_f32() % 60.0
        )
    }

    loop {
        if rl.window_should_close() {
            return Ok(());
        }

        // * Levels
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            state = GameState::transition(level.index());
            assets.audio.play_sound(&assets.restart_level_sound);
        }

        if let GameState::LevelTransition {
            level: next_level,
            timer,
            loaded,
        } = &mut state
        {
            *timer += rl.get_frame_time();
            if *timer >= level_transition_time / 2.0 && !*loaded {
                rl.set_target_fps(i32::MAX as u32);
                if let Some(next_level) = level::Level::load(*next_level) {
                    (level, player) = next_level;
                } else {
                    break;
                }
            }
            if *timer >= level_transition_time {
                *timer = 0.0;
                state = GameState::Playing;
            }
        }

        // * Update
        if let GameState::Playing = state {
            if level.update(&mut rl, &mut assets, &mut player) {
                state = GameState::transition(level.index() + 1);
                assets.audio.play_sound(&assets.next_level_sound);
            }

            player.update(&mut rl, &mut assets, &mut level);
        }

        // * Render
        let screen_size = rvec2(rl.get_screen_width(), rl.get_screen_height());
        let mut d = rl.begin_drawing(&thread);
        level.draw(&mut d);
        player.draw(&mut d);

        if let GameState::LevelTransition { timer, .. } = &mut state {
            let tilt = 200.0;
            let pos = *timer / level_transition_time * (screen_size.x * 2.0 + tilt * 2.0);
            d.draw_triangle_fan(
                &[
                    rvec2(pos - screen_size.x - tilt * 2.0, screen_size.y),
                    rvec2(pos - tilt, screen_size.y),
                    rvec2(pos, 0),
                    rvec2(pos - screen_size.x - tilt, 0),
                ],
                Color::BLACK,
            );
        }

        let time = start.elapsed();
        d.draw_text(&format_time(time), 10, 10, 20, Color::BLACK)
    }

    let time = start.elapsed();
    let mut timer = 0.0;
    while !rl.window_should_close() {
        timer += rl.get_frame_time();

        fn center_text<D: RaylibDraw>(d: &mut D, text: &str, y: i32, size: i32, color: f32) {
            d.draw_text(
                text,
                400 - measure_text(text, size) / 2,
                y,
                size,
                Color::new(0, 0, 0, (color * 255.0) as u8),
            );
        }

        let mut d = rl.begin_drawing(&thread);
        let brightness = (timer / 2.0).min(1.0);
        d.draw_texture(
            &assets.title_screen_texture,
            0,
            0,
            Color::color_from_normalized(rquat(brightness, brightness, brightness, 1.0)),
        );
        center_text(
            &mut d,
            "Thans for playing!",
            200,
            50,
            (timer - 2.0).clamp(0.0, 1.0),
        );
        center_text(
            &mut d,
            &format!("Your time: {}", format_time(time)),
            260,
            50,
            (timer - 3.0).clamp(0.0, 1.0),
        );
        center_text(
            &mut d,
            "Made for Underground Game Jam #1",
            320,
            30,
            (timer - 4.0).clamp(0.0, 1.0),
        );
        center_text(
            &mut d,
            "By InfiniteCoder",
            350,
            40,
            (timer - 5.0).clamp(0.0, 1.0),
        );
    }

    Ok(())
}
