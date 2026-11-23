#![allow(dead_code)]

use mq::{
    camera::{set_camera, Camera2D},
    math::{Rect, Vec2},
    text::{load_ttf_font, Font},
    texture::{self, Texture2D},
};

#[derive(Debug)]
pub enum Err {
    Mq(mq::Error),
}

impl From<mq::Error> for Err {
    fn from(err: mq::Error) -> Self {
        Err::Mq(err)
    }
}

pub fn aspect_ratio() -> f32 {
    mq::window::screen_width() / mq::window::screen_height()
}

pub fn make_and_set_camera(aspect_ratio: f32) -> Camera2D {
    let display_rect = Rect {
        x: -aspect_ratio,
        y: -1.0,
        w: aspect_ratio * 2.0,
        h: 2.0,
    };
    let mut camera = Camera2D::from_display_rect(display_rect);
    // macroquad 0.4 flips the Y axis of cameras that draw to the screen;
    // flip it back so that +Y keeps pointing down.
    camera.zoom.y = -camera.zoom.y;
    set_camera(&camera);
    camera
}

pub fn get_world_mouse_pos(camera: &Camera2D) -> Vec2 {
    camera.screen_to_world(mq::input::mouse_position().into())
}

pub struct Assets {
    pub font: Font,
    pub texture: Texture2D,
}

impl Assets {
    pub async fn load() -> Result<Self, Err> {
        let font = load_ttf_font("zgui/assets/Karla-Regular.ttf").await?;
        let texture = texture::load_texture("zgui/assets/fire.png").await?;
        Ok(Self { font, texture })
    }
}
