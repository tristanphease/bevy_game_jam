use bevy::prelude::*;


pub struct Wall;


pub fn load_map(
	mut commands: Commands,
	assets: Res<AssetServer>,
) {
	let scene = assets.load("walls.glb#Scene0");

	// for wall in scene.world.components() {
	// 	println!("component: {wall:?}");
	// }

	// for wall in scene.world.entities() {
	// 	println!("entity: {wall:?}")
	// }

	commands.spawn_scene(scene);
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