extern crate three;
#[macro_use]
extern crate euler;

use euler::Vec2;
use three::Object;

mod ship;
use ship::Ship;

pub fn world_to_screen(coord: Vec2, screen: Vec2) -> Vec2 {
	vec2!((1.0 + coord.x) * screen.x / 2.0, (1.0 - coord.y) * screen.y / 2.0)
}

fn main() {
    let mut window = three::Window::new("Asteroids");
    let camera = window.factory.orthographic_camera([0.0, 0.0], 1.0, 1.0 .. 100.0);
    camera.look_at([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], None);

    let mut ship = Ship::new(&mut window.factory, None);
    window.scene.add(&ship);

    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
    	ship.update(&mut window);
    	window.render(&camera);
    }
}
