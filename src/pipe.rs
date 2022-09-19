use macroquad::prelude::*;

pub const SPEED: f32 = 1.0;


pub struct Pipe {
    pub up: Rect,
    up_position: Vec2,
    pub gap: Rect,
    gap_position: Vec2,
    pub down: Rect,
    down_position: Vec2,
    gap_size: f32,
    pipe_width: f32,
    pipe_height: f32,
    pub is_passed: bool
}

impl Pipe {
    pub fn new(x: f32, y: f32, gap_size: f32, pipe_width: f32, pipe_height: f32) -> Self {
        Self {
            gap_position: Vec2::new(x, y),
            gap: Rect::new(x - gap_size * 0.5, y - gap_size * 0.5, gap_size, gap_size),

            up_position: Vec2::new(x, y - gap_size * 0.5 - pipe_height * 0.5),
            up: Rect::new(x - pipe_width * 0.5, y - gap_size * 0.5 - pipe_height, pipe_width, pipe_height),

            down_position: Vec2::new(x, y + gap_size * 0.5 + pipe_height * 0.5),
            down : Rect::new(x - pipe_width * 0.5, y + gap_size * 0.5, pipe_width, pipe_height),

            gap_size: gap_size,
            pipe_width: pipe_width,
            pipe_height: pipe_height,
            is_passed: false
        }
    }

    pub fn update(&mut self) {
        self.gap_position.x -= SPEED;

        if self.gap_position.x < -self.gap_size * 0.5 {
            self.gap_position.x = macroquad::window::screen_width() + self.gap_size + 25.0;
            self.is_passed = false;
        }

        self.gap.x = self.gap_position.x - self.gap_size * 0.5;
        self.gap.y = self.gap_position.y - self.gap_size * 0.5;

        self.up_position.x = self.gap_position.x;
        self.up_position.y = self.gap_position.y - self.gap_size * 0.5 - self.pipe_height * 0.5;

        self.up.x = self.gap_position.x - self.pipe_width * 0.5;
        self.up.y = self.gap_position.y - self.gap_size * 0.5 - self.pipe_height;

        self.down_position.x = self.gap_position.x;
        self.down_position.y = self.gap_position.y + self.gap_size * 0.5 + self.pipe_height * 0.5;

        self.down.x = self.gap_position.x - self.pipe_width * 0.5;
        self.down.y = self.gap_position.y + self.gap_size * 0.5;
    }

    pub fn draw(&self) {
        draw_rectangle(self.up.x, self.up.y, self.up.w, self.up.h, GREEN);
        draw_rectangle(self.down.x, self.down.y, self.down.w, self.down.h, GREEN);
    }
}