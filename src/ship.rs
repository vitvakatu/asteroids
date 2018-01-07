use euler::{Vec2, Quat};
use three;
use three::object::Base;
use three::Object;

use world_to_screen;

const SHIP_ROTATION_SPEED: f32 = 3.14;
const SHIP_ACCELERATION: f32 = 1.0;

pub struct Ship {
	pos: Vec2,
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
	pub fn new(factory: &mut three::Factory, init_pos: Option<Vec2>) -> Self {
		let vertices = vec![
			[0.0, 0.0, 0.0].into(),
			[0.0, 0.5, 0.0].into(),
			[-0.2, -0.1, 0.0].into(),
			[0.0, 0.0, 0.0].into(),
			[0.2, -0.1, 0.0].into(),
			[0.0, 0.5, 0.0].into(),
		];
		let geometry = three::Geometry::with_vertices(vertices);
		let material = three::material::Wireframe { color: 0xFFFFFF };
		let group = factory.group();
		let mesh = factory.mesh(geometry, material);
		mesh.set_scale(0.2);
		group.add(mesh);
		Self {
			pos: init_pos.unwrap_or(vec2!()),
			rotation: 0.0,
			speed: vec2!(),
			group,
		}
	}

	pub fn update(&mut self, window: &mut three::Window) {
		let input = &window.input;
		// Rotation
		if input.hit(three::Key::A) {
			self.rotation += SHIP_ROTATION_SPEED * input.delta_time();
		}
		if input.hit(three::Key::D) {
			self.rotation -= SHIP_ROTATION_SPEED * input.delta_time();
		}
		let new_orientation = Quat::axis_angle(vec3!(0.0, 0.0, 1.0), self.rotation);
		self.set_orientation(new_orientation);

		// Acceleration
		if input.hit(three::Key::W) {
			let mut dv = vec3!(0.0, 1.0, 0.0).rotate(new_orientation);
			dv = dv * SHIP_ACCELERATION;
			dv = dv * input.delta_time();
			self.speed.x += dv.x;
			self.speed.y += dv.y;
		}

		self.pos.x += self.speed.x * input.delta_time();
		self.pos.y += self.speed.y * input.delta_time();
		self.speed.x -= self.speed.x * input.delta_time();
		self.speed.y -= self.speed.y * input.delta_time();

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
		let pos = window.renderer.map_to_ndc(screen_pos);
		self.pos.x = pos.x;
		self.pos.y = pos.y;

		self.set_position([self.pos.x, self.pos.y, 0.0]);
	}
}
