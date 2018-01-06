extern crate three;
#[macro_use]
extern crate euler;

use three::Object;
use three::object::Base;

struct Ship {
	pos: euler::Vec2,
	rotation: f32,
	speed: euler::Vec2,
	group: three::Group,
}

impl AsRef<Base> for Ship {
	fn as_ref(&self) -> &Base {
		self.group.as_ref()
	}
}

impl AsMut<Base> for Ship {
	fn as_mut(&mut self) -> &mut Base {
		self.group.as_mut()
	}
}

impl Object for Ship {}

fn main() {
    let mut window = three::Window::new("Asteroids");
    let camera = window.factory.orthographic_camera([0.0, 0.0], 1.0, 1.0 .. 10.0);

    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
    	window.render(&camera);
    }
}
