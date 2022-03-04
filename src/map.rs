use bevy::prelude::*;
use crate::state::AppState;


//walls are defined as x, y, z, x_scale, y_scale, z_scale
const NUM_WALLS: usize = 18;
const WALLS: [f32; NUM_WALLS * 6] = [
	0.0, -0.25, 0.0, 21.0, 0.5, 21.0,
	13.2, -0.25, -20.4, 11.0, 0.5, 11.0,
	-24.2, -0.25, -1.2, 11.0, 0.5, 11.0,
	-16.6, -0.25, 17.6, 11.0, 0.5, 11.0,
	12.8, -0.25, 18.3, 11.0, 0.5, 11.0,

	32.5, -0.25, 1.7, 11.0, 0.5, 11.0,
	37.6, -0.25, -17.4, 11.0, 0.5, 11.0,
	26.0, -0.25, -43.8, 11.0, 0.5, 11.0,
	-6.5, -0.25, -39.5, 11.0, 0.5, 11.0,
	-28.0, -0.25, -21.3, 11.0, 0.5, 11.0,

	-41.2, -0.25, -43.1, 11.0, 0.5, 11.0,
	-53.5, -0.25, -13.7, 11.0, 0.5, 11.0,
	-44.4, -0.25, 12.5, 11.0, 0.5, 11.0,
	-28.9, -0.25, 36.7, 11.0, 0.5, 11.0,
	0.4, -0.25, 36.7, 11.0, 0.5, 11.0,

	32.9, -0.25, 33.5, 11.0, 0.5, 11.0,
	4.9, 1.2, -29.5, 4.2, 0.5, 4.2,
	-16.8, 1.2, -30.1, 4.2, 0.5, 4.2,
];

#[derive(Component)]
pub struct Collision;


pub fn on_load_map(
	mut app_state: ResMut<State<AppState>>,
) {
	app_state.set(AppState::Playing).unwrap();
}

pub fn add_map(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let wall_mesh = meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));
	let wall_material = materials.add(StandardMaterial {
		base_color: Color::rgb(1.0, 1.0, 1.0),
		..StandardMaterial::default()
	});
	for wall in WALLS.chunks(6) {

		let transform = Transform::from_xyz(wall[0], wall[1], wall[2])
			.with_scale(Vec3::new(wall[3], wall[4], wall[5]));

		commands.spawn_bundle(PbrBundle {
			mesh: wall_mesh.clone(),
			material: wall_material.clone(),
			transform,
			..PbrBundle::default()
		})
		.insert(Collision);
	}
}

pub fn add_light(
	mut commands: Commands,
) {
	commands.insert_resource(AmbientLight {
		color: Color::Rgba {
			red: 1.0,
			green: 1.0,
			blue: 1.0,
			alpha: 1.0,
		},
		brightness: 1.0,
	});
}