use bevy::app::{App, Plugin};
use crate::vfx::post_process::PostProcessPlugin;
use crate::vfx::utils::UtilShaders;

mod post_process;
mod utils;

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<UtilShaders>()
			.add_plugin(PostProcessPlugin)
		;
	}
}
