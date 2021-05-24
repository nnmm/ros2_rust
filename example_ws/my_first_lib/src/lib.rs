use nalgebra::Vector3;

pub type Vec3 = Vector3<f64>;

pub fn make_vec(x: f64, y: f64, z: f64) -> Vec3 {
	Vector3::new(x, y, z)
}

pub fn bump_up(mut v: Vec3) -> Vec3 {
	v.z += 1.0;
	v
}