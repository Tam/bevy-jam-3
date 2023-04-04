use bevy::prelude::{AssetServer, FromWorld, Handle, Resource, Shader, World};

#[derive(Resource)]
pub struct UtilShaders (Vec<Handle<Shader>>);

impl FromWorld for UtilShaders {
	fn from_world(world: &mut World) -> Self {
		let assets_server = world.resource::<AssetServer>();
		
		let mut shaders = Self(Vec::new());
		
		for name in [
			"random",
			"noise",
		] {
			shaders.0.push(assets_server.load(format!("shaders/utils/{name}.wgsl")));
		}
		
		shaders
	}
}
