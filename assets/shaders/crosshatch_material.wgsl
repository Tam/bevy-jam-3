#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::fog
#import bevy_pbr::pbr_functions
#import bevy_pbr::pbr_ambient

struct CrosshatchMaterial {
	start_color : vec4<f32>,
	end_color   : vec4<f32>,
}

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@group(1) @binding(0)
var<uniform> material : CrosshatchMaterial;

@fragment
fn fragment (in: FragmentInput) -> @location(0) vec4<f32> {
	let base_color = mix(material.start_color, material.end_color, in.uv.y);

	let layer = i32(in.world_position.x) & 0x3;

	// Prepare a 'processed' StandardMaterial by sampling all textures to resolve
	// the material members
	var pbr_input: PbrInput = pbr_input_new();
	pbr_input.material.base_color = base_color;

	pbr_input.frag_coord = in.frag_coord;
	pbr_input.world_position = in.world_position;
	pbr_input.world_normal = prepare_world_normal(
		in.world_normal,
		(pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u,
		in.is_front,
	);

	pbr_input.is_orthographic = view.projection[3].w == 1.0;

	pbr_input.N = apply_normal_mapping(
		pbr_input.material.flags,
		pbr_input.world_normal,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
		in.world_tangent,
#endif
#endif
		in.uv,
	);
	pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);
	pbr_input.flags = MESH_FLAGS_SHADOW_RECEIVER_BIT;

	return tone_mapping(pbr(pbr_input));
}
