use ggez::graphics::{self};
use ggez::Context;

use crate::assets::*;
use crate::mbtext::*;
use crate::ggez_utility::*;

pub struct Message {
    pub text: MBText,
    pub duration: f32,
    pub elapsed: f32,
}

impl Message {
    pub fn new(text: String, duration: f32, assets: &Assets,ctx: &mut Context) -> Message {
        Message {
            text: MBText::new(text, &assets.title_font,WHITE, 64.0,ctx),
            duration: duration,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.elapsed += dt.as_millis() as f32;
    }

    pub fn draw(&self, ctx: &mut Context) {
        let text_pos = self.text.center(ctx);
        self.text.draw(text_pos,ctx);        
    }
}
