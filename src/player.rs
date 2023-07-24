use crate::assets::*;

#[derive(Clone)]
pub struct Player {
    pub position: Vector2,
    pub size: Vector2,
    pub velocity: Vector2,
    jumps: u32,
    pub max_jumps: u32,
}

impl Player {
    pub fn new(position: Vector2, size: Vector2) -> Self {
        Self {
            position,
            size,
            velocity: Vector2::zero(),
            jumps: 2,
            max_jumps: 2,
        }
    }

    pub fn collides(&self, level: &mut crate::level::Level) -> bool {
        let player_rect = rrect(
            self.position.x + 0.5,
            self.position.y + 0.5,
            self.size.x - 1.0,
            self.size.y - 1.0,
        );
        for platform in level.platforms() {
            if platform.check_collision_recs(&player_rect) {
                return true;
            }
        }
        self.position.x < 0.0
            || self.position.y < 0.0
            || self.position.x + self.size.x >= 800.0
            || self.position.y + self.size.y >= 800.0
    }

    pub fn collidable_move(
        &mut self,
        rl: &mut RaylibHandle,
        level: &mut crate::level::Level,
        direction: Vector2,
    ) {
        let motion = self.velocity * direction;
        self.position += motion * rl.get_frame_time();
        if self.collides(level) {
            loop {
                self.position -=
                    rvec2(self.velocity.x.signum(), self.velocity.y.signum()) * direction * 0.5;
                if !self.collides(level) {
                    break;
                }
            }

            if direction.x != 0.0 {
                self.velocity.x = 0.0;
            }
            if direction.y != 0.0 {
                self.velocity.y = 0.0;
                if direction.y > 0.0 {
                    self.jumps = self.max_jumps;
                }
            }
        }
    }

    pub fn update(
        &mut self,
        rl: &mut RaylibHandle,
        assets: &mut Assets,
        level: &mut crate::level::Level,
    ) {
        // * Jump
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && self.jumps > 0 {
            self.jumps -= 1;
            self.velocity.y = -600.0;
            assets.audio.play_sound(&assets.jump_sound);
        }

        if rl.is_key_released(KeyboardKey::KEY_SPACE) && self.velocity.y < 0.0 {
            self.velocity.y *= 0.7;
        }

        // Gravity
        self.velocity.y += 2000.0 * rl.get_frame_time();

        // * Integration
        let target_velocity = (rl.is_key_down(KeyboardKey::KEY_D) as i32
            - rl.is_key_down(KeyboardKey::KEY_A) as i32) as f32
            * self.size.x
            * 10.0;

        self.velocity.x +=
            (target_velocity - self.velocity.x) * (1.0 - 0.5_f32.powf(rl.get_frame_time() / 0.1));

        while self.collides(level) {
            self.position.y -= 0.5;
        }

        self.collidable_move(rl, level, rvec2(1, 0));
        self.collidable_move(rl, level, rvec2(0, 1));

        self.check_token_collisions(assets, level);
    }

    fn check_token_collisions(&self, assets: &mut Assets, level: &mut crate::level::Level) {
        let player_rect = rrect(
            self.position.x + 0.5,
            self.position.y + 0.5,
            self.size.x - 1.0,
            self.size.y - 1.0,
        );
        for token in &mut level.tokens {
            if token.rect().check_collision_recs(&player_rect) {
                token.collect(assets, level.cursor_target_position);
            }
        }
    }

    pub fn draw<D: RaylibDraw>(&self, d: &mut D) {
        d.draw_rectangle_v(self.position, self.size, Color::GRAY);
    }

    pub fn jumping(&self) -> bool {
        self.jumps < self.max_jumps
    }
}
