use macroquad::prelude::*;
use bird::*;
use pipe::*;

pub mod bird;
pub mod pipe;

enum GameStates {
    Menu,
    Run,
    Dead
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_owned(),
        window_width: 768,
        window_height: 488,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = GameStates::Menu;
    let mut dead = false;
    let mut score = 0;
    let mut best_score = 0;

    let mut bird = Bird::new(
        (macroquad::window::screen_width() * 0.5) as f32,
        (macroquad::window::screen_height() * 0.5) as f32,
        25f32
    );

    let mut pipes: [Pipe; 3] = [
        Pipe::new(macroquad::window::screen_width() as f32,
                  macroquad::rand::gen_range(75.0 * 0.5, macroquad::window::screen_height() - 75.0 * 0.5), 
                  75.0, 
                  75.0, 
                  macroquad::window::screen_height()),
        Pipe::new(macroquad::window::screen_width() as f32 + 300.0,
               macroquad::rand::gen_range(75.0 * 0.5, macroquad::window::screen_height() - 75.0 * 0.5), 
                  75.0, 
                  75.0, 
                  macroquad::window::screen_height()),
        
        Pipe::new(macroquad::window::screen_width() as f32 + 600.0,
                  macroquad::rand::gen_range(75.0 * 0.5, macroquad::window::screen_height() - 75.0 * 0.5), 
                  75.0, 
                  75.0, 
                  macroquad::window::screen_height()),

    ];

    loop {
        clear_background(BLUE);

        match state {
            GameStates::Menu => {
                draw_text("Press space to start", 250.0, (macroquad::window::screen_height() - 100.0) as f32, 30.0, BLACK);
                bird.draw(dead);

                if is_key_down(KeyCode::Space) {
                    state = GameStates::Run;
                    bird.jump()
                }
            },

            GameStates::Run => {
                if !dead {
                    bird.control();
                }

                for i in 0..pipes.len() {
                    pipes[i].update();
                    pipes[i].draw();

                    if bird.rect.overlaps(&pipes[i].up) || bird.rect.overlaps(&pipes[i].down) {
                        dead = true
                    }

                    if bird.rect.overlaps(&pipes[i].gap) && !pipes[i].is_passed {
                        score += 1;
                        pipes[i].is_passed = true;
                    }
                }

                bird.update();
                bird.draw(dead);

                draw_text(&format!("Best Score: {}", best_score), 5.0, 25.0, 30.0, BLACK);
                draw_text(&format!("Score: {}", score), 5.0, 50.0, 30.0, BLACK);

                if bird.position.y > macroquad::window::screen_height() - bird.rect.h * 0.5 || bird.position.y < bird.rect.h * 0.5 {
                    dead = true;
                }

                if bird.position.y > macroquad::window::screen_height() - bird.rect.h * 0.5 && dead {
                    state = GameStates::Dead;
                }
            },

            GameStates::Dead => {
                draw_text("Press R to restart", 260.0, 100.0, 30.0, BLACK);
                draw_text(&format!("Score: {}", score), 260.0, 125.0, 30.0, BLACK);
                draw_text(&format!("Best Score: {}", best_score), 260.0, 150.0, 30.0, BLACK);

                bird.draw(dead);

                if is_key_down(KeyCode::R) {
                    if score > best_score {
                        best_score = score;
                        score = 0;
                    } else {
                        score = 0;
                    }
                    dead = false;
                    state = GameStates::Menu;
                    
                    bird = Bird::new(
                        (macroquad::window::screen_width() * 0.5) as f32,
                        (macroquad::window::screen_height() * 0.5) as f32,
                        25f32
                    );

                    pipes = [
                        Pipe::new(macroquad::window::screen_width() as f32,
                                  macroquad::rand::gen_range(75.0 * 0.5, macroquad::window::screen_height() - 75.0 * 0.5), 
                                  75.0, 
                                  75.0, 
                                  macroquad::window::screen_height()),
                        Pipe::new(macroquad::window::screen_width() as f32 + 300.0,
                                  macroquad::rand::gen_range(75.0 * 0.5, macroquad::window::screen_height() - 75.0 * 0.5), 
                                  75.0, 
                                  75.0, 
                                  macroquad::window::screen_height()),
                        
                        Pipe::new(macroquad::window::screen_width() as f32 + 600.0,
                                  macroquad::rand::gen_range(75.0 * 0.5, macroquad::window::screen_height() - 75.0 * 0.5), 
                                  75.0, 
                                  75.0, 
                                  macroquad::window::screen_height()),
                
                    ];
                }
            }

        }
        next_frame().await
    }
}