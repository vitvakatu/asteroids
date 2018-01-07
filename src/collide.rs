use euler::Vec2;

pub fn collide(pos1: Vec2, r1: f32, pos2: Vec2, r2: f32) -> bool {
	let distance = (pos1 - pos2).squared_length();
	let radiuses = (r1 + r2).powf(2.0);
	distance <= radiuses
}
