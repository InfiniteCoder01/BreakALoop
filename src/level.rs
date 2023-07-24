use crate::assets::*;
use crate::language::*;
use crate::player::Player;

#[derive(Clone)]
pub struct Level {
    index: usize,
    code: String,
    status: CompilationStatus,
    platforms: Vec<Rectangle>,
    pub tokens: Vec<Token>,
    enemies: Vec<Enemy>,

    cursor_blink_timer: f32,
    pub cursor_target_position: Option<Vector2>,
}

impl Level {
    pub fn new(
        code: String,
        platforms: Vec<Rectangle>,
        tokens: Vec<Token>,
        enemies: Vec<Enemy>,
    ) -> Self {
        Self {
            index: 0,
            code,
            status: CompilationStatus::None,
            platforms,
            tokens,
            enemies,
            cursor_blink_timer: 0.0,
            cursor_target_position: None,
        }
    }

    pub fn load(index: usize) -> Option<(Self, Player)> {
        let player_size = rvec2(60, 80);
        let levels = [
            (
                Self::new(
                    include_str!("levels/code1.c").to_owned(),
                    vec![rrect(600, 670, 200, 70)],
                    vec![
                        Token::new("shrink_player();", rvec2(700, 670)),
                        Token::new("break;", rvec2(740, 800)),
                    ],
                    vec![],
                ),
                Player::new(rvec2(10, 800.0 - player_size.y - 1.0), player_size),
            ),
            (
                Self::new(
                    include_str!("levels/code2.c").to_owned(),
                    vec![rrect(400, 660, 200, 80), rrect(600, 660, 200, 50)],
                    vec![
                        Token::new("break;", rvec2(600, 660)),
                        Token::new("break;", rvec2(700, 800)),
                    ],
                    vec![],
                ),
                Player::new(rvec2(10, 800.0 - player_size.y - 1.0), player_size),
            ),
            (
                Self::new(
                    include_str!("levels/code3.c").to_owned(),
                    vec![
                        rrect(600, 640, 200, 80),
                        rrect(300, 640, 200, 80),
                        rrect(300, 480, 100, 80),
                        rrect(300, 330, 100, 80),
                        rrect(600, 290, 200, 80),
                    ],
                    vec![
                        Token::new("shrink_player();", rvec2(400, 630)),
                        Token::new("increase_jumps();", rvec2(700, 630)),
                        Token::new("break;", rvec2(700, 290)),
                    ],
                    vec![],
                ),
                Player::new(rvec2(10, 800.0 - player_size.y - 1.0), player_size),
            ),
            (
                Self::new(
                    include_str!("levels/code4.c").to_owned(),
                    vec![
                        rrect(300, 500, 240, 80),
                        rrect(632, 290, 168, 80),
                        rrect(680, 640, 75, 110),
                        rrect(680, 750, 20, 50),
                    ],
                    vec![
                        Token::new("increase_jumps();", rvec2(420, 500)),
                        Token::new("shrink_player();", rvec2(710, 290)),
                        Token::new("break;", rvec2(750, 800)),
                    ],
                    vec![],
                ),
                Player::new(rvec2(10, 800.0 - player_size.y - 1.0), player_size),
            ),
            (
                Self::new(
                    include_str!("levels/code5.c").to_owned(),
                    vec![],
                    vec![
                        Token::new("shrink_player();", rvec2(600, 800)),
                        Token::new("break;", rvec2(750, 800)),
                    ],
                    vec![Enemy::new(rvec2(250, 800))],
                ),
                Player::new(rvec2(10, 800.0 - player_size.y - 1.0), player_size),
            ),
            (
                Self::new(
                    include_str!("levels/code6.c").to_owned(),
                    vec![rrect(390, 640, 20, 160), rrect(190, 640, 200, 70)],
                    vec![
                        Token::new("player_is_jumping()", rvec2(300, 630)),
                        Token::new("break;", rvec2(750, 800)),
                    ],
                    vec![Enemy::new(rvec2(410, 800))],
                ),
                Player::new(rvec2(10, 800.0 - player_size.y - 1.0), player_size),
            ),
            (
                Self::new(
                    include_str!("levels/code7.c").to_owned(),
                    vec![rrect(300, 640, 10, 160)],
                    vec![
                        Token::new("shrink_player();", rvec2(710, 800)),
                        Token::new("break;", rvec2(350, 800)),
                    ],
                    vec![Enemy::new(rvec2(390, 800))],
                ),
                Player::new(rvec2(10, 800.0 - player_size.y - 1.0), player_size),
            ),
            (
                Self::new(
                    include_str!("levels/code8.c").to_owned(),
                    vec![rrect(390, 600, 20, 200)],
                    vec![
                        Token::new("shrink_player();", rvec2(500, 800)),
                        Token::new("break;", rvec2(350, 800)),
                    ],
                    vec![Enemy::new(rvec2(590, 800))],
                ),
                Player::new(rvec2(180, 800.0 - player_size.y - 1.0), player_size),
            ),
            (
                Self::new(
                    include_str!("levels/code9.c").to_owned(),
                    vec![
                        rrect(390, 400, 20, 400),
                        rrect(0, 640, 100, 70),
                        rrect(410, 640, 100, 70),
                        rrect(340, 560, 50, 80),
                        rrect(750, 560, 50, 80),
                    ],
                    vec![
                        Token::new("lagB();", rvec2(250, 600)),
                        Token::new("break;", rvec2(665, 575)),
                    ],
                    vec![Enemy::new(rvec2(575, 800))],
                ),
                Player::new(rvec2(165, 800.0 - player_size.y - 1.0), player_size),
            ),
            (
                Self::new(
                    include_str!("levels/code10.c").to_owned(),
                    vec![
                        rrect(390, 0, 20, 800),
                        rrect(410, 390, 390, 20),
                        rrect(0, 640, 95, 70),
                        rrect(410, 640, 105, 70),
                        rrect(410, 220, 95, 80),
                        // rrect(410, 630, 100, 80),
                        // rrect(340, 560, 50, 80),
                        // rrect(750, 560, 50, 80),
                    ],
                    vec![
                        Token::new("lagB();", rvec2(715, 390)),
                        Token::new("lagB();", rvec2(305, 800)),
                        Token::new("break;", rvec2(710, 800)),
                    ],
                    vec![Enemy::new(rvec2(575, 800)), Enemy::new(rvec2(575, 390))],
                ),
                Player::new(rvec2(165, 800.0 - player_size.y - 1.0), player_size),
            ),
        ];

        levels.get(index).cloned().map(|mut level| {
            level.0.index = index;
            level.0.recompile();
            level
        })
    }

