#import bevy_core_pipeline::fullscreen_vertex_shader
#import bevy_pbr::prepass_utils

struct PostProcessSettings {
	intensity : f32,
}

@group(0) @binding(0)
var screen_texture : texture_2d<f32>;
@group(0) @binding(1)
var depth_prepass_texture : texture_2d<f32>;
@group(0) @binding(2)
var normal_prepass_texture : texture_2d<f32>;
@group(0) @binding(3)
var texture_sampler : sampler;
@group(0) @binding(4)
var<uniform> settings : PostProcessSettings;

@fragment
fn fragment (
	in : FullscreenVertexOutput,
	@builtin(sample_index) sample_index : u32,
) -> @location(0) vec4<f32> {
	let frag_coord = in.position;
	let offset_strength = settings.intensity;

	return vec4<f32>(
		textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(offset_strength, -offset_strength)).r,
		textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-offset_strength, 0.0)).g,
		textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, offset_strength)).b,
		1.0
	);
}
