use three;
use three::Object;

use rand;
use rand::Rng;

use euler::{Quat, Vec2};
use std::f32::consts::PI;

use world_to_screen;

#[derive(Clone)]
pub struct Asteroid {
	pos: Vec2,
	vel: Vec2,
	size: u8,
	hitpoints: u8,
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
		let radius = size as f32 * 0.1;
		let geometry = three::Geometry::cylinder(radius, radius, 0.01, 32);
		let material = three::material::Wireframe { color: rng.next_u32() };
		let mesh = factory.mesh(geometry, material);
		let group = factory.group();
		mesh.set_orientation(Quat::euler(vec3!(PI / 2.0, 0.0, 0.0)));
		group.add(&mesh);
		let vel_x = rng.gen_range(0.01, 0.2);
		let vel_y = rng.gen_range(0.01, 0.2);
		Self {
			pos,
			vel: vec2!(vel_x, vel_y),
			size,
			hitpoints: size + rng.gen_range(0, 2),
			group,
		}
	}

	pub fn update(&mut self, window: &three::Window) {
		self.pos += self.vel * window.input.delta_time();

		// Check window borders
		let window_size = window.size();
		let mut screen_pos = world_to_screen(self.pos, window_size.into());
		if screen_pos.x < 0.0 {
			screen_pos.x += window_size.x;
		} else if screen_pos.x > window_size.x {
			screen_pos.x -= window_size.x;
		}
		if screen_pos.y < 0.0 {
			screen_pos.y += window_size.y;
		} else if screen_pos.y > window_size.y {
			screen_pos.y -= window_size.y;
		}
		self.pos = window.renderer.map_to_ndc(screen_pos).into();

		self.set_position(vec3!(self.pos, 0.0));
	}

	pub fn apply_damage(&mut self, factory: &mut three::Factory) -> (u32, Vec<Asteroid>) {
		self.hitpoints -= 1;
		let mut score = 0;
		if self.hitpoints <= 0 {
			score += self.size as u32 * 100;
			let number_of_debris = rand::thread_rng().gen_range(0, 2);
			let vec = (0..number_of_debris).map(|_| Asteroid::new(factory, self.pos, self.size - 1)).collect::<Vec<_>>();
			(score, vec)
		} else {
			(0, vec![])
		}
	}
}
