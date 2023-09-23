#import bevy_pbr::utils
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::clustered_forward
#import bevy_pbr::shadows
#import bevy_pbr::prepass_utils
#import utils::light

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

	var direct_light : vec3<f32> = vec3<f32>(0.);

	let view_z = dot(vec4<f32>(
		view.inverse_view[0].z,
		view.inverse_view[1].z,
		view.inverse_view[2].z,
		view.inverse_view[3].z,
	), in.world_position);
	let cluster_index = fragment_cluster_index(in.frag_coord.xy, view_z, false /* Perspective Camera */);
	let offset_and_counts = unpack_offset_and_counts(cluster_index);

#ifdef LOAD_PREPASS_NORMALS
	let world_normal = prepass_normal(in.frag_coord, 0u);
#else // load_prepass_normals
	let world_normal = prepare_world_normal(
		in.world_normal,
		false, // not double-sided
		in.is_front,
	);
#endif // load_prepass_normals

	let normal = apply_normal_mapping(
		0u,
		world_normal,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
		in.world_tangent,
#endif
#endif
#ifdef VERTEX_UVS
		in.uv,
#endif
	);

	let V = calculate_view(in.world_position);

	// Neubelt and Pettineo 2013, "Crafting a Next-gen Material Pipeline for The Order: 1886"
    let NdotV = max(dot(normal, V), 0.0001);

	// Point lights
	for (
		var i : u32 = offset_and_counts[0];
		i < offset_and_counts[0] + offset_and_counts[1];
		i = i + 1u
	) {
		let light_id = get_light_id(i);

		// TODO: allow unlit / unshadowed materials
		let shadow : f32 = fetch_point_shadow(
			light_id,
			in.world_position,
			in.world_normal,
		);

		let light_contrib = point_light(
			light_id,
			in.world_position.xyz,
			base_color.rgb,
			normal,
		);

		direct_light += light_contrib * shadow;
		if (shadow < 1.) {
			direct_light += vec3<f32>(1., 0., 1.) * 0.04;
		} else {
			direct_light += light_contrib;
		}
	}

	// TODO: spot lights

	// TODO: directional lights

	let diffuse_ambient = EnvBRDFApprox(base_color.rgb, F_AB(1.0, NdotV));// * occlusion
	let ambient_light = diffuse_ambient * lights.ambient_color.rgb;

	return vec4<f32>(
		direct_light + ambient_light,
		1.
	);
}

fn EnvBRDFApprox(f0: vec3<f32>, f_ab: vec2<f32>) -> vec3<f32> {
    return f0 * f_ab.x + f_ab.y;
}

// Scale/bias approximation
// https://www.unrealengine.com/en-US/blog/physically-based-shading-on-mobile
// TODO: Use a LUT (more accurate)
fn F_AB(perceptual_roughness: f32, NoV: f32) -> vec2<f32> {
    let c0 = vec4<f32>(-1.0, -0.0275, -0.572, 0.022);
    let c1 = vec4<f32>(1.0, 0.0425, 1.04, -0.04);
    let r = perceptual_roughness * c0 + c1;
    let a004 = min(r.x * r.x, exp2(-9.28 * NoV)) * r.x + r.y;
    return vec2<f32>(-1.04, 1.04) * a004 + r.zw;
}

fn calculate_view(world_position: vec4<f32>) -> vec3<f32> {
	return normalize(view.world_position.xyz - world_position.xyz);
}

fn prepare_world_normal(
    world_normal: vec3<f32>,
    double_sided: bool,
    is_front: bool,
) -> vec3<f32> {
    var output: vec3<f32> = world_normal;
#ifndef VERTEX_TANGENTS
#ifndef STANDARDMATERIAL_NORMAL_MAP
    // NOTE: When NOT using normal-mapping, if looking at the back face of a double-sided
    // material, the normal needs to be inverted. This is a branchless version of that.
    output = (f32(!double_sided || is_front) * 2.0 - 1.0) * output;
#endif
#endif
    return output;
}

fn apply_normal_mapping(
    standard_material_flags: u32,
    world_normal: vec3<f32>,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
    world_tangent: vec4<f32>,
#endif
#endif
#ifdef VERTEX_UVS
    uv: vec2<f32>,
#endif
) -> vec3<f32> {
    // NOTE: The mikktspace method of normal mapping explicitly requires that the world normal NOT
    // be re-normalized in the fragment shader. This is primarily to match the way mikktspace
    // bakes vertex tangents and normal maps so that this is the exact inverse. Blender, Unity,
    // Unreal Engine, Godot, and more all use the mikktspace method. Do not change this code
    // unless you really know what you are doing.
    // http://www.mikktspace.com/
    var N: vec3<f32> = world_normal;

#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
    // NOTE: The mikktspace method of normal mapping explicitly requires that these NOT be
    // normalized nor any Gram-Schmidt applied to ensure the vertex normal is orthogonal to the
    // vertex tangent! Do not change this code unless you really know what you are doing.
    // http://www.mikktspace.com/
    var T: vec3<f32> = world_tangent.xyz;
    var B: vec3<f32> = world_tangent.w * cross(N, T);
#endif
#endif

#ifdef VERTEX_TANGENTS
#ifdef VERTEX_UVS
#ifdef STANDARDMATERIAL_NORMAL_MAP
    // Nt is the tangent-space normal.
    var Nt = textureSample(normal_map_texture, normal_map_sampler, uv).rgb;
    if (standard_material_flags & STANDARD_MATERIAL_FLAGS_TWO_COMPONENT_NORMAL_MAP) != 0u {
        // Only use the xy components and derive z for 2-component normal maps.
        Nt = vec3<f32>(Nt.rg * 2.0 - 1.0, 0.0);
        Nt.z = sqrt(1.0 - Nt.x * Nt.x - Nt.y * Nt.y);
    } else {
        Nt = Nt * 2.0 - 1.0;
    }
    // Normal maps authored for DirectX require flipping the y component
    if (standard_material_flags & STANDARD_MATERIAL_FLAGS_FLIP_NORMAL_MAP_Y) != 0u {
        Nt.y = -Nt.y;
    }
    // NOTE: The mikktspace method of normal mapping applies maps the tangent-space normal from
    // the normal map texture in this way to be an EXACT inverse of how the normal map baker
    // calculates the normal maps so there is no error introduced. Do not change this code
    // unless you really know what you are doing.
    // http://www.mikktspace.com/
    N = Nt.x * T + Nt.y * B + Nt.z * N;
#endif
#endif
#endif

    return normalize(N);
}
