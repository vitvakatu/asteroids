use euler::{Vec2, Quat};
use three;
use three::Object;

use std::f32::consts::PI;

const BULLET_SPEED: f32 = 5.0;

#[derive(Clone)]
pub struct Bullet {
	pos: Vec2,
	vel: Vec2,
	pub alive_timer: three::Timer,
	group: three::Group,
}

impl AsRef<three::object::Base> for Bullet {
	fn as_ref(&self) -> &three::object::Base {
		self.group.as_ref()
	}
}

impl AsMut<three::object::Base> for Bullet {
	fn as_mut(&mut self) -> &mut three::object::Base {
		self.group.as_mut()
	}
}

impl Object for Bullet {}

impl Bullet {
	pub fn new(factory: &mut three::Factory, input: &three::Input, pos: Vec2, vel: Vec2) -> Self {
		let geometry = three::Geometry::cylinder(0.02, 0.02, 0.01, 12);
		let material = three::material::Wireframe { color: 0xFF0000 };
		let mesh = factory.mesh(geometry, material);
		let group = factory.group();
		mesh.set_orientation(Quat::euler(vec3!(PI / 2.0, 0.0, 0.0)));
		group.add(&mesh);
		Self {
			pos,
			vel: vel * BULLET_SPEED,
			group,
			alive_timer: input.time(),
		}
	}

	pub fn update(&mut self, input: &three::Input) {
		self.pos += self.vel * input.delta_time();
		self.set_position(vec3!(self.pos, 0.0));
	}
}