    // * --------------------------------------------------------------------------------- Update --------------------------------------------------------------------------------- * //
    pub fn update(
        &mut self,
        rl: &mut RaylibHandle,
        assets: &mut Assets,
        player: &mut Player,
    ) -> bool {
        self.cursor_blink_timer = (self.cursor_blink_timer + rl.get_frame_time()) % 1.0;
        let mut enemies = self.enemies.drain(..).collect::<Vec<_>>();
        for enemy in &mut enemies {
            enemy.update(rl, assets, player, self);
        }
        self.enemies = enemies;
        for i in (0..self.tokens.len()).rev() {
            let token = &mut self.tokens[i];
            token.update(rl);
            if token.finished {
                self.code = self.code.replacen('$', token.format(), 1);
                self.recompile();
                self.tokens.remove(i);
                assets.audio.play_sound(&assets.token_placed_sound);
                if player.size.x < 60.0 {
                    player.position -= rvec2(60, 80) - player.size;
                    player.size = rvec2(60, 80);
                }
                rl.set_target_fps(i32::MAX as u32);
            }
        }
        if let CompilationStatus::Success(loops) = &mut self.status {
            let api_layer = |function: &_| {
                match function {
                    "shrink_player" => {
                        player.size = rvec2(45, 60);
                    }
                    "increase_jumps" => {
                        player.max_jumps = 4;
                    }
                    "lagB" => {
                        rl.set_target_fps(10);
                    }
                    "player_is_jumping" => return player.jumping(),
                    "update_game" | "free_texture" => (),
                    _ => {
                        println!("Warning: Unimplemented function {}!", function)
                    }
                }
                false
            };

            if loops.is_empty() {
                return true;
            }
            let breaks = loops[0].iteration(api_layer);
            for _ in 0..breaks {
                loops.remove(0);
                if loops.is_empty() {
                    break;
                }
            }
        }
        false
    }

    // * ---------------------------------------------------------------------------------- Draw ---------------------------------------------------------------------------------- * //
    pub fn draw<D: RaylibDraw>(&mut self, d: &mut D) {
        d.clear_background(Color::RAYWHITE);
        for platform in &self.platforms {
            d.draw_rectangle_rec(platform, Color::new(200, 200, 200, 255));
        }
        for enemy in &self.enemies {
            enemy.draw(d);
        }
        for token in &self.tokens {
            token.draw(d);
        }

        let font_size = 20;
        let cursor_width = (font_size as f32 * 0.6) as i32;

        let mut cursor = rvec2(12, 30);
        let mut passed_cursor = false;
        for line in self.code.split('\n') {
            if !passed_cursor {
                if let Some((left, right)) = line.split_once('$') {
                    let advance = measure_text(left, font_size);
                    d.draw_text(left, cursor.x as _, cursor.y as _, font_size, Color::GRAY);
                    self.cursor_target_position = Some(cursor + rvec2(advance, font_size / 2));
                    if self.cursor_blink_timer < 0.5 {
                        d.draw_rectangle_v(
                            cursor + rvec2(advance, 0),
                            rvec2(cursor_width, font_size),
                            Color::GRAY,
                        );
                    }
                    let advance = advance + cursor_width;
                    d.draw_text(
                        &right.replace('$', ""),
                        cursor.x as i32 + advance,
                        cursor.y as _,
                        font_size,
                        Color::GRAY,
                    );
                    passed_cursor = true;
                    cursor.y += font_size as f32;
                    continue;
                }
            }
            d.draw_text(
                &line.replace('$', ""),
                cursor.x as _,
                cursor.y as _,
                font_size,
                Color::GRAY,
            );
            cursor.y += font_size as f32;
        }
        match self.status {
            CompilationStatus::Success(_) => d.draw_text(
                "Compiled successfully!",
                cursor.x as _,
                cursor.y as _,
                30,
                Color::GREEN,
            ),
            CompilationStatus::Failure => d.draw_text(
                "Compilation failed!",
                cursor.x as _,
                cursor.y as _,
                30,
                Color::RED,
            ),
            CompilationStatus::None => (),
        }
    }

