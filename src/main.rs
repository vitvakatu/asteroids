extern crate three;
#[macro_use]
extern crate euler;

use euler::{Vec2, Vec3};

use three::Object;
use three::object::Base;

const SHIP_ROTATION_SPEED: f32 = 3.14;
const SHIP_ACCELERATION: f32 = 1.0;

struct Ship {
	pos: euler::Vec2,
	orientation: euler::Quat,
	rotation: f32,
	speed: Vec2,
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

impl Ship {
	fn new(factory: &mut three::Factory, init_pos: Option<Vec2>) -> Self {
		let vertices = vec![
			[0.0, 0.0, 0.0].into(),
			[-0.2, -0.1, 0.0].into(),
			[0.0, 0.5, 0.0].into(),
			[0.0, 0.0, 0.0].into(),
			[0.0, 0.5, 0.0].into(),
			[0.2, -0.1, 0.0].into(),
		];
		let geometry = three::Geometry::with_vertices(vertices);
		let material = three::material::Wireframe { color: 0xFFFFFF };
		let group = factory.group();
		let mesh = factory.mesh(geometry, material);
		mesh.set_scale(0.2);
		group.add(mesh);
	    group.look_at([0.0, 0.0, 0.0], [0.0, 0.0, 10.0], None);
		Self {
			pos: init_pos.unwrap_or(vec2!(0.0, 0.0)),
			rotation: 0.0,
			orientation: quat!(),
			speed: vec2!(),
			group,
		}
	}

	fn update(&mut self, factory: &mut three::Factory, input: &three::Input) {
		if input.hit(three::Key::A) {
			self.rotation -= SHIP_ROTATION_SPEED * input.delta_time();
		}
		if input.hit(three::Key::D) {
			self.rotation += SHIP_ROTATION_SPEED * input.delta_time();
		}
		let new_orientation = self.orientation * quat!(vec3!(-1.0, 0.0, 0.0); self.rotation);
		self.set_orientation(new_orientation);
	}
}

fn main() {
    let mut window = three::Window::new("Asteroids");
    let camera = window.factory.orthographic_camera([0.0, 0.0], 1.0, 1.0 .. 100.0);
    camera.look_at([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], None);

    let mut ship = Ship::new(&mut window.factory, None);
    window.scene.add(&ship);
	{
    	let mut guard = window.scene.sync_guard();
    	ship.orientation = guard.resolve(&ship).transform.orientation.into();
	}

    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
    	ship.update(&mut window.factory, &window.input);
    	window.render(&camera);
    }
}
