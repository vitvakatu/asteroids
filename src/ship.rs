use euler::{Vec2, Quat};
use three;
use three::object::Base;
use three::Object;

use bullet;
use world_to_screen;

const SHIP_ROTATION_SPEED: f32 = 3.14;
const SHIP_ACCELERATION: f32 = 1.0;
const SHOT_TIMEOUT: f32 = 0.3;
const BULLET_LIFETIME: f32 = 1.0;

pub struct Ship {
	pos: Vec2,
	rotation: f32,
	vel: Vec2,
	shot_timer: three::Timer,
	bullets: Vec<bullet::Bullet>,
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
	pub fn new(window: &mut three::Window, init_pos: Option<Vec2>) -> Self {
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
		let group = window.factory.group();
		let mesh = window.factory.mesh(geometry, material);
		mesh.set_scale(0.2);
		group.add(mesh);
		Self {
			pos: init_pos.unwrap_or(vec2!()),
			rotation: 0.0,
			vel: vec2!(),
			bullets: vec![],
			shot_timer: window.input.time(),
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

		let mut dv = vec3!(0.0, 1.0, 0.0).rotate(new_orientation);

		if input.hit(three::Key::Space) && self.shot_timer.get(input) > SHOT_TIMEOUT {
			let bullet = bullet::Bullet::new(&mut window.factory, input, self.pos, dv.xy().normalize());
			window.scene.add(&bullet);
			self.bullets.push(bullet);
			self.shot_timer = input.time();
		}

		// Acceleration
		if input.hit(three::Key::W) {
			dv = dv * SHIP_ACCELERATION;
			dv = dv * input.delta_time();
			self.vel += dv.xy();
		}

		self.pos += self.vel * input.delta_time();
		self.vel -= self.vel * input.delta_time();

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

		self.set_position([self.pos.x, self.pos.y, 0.0]);

		// Update bullets
		self.bullets.iter_mut().for_each(|bullet| bullet.update(input));
		self.bullets.retain(|bullet| bullet.alive_timer.get(input) < BULLET_LIFETIME);
		println!("{}", self.bullets.len());
	}
}
