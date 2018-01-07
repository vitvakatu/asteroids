extern crate three;
#[macro_use]
extern crate euler;
extern crate rand;

use rand::Rng;

use euler::Vec2;
use three::Object;

mod ship;
use ship::Ship;

mod bullet;
mod asteroid;
mod collide;

pub fn world_to_screen(coord: Vec2, screen: Vec2) -> Vec2 {
	vec2!((1.0 + coord.x) * screen.x / 2.0, (1.0 - coord.y) * screen.y / 2.0)
}

fn find_place_to_spawn(aspect: f32) -> Vec2 {
	let mut rng = rand::thread_rng();
	let mut x: f32 = 0.0;
	let mut y: f32 = 0.0;
	while x.abs() < 0.5 || y.abs() < 0.5 {
		x = rng.gen_range(-1.0 / aspect, 1.0 / aspect);
		y = rng.gen_range(-1.0, 1.0);
	}
	vec2!(x, y)
}

fn main() {
    let mut window = three::Window::new("Asteroids");
    let camera = window.factory.orthographic_camera([0.0, 0.0], 1.0, 1.0 .. 100.0);
    camera.look_at([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], None);

    let mut ship = Ship::new(&mut window, None);
    window.scene.add(&ship);

    let mut asteroids = vec![];

    for _ in 0..10 {
	    let mut asteroid = asteroid::Asteroid::new(&mut window.factory, find_place_to_spawn(window.renderer.aspect_ratio()), 1);
	    window.scene.add(&asteroid);
	    asteroids.push(asteroid);
    }

    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
    	ship.update(&mut window);
    	asteroids.iter_mut().for_each(|a| a.update(&window));
    	window.render(&camera);
    }
}
