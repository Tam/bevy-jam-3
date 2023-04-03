mod vfx;

use bevy::core_pipeline::fxaa::{Fxaa, Sensitivity};
use bevy::core_pipeline::prepass::{DepthPrepass, NormalPrepass};
use bevy::prelude::*;
use bevy::window::PresentMode;
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin};
use smooth_bevy_cameras::LookTransformPlugin;
use crate::vfx::VfxPlugin;

fn main() {
	App::new()
		.insert_resource(Msaa::Off)
		.add_plugins(DefaultPlugins.set(AssetPlugin {
			watch_for_changes: true,
			..default()
		}).set(WindowPlugin {
			primary_window: Some(Window {
				present_mode: PresentMode::AutoVsync,
				..default()
			}),
			..default()
		}))
		.add_plugin(VfxPlugin)
		.add_plugin(LookTransformPlugin)
		.add_plugin(OrbitCameraPlugin::default())
		.add_startup_system(setup)
		.run();
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	shaders : ResMut<Assets<Shader>>,
) {
	for (_, shader) in shaders.iter() {
		if let Some(path) = shader.import_path() {
			println!("{:?}", path);
		}
	}
	
	// Camera
	commands.spawn((
		Camera3dBundle::default(),
		DepthPrepass,
		NormalPrepass,
		Fxaa {
			enabled: true,
			edge_threshold: Sensitivity::Extreme,
			edge_threshold_min: Sensitivity::Extreme,
		},
	)).insert(OrbitCameraBundle::new(
		OrbitCameraController::default(),
		Vec3::new(-2.0, 5.0, 5.0),
		Vec3::ZERO,
		Vec3::Y,
	));
	
	// Plane
	commands.spawn(PbrBundle {
		mesh: meshes.add(shape::Plane::from_size(5.0).into()),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..default()
	});
	
	// Cube
	commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
		material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
		transform: Transform::from_xyz(-1.0, 0.5, 1.0),
		..default()
	});
	
	// Ball
	commands.spawn(PbrBundle {
		mesh: meshes.add(shape::Icosphere::default().try_into().unwrap()),
		material: materials.add(Color::rgb(1.0, 0.1, 0.1).into()),
		transform: Transform::from_xyz(1.0, 0.5, -1.0),
		..default()
	});
	
	// Light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});
}

