extern crate three;
#[macro_use]
extern crate euler;

use three::Object;

fn main() {
    let mut window = three::Window::new("Asteroids");
    let camera = window.factory.orthographic_camera([0.0, 0.0], 1.0, 1.0 .. 10.0);

    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
    	window.render(&camera);
    }
}
