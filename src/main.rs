use macroquad::prelude::*;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;
const SHIP_ACC: f32 = 0.33;

/// Important note: speed rotation in degrees.
const SHIP_ROT_SPEED: f32 = 5.;

struct Ship {
    pos: Vec2,
    rot: f32,
    vel: Vec2,
}

#[macroquad::main("Rust Asteroids")]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_text("Hello, macroquad!", 0.0, 45.0, 70.0, WHITE);
        next_frame().await;
    }
}

/// Returns a Vec2 from a rotation in radians.
fn vec_from_rad(rad: f32) -> Vec2 {
    return Vec2::new(rad.sin(), -rad.cos());
} 