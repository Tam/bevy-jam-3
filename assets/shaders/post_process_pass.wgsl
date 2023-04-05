#import bevy_core_pipeline::fullscreen_vertex_shader
#import bevy_pbr::prepass_utils
#import utils::noise

@group(0) @binding(0)
var screen_texture : texture_2d<f32>;
@group(0) @binding(1)
var depth_prepass_texture : texture_depth_2d;
@group(0) @binding(2)
var normal_prepass_texture : texture_2d<f32>;
@group(0) @binding(3)
var texture_sampler : sampler;

var<private> SOBEL_X : array<f32, 9> = array<f32, 9>(
	1., 0., -1.,
	2., 0., -2.,
	1., 0., -1.,
);

var<private> SOBEL_Y : array<f32, 9> = array<f32, 9>(
	 1.,  2.,  1.,
	 0.,  0.,  0.,
	-1., -2., -1.,
);

fn depth_edge (frag_coord : vec4<f32>, sample_index : u32) -> f32 {
	var x_pass = vec3<f32>(0.);
	var y_pass = vec3<f32>(0.);

	var i = 0;
	for (var x = -1.; x <= 1.; x += 1.) {
		for (var y = -1.; y <= 1.; y += 1.) {
			var pos = frag_coord;
			pos.x += x;
			pos.y += y;
			let sample = 0.001 / prepass_depth(pos, sample_index);
			let ci = u32(3. * x + y + 3.);
			x_pass += sample * SOBEL_X[i];
			y_pass += sample * SOBEL_Y[i];
			i++;
		}
	}

	let edge = sqrt(dot(x_pass, x_pass) + dot(y_pass, y_pass));

	if (edge < 0.05) {
		return 0.;
	}

	return edge;
}

fn normal_edge (frag_coord : vec4<f32>, sample_index : u32) -> f32 {
	var x_pass = vec3<f32>(0.);
	var y_pass = vec3<f32>(0.);

	var i = 0;
	for (var x = -1.; x <= 1.; x += 1.) {
		for (var y = -1.; y <= 1.; y += 1.) {
			var pos = frag_coord;
			pos.x += x;
			pos.y += y;
			let sample = prepass_normal(pos, sample_index);
			x_pass += sample * SOBEL_X[i];
			y_pass += sample * SOBEL_Y[i];
			i++;
		}
	}

	let edge = sqrt(dot(x_pass, x_pass) + dot(y_pass, y_pass));

	if (edge < 3.) {
		return 0.;
	}

	return edge;
}

@fragment
fn fragment (
	in : FullscreenVertexOutput,
	@builtin(sample_index) sample_index : u32,
) -> @location(0) vec4<f32> {
	let frag_coord = in.position;

	let color = textureSample(screen_texture, texture_sampler, in.uv);

	let edge = max(
		depth_edge(frag_coord, sample_index),
		normal_edge(frag_coord, sample_index),
	);

	if (edge > 0.01) {
		return vec4<f32>(0., 0., 0., 1.);
	}

	return color;
}
