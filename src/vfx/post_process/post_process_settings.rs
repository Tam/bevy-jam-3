use bevy::prelude::Component;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::ShaderType;

#[derive(Component, Default, Copy, Clone, ExtractComponent, ShaderType)]
pub struct PostProcessSettings {
	pub intensity : f32,
}
