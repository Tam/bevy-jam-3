#import bevy_core_pipeline::fullscreen_vertex_shader
#import bevy_pbr::prepass_utils

@group(0) @binding(0)
var screen_texture : texture_2d<f32>;
@group(0) @binding(1)
var depth_prepass_texture : texture_depth_2d;
@group(0) @binding(2)
var normal_prepass_texture : texture_2d<f32>;
@group(0) @binding(3)
var texture_sampler : sampler;

const SOBEL_X = array<f32, 9>(
	1., 0., -1.,
	2., 0., -2.,
	1., 0., -1.,
);

const SOBEL_Y = array<f32, 9>(
	 1.,  2.,  1.,
	 0.,  0.,  0.,
	-1., -2., -1.,
);

@fragment
fn fragment (
	in : FullscreenVertexOutput,
	@builtin(sample_index) sample_index : u32,
) -> @location(0) vec4<f32> {
	let frag_coord = in.position;

	let color = textureSample(screen_texture, texture_sampler, in.uv);

	if false {
		let depth = prepass_depth(frag_coord, sample_index);
		return vec4<f32>(depth, depth, depth, 1.);
	} else {
		let normal = prepass_normal(frag_coord, sample_index);
		return vec4(normal, 1.);
	}

	return color;
}
