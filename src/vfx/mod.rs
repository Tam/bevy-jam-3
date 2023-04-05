use bevy::app::{App, Plugin};
use bevy::prelude::MaterialPlugin;
use bevy::utils::default;
pub use crate::vfx::crosshatch_material::CrosshatchMaterial;
use crate::vfx::post_process::PostProcessPlugin;
use crate::vfx::utils::UtilShaders;

mod post_process;
mod utils;
mod crosshatch_material;

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<UtilShaders>()
			.add_plugin(PostProcessPlugin)
			// .add_plugin(MaterialPlugin::<CrosshatchMaterial>::default())
			.add_plugin(MaterialPlugin::<CrosshatchMaterial> {
				prepass_enabled: true,
				..default()
			})
		;
	}
}
