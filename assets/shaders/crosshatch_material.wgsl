struct CrosshatchMaterial {
	start_color : vec4<f32>,
	end_color   : vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material : CrosshatchMaterial;

@fragment
fn fragment (
	#import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
	return mix(material.start_color, material.end_color, uv.y);
}
