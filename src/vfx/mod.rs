use bevy::app::{App, Plugin};
use crate::vfx::post_process::PostProcessPlugin;
use crate::vfx::utils::UtilsPlugin;

mod post_process;
mod utils;

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(UtilsPlugin)
			.add_plugin(PostProcessPlugin)
		;
	}
}
