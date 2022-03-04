use bevy::prelude::*;
use crate::{Sticky, Enemy,
	MAJOR_HEIGHT, MINOR_HEIGHT, STICK_SIZE,
	LEFT_ARM, RIGHT_ARM, LEFT_LEG, RIGHT_LEG, 
	AnimPos, AnimInfo, Head,
};
use crate::anim::PlayerState;

const ENEMY_HEALTH: u16 = 5;

pub struct EnemyMaterials {
	red: Handle<StandardMaterial>,
	green: Handle<StandardMaterial>,
	blue: Handle<StandardMaterial>,
}

pub fn create_enemies(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let sphere_handle = meshes.add(Mesh::from(shape::UVSphere::default()));

	let red_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 0.0),
        unlit: true,
        ..StandardMaterial::default()
    });

    let green_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 1.0, 0.0),
        unlit: true,
        ..StandardMaterial::default()
    });

    let blue_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.0, 1.0),
        unlit: true,
        ..StandardMaterial::default()
    });

    let main_line_handle = meshes.add(Mesh::from(shape::Box::new(STICK_SIZE, MAJOR_HEIGHT, STICK_SIZE)));
    let minor_line_handle = meshes.add(Mesh::from(shape::Box::new(STICK_SIZE, MINOR_HEIGHT, STICK_SIZE)));

    create_enemy(&mut commands, sphere_handle.clone(), red_material_handle.clone(), main_line_handle.clone(), minor_line_handle.clone());
    create_enemy(&mut commands, sphere_handle.clone(), green_material_handle.clone(), main_line_handle.clone(), minor_line_handle.clone());
    create_enemy(&mut commands, sphere_handle, blue_material_handle.clone(), main_line_handle, minor_line_handle);

    commands.insert_resource(EnemyMaterials {
    	red: red_material_handle,
    	green: green_material_handle,
    	blue: blue_material_handle,
    });
}

pub fn create_enemy(
    commands: &mut Commands, 
    sphere_handle: Handle<Mesh>,
    material_handle: Handle<StandardMaterial>,
    main_line_handle: Handle<Mesh>,
    minor_line_handle: Handle<Mesh>,
) {
    //first, sphere head
    commands.spawn_bundle(PbrBundle {
        mesh: sphere_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(20.0 + fastrand::f32() * 20.0, MAJOR_HEIGHT + MINOR_HEIGHT , 0.0),
        ..PbrBundle::default()
    })
    .insert(Enemy)
    .insert(Sticky::Enemy)
    .insert(Head)
    .insert(Health {
    	amount: ENEMY_HEALTH,
    })
    .insert(AnimInfo {
        time_takes: 1.0,
        amount_through: 1.0,
        index: 0,
        anim: PlayerState::Spin,
    })
    .with_children(|parent| {

        parent.spawn_bundle(PbrBundle {
            mesh: main_line_handle,
            material: material_handle.clone(),
            ..PbrBundle::default()
        })
        .insert(Enemy)
        .insert(Sticky::Enemy)
        .with_children(|parent| { 
            parent.spawn_bundle(PbrBundle {
                mesh: minor_line_handle.clone(),
                material: material_handle.clone(),
                ..PbrBundle::default()
            })
            .insert(Enemy)
            .insert(Sticky::Enemy)
            .insert(AnimPos::default())
            .insert(LEFT_ARM);

            parent.spawn_bundle(PbrBundle {
                mesh: minor_line_handle.clone(),
                material: material_handle.clone(),
                ..PbrBundle::default()
            })
            .insert(Enemy)
            .insert(Sticky::Enemy)
            .insert(AnimPos::default())
            .insert(RIGHT_ARM);
            parent.spawn_bundle(PbrBundle {
                mesh: minor_line_handle.clone(),
                material: material_handle.clone(),
                ..PbrBundle::default()
            })
            .insert(Enemy)
            .insert(Sticky::Enemy)
            .insert(AnimPos::default())
            .insert(LEFT_LEG);
            
            parent.spawn_bundle(PbrBundle {
                mesh: minor_line_handle,
                material: material_handle,
                ..PbrBundle::default()
            })
            .insert(Enemy)
            .insert(Sticky::Enemy)
            .insert(AnimPos::default())
            .insert(RIGHT_LEG);
        });
    });
}

#[derive(Component)]
pub struct Health {
	pub amount: u16,
}

pub fn place_enemies_system(
	mut query: Query<&mut Transform, (With<Enemy>, With<Head>)>,
) {

	let y = MAJOR_HEIGHT + MINOR_HEIGHT + 1.0;

	for mut enemy_trans in query.iter_mut() {
		enemy_trans.translation.x = (fastrand::f32() - 0.5) * 40.0;
		enemy_trans.translation.y = y;
		enemy_trans.translation.z = (fastrand::f32() - 0.5) * 40.0;
	}
}