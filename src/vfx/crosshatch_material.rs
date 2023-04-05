use bevy::prelude::{Color, Material};
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "46eee64b-2f04-4b31-9b65-9e0ff920f0ef"]
pub struct CrosshatchMaterial {
	#[uniform(0)] start_color : Color,
	#[uniform(0)] end_color   : Color,
}

impl Material for CrosshatchMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/crosshatch_material.wgsl".into()
	}
	
	fn prepass_fragment_shader() -> ShaderRef {
		"shaders/crosshatch_fragment_prepass.wgsl".into()
	}
}

impl CrosshatchMaterial {
	pub fn new (start : Color, end : Color) -> Self {
		Self {
			start_color: start,
			end_color: end,
		}
	}
	
	pub fn splat (color : Color) -> Self {
		Self {
			start_color: color,
			end_color: color,
		}
	}
}

impl From<Color> for CrosshatchMaterial {
	fn from(value: Color) -> Self {
		CrosshatchMaterial::splat(value)
	}
}

impl From<(Color, Color)> for CrosshatchMaterial {
	fn from(value: (Color, Color)) -> Self {
		CrosshatchMaterial::new(value.0, value.1)
	}
}
