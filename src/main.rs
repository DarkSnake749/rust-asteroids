use macroquad::prelude::*;

const BG_COLOR: Color = LIGHTGRAY;
const OBJ_COLOR: Color = BLACK;
const THICKNESS: f32 = 3.;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;
const SHIP_ACC: f32 = 0.33;
const SHIP_MAX_VEL: f32 = 5.;
/// Important note: speed rotation in degrees.
const SHIP_ROT_SPEED: f32 = 5.;
const SHIP_FRICTION: f32 = 100.;

struct Ship {
    pos: Vec2,
    rot: f32,
    vel: Vec2,
}

#[macroquad::main("Rust Asteroids")]
async fn main() {

    let mut ship = Ship {
            pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
            rot: 5.,
            vel: Vec2::new(0., 0.),
    };

    loop {
        clear_background(BG_COLOR);
        
        ship.rot = rotate_ship(ship.rot);
        ship = move_ship(ship);
        draw_ship(&ship);

        next_frame().await
    }
}

/// Returns a Vec2 from a rotation in radians.
fn vec_from_rad(rad: f32) -> Vec2 {
    return Vec2::new(rad.sin(), -rad.cos());
}

fn wrap_around(v: &Vec2) -> Vec2 {
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > screen_width() + SHIP_HEIGHT / 2. {
        vr.x = 0. - SHIP_HEIGHT / 2.;
    }
    if vr.x < 0. - SHIP_HEIGHT / 2. {
        vr.x = screen_width() + SHIP_HEIGHT / 2.;
    }
    if vr.y > screen_height() + SHIP_HEIGHT / 2. {
        vr.y = 0. - SHIP_HEIGHT / 2.;
    }
    if vr.y < 0. - SHIP_HEIGHT / 2. {
        vr.y = screen_height() + SHIP_HEIGHT / 2.;
    }
    vr
}

fn move_ship(ship: Ship) -> Ship {
    let rot_rad = ship.rot.to_radians();
    let mut new_ship = ship;

    let mut acc = -new_ship.vel / SHIP_FRICTION;

    if is_key_down(KeyCode::W) {
        acc = vec_from_rad(rot_rad) * SHIP_ACC;
    }

    new_ship.vel += acc;
    if new_ship.vel.length() > SHIP_MAX_VEL {
        new_ship.vel = new_ship.vel.normalize() * SHIP_MAX_VEL;
    }
    new_ship.pos += new_ship.vel;
    new_ship.pos = wrap_around(&new_ship.pos);
    new_ship
}

fn rotate_ship(rot: f32) -> f32 {
    let mut new_rot = rot;

    if is_key_down(KeyCode::D) {
        new_rot += SHIP_ROT_SPEED;
    } else if is_key_down(KeyCode::A) {
        new_rot -= SHIP_ROT_SPEED;
    }

    new_rot
}

fn draw_ship(ship: &Ship) {
    let rot_rad = ship.rot.to_radians();
    let rot_sin = rot_rad.sin();
    let rot_cos = rot_rad.cos();

    let v1 = Vec2::new(
        ship.pos.x + rot_sin * SHIP_HEIGHT / 2.,
        ship.pos.y - rot_cos * SHIP_HEIGHT / 2.,
    );
    let v2 = Vec2::new(
        ship.pos.x - rot_cos * SHIP_BASE / 2. - rot_sin * SHIP_HEIGHT / 2.,
        ship.pos.y - rot_sin * SHIP_BASE / 2. + rot_cos * SHIP_HEIGHT / 2.,
    );
    let v3 = Vec2::new(
        ship.pos.x + rot_cos * SHIP_BASE / 2. - rot_sin * SHIP_HEIGHT / 2.,
        ship.pos.y + rot_sin * SHIP_BASE / 2. + rot_cos * SHIP_HEIGHT / 2.,
    );

    draw_triangle_lines(v1, v2, v3, THICKNESS, OBJ_COLOR);
}
