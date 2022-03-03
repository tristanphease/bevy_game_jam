use bevy::prelude::*;


//copied from three.js lol
pub struct Spherical {
	pub radius: f32,
	pub theta: f32,
	pub phi: f32,
}

impl Spherical {
	pub fn from_vec3(vector: Vec3) -> Spherical {
		Self::from_xyz(vector.x, vector.y, vector.z)
	}

	pub fn from_xyz(x: f32, y: f32, z: f32) -> Spherical {
		let radius = f32::sqrt(x * x + y * y + z * z);

		if radius == 0.0 {
			Spherical {
				radius,
				theta: 0.0,
				phi: 0.0,
			}
		} else {
			Spherical {
				radius,
				theta: f32::atan2(x, z),
				phi: f32::acos(f32::clamp(y / radius, -1.0, 1.0)),
			}
		}
	}

	pub fn to_vec3(&self) -> Vec3 {
		let sin_phi_radius = f32::sin(self.phi) * self.radius;

		let x = sin_phi_radius * f32::sin(self.theta);
		let y = f32::cos(self.phi) * self.radius;
		let z = sin_phi_radius * f32::cos(self.theta);

		Vec3::new(x, y, z)
	}
}