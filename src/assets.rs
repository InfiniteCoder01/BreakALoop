pub use anyhow::*;
pub use raylib::misc::get_random_value;
pub use raylib::prelude::*;

pub fn frandom(min: f32, max: f32) -> f32 {
    get_random_value::<i32>((min * 100.0) as i32, (max * 100.0) as i32) as f32 / 100.0
}

pub struct Assets {
    pub title_screen_texture: Texture2D,
    pub play_button_texture: Texture2D,

    pub audio: RaylibAudio,
    pub play_button_hover_sound: Sound,
    pub play_button_click_sound: Sound,
    pub jump_sound: Sound,
    pub restart_level_sound: Sound,
    pub next_level_sound: Sound,
    pub pick_token_sound: Sound,
    pub token_placed_sound: Sound,
}

impl Assets {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self> {
        Ok(Self {
            title_screen_texture: rl
                .load_texture(thread, "Assets/TitleScreen.png")
                .map_err(|err| anyhow!(err))?,
            play_button_texture: rl
                .load_texture(thread, "Assets/PlayButton.png")
                .map_err(|err| anyhow!(err))?,

            audio: RaylibAudio::init_audio_device(),
            play_button_hover_sound: Sound::load_sound("Assets/PlayButtonHover.wav")
                .map_err(|err| anyhow!(err))?,
            play_button_click_sound: Sound::load_sound("Assets/PlayButtonClick.wav")
                .map_err(|err| anyhow!(err))?,
            jump_sound: Sound::load_sound("Assets/Jump.wav").map_err(|err| anyhow!(err))?,
            restart_level_sound: Sound::load_sound("Assets/RestartLevel.wav")
                .map_err(|err| anyhow!(err))?,
            next_level_sound: Sound::load_sound("Assets/NextLevel.wav")
                .map_err(|err| anyhow!(err))?,
            pick_token_sound: Sound::load_sound("Assets/PickupToken.wav")
                .map_err(|err| anyhow!(err))?,
            token_placed_sound: Sound::load_sound("Assets/TokenPlaced.wav")
                .map_err(|err| anyhow!(err))?,
        })
    }
}
