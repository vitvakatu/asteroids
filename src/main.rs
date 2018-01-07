extern crate three;
#[macro_use]
extern crate euler;

use euler::*;
use std::f32::consts::PI;

use three::Object;
use three::object::Base;

const SHIP_ROTATION_SPEED: f32 = 3.14;
const SHIP_ACCELERATION: f32 = 1.0;

fn world_to_screen(coord: Vec2, screen: Vec2) -> Vec2 {
	vec2!((1.0 + coord.x) * screen.x / 2.0, (1.0 - coord.y) * screen.y / 2.0)
}

struct Ship {
	pos: Vec2,
	orientation: Quat,
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
		mesh.set_orientation(Quat::euler(vec3!(0.0, PI, 0.0)));
		group.add(mesh);
		Self {
			pos: init_pos.unwrap_or(vec2!()),
			rotation: 0.0,
			orientation: quat!(),
			speed: vec2!(),
			group,
		}
	}

	fn update(&mut self, window: &mut three::Window) {
		let input = &window.input;
		// Rotation
		if input.hit(three::Key::A) {
			self.rotation -= SHIP_ROTATION_SPEED * input.delta_time();
		}
		if input.hit(three::Key::D) {
			self.rotation += SHIP_ROTATION_SPEED * input.delta_time();
		}
		let new_orientation = Quat::axis_angle(vec3!(1.0, 0.0, 0.0), self.rotation) * self.orientation;
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

fn main() {
    let mut window = three::Window::new("Asteroids");
    let camera = window.factory.orthographic_camera([0.0, 0.0], 1.0, 1.0 .. 100.0);
    camera.look_at([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], None);

    let mut ship = Ship::new(&mut window.factory, None);
    window.scene.add(&ship);
	{
    	let mut guard = window.scene.sync_guard();
    	ship.orientation = guard.resolve_world(&ship).transform.orientation.into();
	}

    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
    	ship.update(&mut window);
    	window.render(&camera);
    }
}
