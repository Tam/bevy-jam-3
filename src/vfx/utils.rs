use bevy::prelude::{App, AssetServer, Handle, Plugin, Shader};

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
	fn build(&self, app: &mut App) {
		let assets_server = app.world.resource::<AssetServer>();
		
		for name in [
			"random",
			"noise",
		] {
			let _ : Handle<Shader> = assets_server.load(
				format!("shaders/utils/{name}.wgsl")
			);
		}
	}
}
