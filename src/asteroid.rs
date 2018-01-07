use three;
use three::Object;

use rand;
use rand::Rng;

use euler::{Quat, Vec2};
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Asteroid {
	pos: Vec2,
	vel: Vec2,
	group: three::Group,
}

impl AsRef<three::object::Base> for Asteroid {
	fn as_ref(&self) -> &three::object::Base {
		self.group.as_ref()
	}
}

impl AsMut<three::object::Base> for Asteroid {
	fn as_mut(&mut self) -> &mut three::object::Base {
		self.group.as_mut()
	}
}

impl Object for Asteroid {}

impl Asteroid {
	pub fn new(factory: &mut three::Factory, pos: Vec2, size: u8) -> Self {
		let mut rng = rand::thread_rng();
		let radius = size as f32 * 0.2;
		let geometry = three::Geometry::cylinder(radius, radius, 0.01, 32);
		let material = three::material::Wireframe { color: rng.next_u32() };
		let mesh = factory.mesh(geometry, material);
		let group = factory.group();
		mesh.set_orientation(Quat::euler(vec3!(PI / 2.0, 0.0, 0.0)));
		group.add(&mesh);
		let vel_x = rng.gen_range(0.05, 0.5);
		let vel_y = rng.gen_range(0.05, 0.5);
		Self {
			pos,
			vel: vec2!(vel_x, vel_y),
			group,
		}
	}

	pub fn update(&mut self, input: &three::Input) {
		self.pos += self.vel * input.delta_time();
		self.set_position(vec3!(self.pos, 0.0));
	}
}
