
use macroquad::prelude::*;

pub const GRAVITY: f32 = 10.0;
pub const JUMP_HEIGHT: f32 = 200.0;

pub struct Bird {
    pub position: Vec2, 
    pub rect: Rect,
    pub velocity_y: f32,
}

impl Bird {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            rect: Rect::new(x  - size * 0.5, y - size * 0.5, size, size),
            velocity_y: 0.0,
        }
    }

    pub fn jump(&mut self) {
        self.velocity_y = -JUMP_HEIGHT;
    }

    pub fn control(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.jump();
        }
    }

    pub fn update(&mut self) {
        self.velocity_y += GRAVITY;
        self.position.y += self.velocity_y * get_frame_time();

        self.rect.x = self.position.x - self.rect.w * 0.5;
        self.rect.y = self.position.y - self.rect.h * 0.5;
    }

    pub fn draw(&self, dead: bool) {
        if dead {
            draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, RED);
        } else {
            draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, YELLOW);
        }
    }
}