    // * -------------------------------------------------------------------------------- Recompile ------------------------------------------------------------------------------- * //
    pub fn recompile(&mut self) {
        self.status = compile(&self.code);
        self.cursor_target_position = None;
    }

    pub fn platforms(&self) -> &Vec<Rectangle> {
        &self.platforms
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Clone)]
pub struct Token {
    token: &'static str,
    position: Vector2,
    font_size: i32,
    finished: bool,
    target: Option<Vector2>,
}

impl Token {
    pub fn new(token: &'static str, position: Vector2) -> Self {
        Self {
            token,
            font_size: 20,
            position,
            finished: false,
            target: None,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if let Some(target) = self.target {
            let target =
                target + rvec2(measure_text(self.token, self.font_size), self.font_size) / 2.0;
            let distance = (target - self.position).length();
            let velocity = (target - self.position) / distance * 2000.0 * rl.get_frame_time();
            self.position += if velocity.length() > distance {
                self.finished = true;
                velocity.normalized() * distance
            } else {
                velocity
            };
        }
    }

    pub fn draw<D: RaylibDraw>(&self, d: &mut D) {
        d.draw_text(
            self.token,
            self.position.x as i32 - measure_text(self.token, self.font_size) / 2,
            self.position.y as i32 - self.font_size,
            self.font_size,
            Color::RED,
        );
    }

    pub fn collect(&mut self, assets: &mut Assets, target: Option<Vector2>) {
        if self.target.is_none() {
            if let Some(target) = target {
                self.target = Some(target);
                assets.audio.play_sound(&assets.pick_token_sound)
            }
        }
    }

    pub fn token(&self) -> &'static str {
        self.token
    }

    pub fn format(&self) -> &'static str {
        match self.token {
            "if" => "if ($) $",
            token => token,
        }
    }

    pub fn rect(&self) -> Rectangle {
        let size = rvec2(measure_text(self.token, self.font_size), self.font_size);
        let tl = self.position - size * rvec2(0.5, 1);
        rrect(tl.x, tl.y, size.x, size.y)
    }

    pub fn font_size(&self) -> i32 {
        self.font_size
    }

    pub fn finished(&self) -> bool {
        self.finished
    }
}

#[derive(Clone)]
pub struct Enemy {
    position: Vector2,
    direction: f32,
    size: Vector2,
}

impl Enemy {
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
            direction: 1.0,
            size: rvec2(60, 80),
        }
    }

    fn collides(&self, level: &Level) -> bool {
        let rect = rrect(
            self.position.x + 1.0,
            self.position.y - self.size.y + 1.0,
            self.size.x - 2.0,
            self.size.y - 2.0,
        );
        if rect.x < 0.0
            || rect.x + rect.width >= 800.0
            || rect.y < 0.0
            || rect.y + rect.height >= 800.0
        {
            return true;
        } else {
            for platform in &level.platforms {
                if platform.check_collision_recs(&rect) {
                    return true;
                }
            }
        }
        false
    }

    pub fn update(
        &mut self,
        rl: &mut RaylibHandle,
        assets: &mut Assets,
        player: &Player,
        level: &mut Level,
    ) {
        if level.index >= 7 {
            self.position.x += player.velocity.x * rl.get_frame_time();
            while self.collides(level) {
                self.position.x -= player.velocity.x.signum() * 0.5;
            }
            self.position.y += player.velocity.y * rl.get_frame_time();
            while self.collides(level) {
                self.position.y -= player.velocity.y.signum() * 0.5;
            }
        } else {
            self.position.x += self.direction * 120.0 * rl.get_frame_time();
            if self.collides(level) {
                self.direction *= -1.0;
            }
        }
        let rect = rrect(
            self.position.x,
            self.position.y - self.size.y,
            self.size.x,
            self.size.y,
        );
        for token in &mut level.tokens {
            if token.rect().check_collision_recs(&rect) {
                token.collect(assets, level.cursor_target_position);
            }
        }
    }

    pub fn draw<D: RaylibDraw>(&self, d: &mut D) {
        d.draw_rectangle_v(
            self.position - self.size * rvec2(0, 1),
            self.size,
            Color::RED,
        );
    }
}
