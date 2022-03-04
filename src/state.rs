use crate::{Player, Head};
use bevy::prelude::*;

const LOSE_Y: f32 = -50.0;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
	Loading,
	Playing,
	GameOver,
	Win,
}

pub fn check_lose_system(
	mut app_state: ResMut<State<AppState>>,
	player_query: Query<&Transform, (With<Player>, With<Head>)>,
) {
	let player_trans = player_query.get_single().unwrap();

	if player_trans.translation.y < LOSE_Y {
		app_state.set(AppState::GameOver).unwrap();
	}
}

pub fn game_over_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn_bundle(UiCameraBundle::default());

	commands.spawn_bundle(TextBundle {
		style: Style {
			align_self: AlignSelf::Center,
			align_content: AlignContent::Center,
            position_type: PositionType::Absolute,
            ..Style::default()
		},
		text: Text::with_section(
			"Game over! :(",
			TextStyle {
				font: asset_server.load("verdanab.ttf"),
				font_size: 100.0,
				color: Color::WHITE,
			},
			TextAlignment {
				horizontal: HorizontalAlign::Center,
				..TextAlignment::default()
			}
		),
		..TextBundle::default()
	});
}