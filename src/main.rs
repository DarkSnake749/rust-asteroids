use macroquad::prelude::*;

#[macroquad::main("Rust Asteroids")]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_text("Hello, macroquad!", 0.0, 45.0, 70.0, WHITE);
        next_frame().await;
    }
}