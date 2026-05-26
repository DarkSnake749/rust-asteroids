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

const BULLET_COOLDOWN: f64 = 0.15;
const BULLET_SPEED: f32 = 10.;
const BULLET_SIZE: f32 = 3.;
const BULLET_LIFE_TIME: f64 = 0.5;

const NB_ASTEROIDS: usize = 10;

struct Ship {
    pos: Vec2,
    rot: f32,
    vel: Vec2,
}

struct Bullet {
    pos: Vec2,
    shot_at: f64,
    rot: f32,
    collided: bool,
}

struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    rot: f32,
    rot_speed: f32,
    size: f32,
    sides: u8,
    collided: bool,
}

#[macroquad::main("Rust Asteroids")]
async fn main() {

    let mut ship = Ship {
            pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
            rot: 5.,
            vel: Vec2::new(0., 0.),
    };

    let mut bullets: Vec<Bullet> = Vec::new();
    let mut asteroids: Vec<Asteroid> = init_asteroids();
    let mut last_shot = get_time();

    loop {
        clear_background(BG_COLOR);
        
        ship.rot = rotate_ship(ship.rot);
        ship = move_ship(ship);
        
        (bullets, last_shot) = shoot(bullets, &ship, &last_shot);
        bullets = move_bullets(bullets);

        asteroids = move_asteroids(asteroids);

        draw_ship(&ship);
        draw_bullets(&bullets);
        draw_asteroids(&asteroids);

        next_frame().await
    }
}

/// Returns a Vec2 from a rotation in radians.
fn vec_from_rad(rad: f32) -> Vec2 {
    return Vec2::new(rad.sin(), -rad.cos());
}

fn init_asteroids() -> Vec<Asteroid> {
    let mut list: Vec<Asteroid> = Vec::new();
    let screen_center: Vec2 = Vec2::new(screen_width() / 2., screen_width() / 2.);

    for i in 0..NB_ASTEROIDS {
        list.push(Asteroid { 
            pos: screen_center + Vec2::new(
                rand::gen_range(-1., 1.), 
                rand::gen_range(-1., 1.)).normalize() * screen_width().min(screen_height()),
            vel: Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)), 
            rot: 0., 
            rot_speed: rand::gen_range(-2., 2.), 
            size: screen_width().min(screen_height()) / 10., 
            sides: rand::gen_range(3, 8),
            collided: false
        });
    }

    list
}

fn wrap_around(v: &Vec2, offset: f32) -> Vec2 {
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > screen_width() + offset {
        vr.x = 0. - offset;
    }
    if vr.x < 0. - offset {
        vr.x = screen_width() + offset;
    }
    if vr.y > screen_height() + offset {
        vr.y = 0. - offset;
    }
    if vr.y < 0. - offset {
        vr.y = screen_height() + offset;
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
    new_ship.pos = wrap_around(&new_ship.pos, SHIP_HEIGHT / 2.);
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

fn shoot(bullets: Vec<Bullet>, ship: &Ship, time: &f64) -> (Vec<Bullet>, f64) {
    let mut new_bullets = bullets;
    let mut new_time: f64 = *time;

    if is_key_down(KeyCode::Space) && get_time() - time > BULLET_COOLDOWN {
        new_time = get_time();
        new_bullets.push(Bullet {
            pos: ship.pos,
            shot_at: get_time(),
            rot: ship.rot,
            collided: false,
        });
    }

   return (new_bullets, new_time,)
}

fn move_bullets(bullets: Vec<Bullet>) -> Vec<Bullet> {
    let mut new_bullets: Vec<Bullet> = bullets;

    new_bullets.retain(|bullet| get_time() - bullet.shot_at < BULLET_LIFE_TIME);

    for bullet in new_bullets.iter_mut() {
        let vel = vec_from_rad(bullet.rot.to_radians()) * BULLET_SPEED;
        bullet.pos += vel;
        bullet.pos = wrap_around(&bullet.pos, 0.0);
    }

    new_bullets
}

fn move_asteroids(asteroids: Vec<Asteroid>) -> Vec<Asteroid> {
    let mut new_asteroids = asteroids;
    for asteroid in new_asteroids.iter_mut() {
        asteroid.pos += asteroid.vel;
        asteroid.pos = wrap_around(&asteroid.pos, asteroid.size / 2.);
        asteroid.rot += asteroid.rot_speed;
    }

    new_asteroids
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

fn draw_bullets(bullets: &Vec<Bullet>) {
    for bullet in bullets.iter() {
        draw_circle(bullet.pos.x, bullet.pos.y, BULLET_SIZE, OBJ_COLOR);
    }
}

fn draw_asteroids(asteroids: &Vec<Asteroid>) {
    for asteroid in asteroids.iter() {
        draw_poly_lines(
            asteroid.pos.x, 
            asteroid.pos.y, 
            asteroid.sides, 
            asteroid.size, 
            asteroid.rot, 
            THICKNESS, 
            OBJ_COLOR
        );
    }
}
