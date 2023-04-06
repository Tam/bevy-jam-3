#define_import_path utils::light

// distanceAttenuation is simply the square falloff of light intensity
// combined with a smooth attenuation at the edge of the light radius
//
// light radius is a non-physical construct for efficiency purposes,
// because otherwise every light affects every fragment in the scene
fn getDistanceAttenuation(distanceSquare: f32, inverseRangeSquared: f32) -> f32 {
    let factor = distanceSquare * inverseRangeSquared;
    let smoothFactor = saturate(1.0 - factor * factor);
    let attenuation = smoothFactor * smoothFactor;
    return attenuation * 1.0 / max(distanceSquare, 0.0001);
}

fn point_light (
	light_id : u32,
	world_position : vec3<f32>,
	base_color : vec3<f32>,
	normal : vec3<f32>,
) -> vec3<f32> {
	let light = &point_lights.data[light_id];
	let light_to_frag = (*light).position_radius.xyz - world_position.xyz;
	let light_normal = normalize(light_to_frag);
	let normal_over_light_normal = saturate(dot(normal, light_normal));
	let distance_square = dot(light_to_frag, light_to_frag);
    let range_attenuation = getDistanceAttenuation(distance_square, (*light).color_inverse_square_range.w);

	return base_color * (*light).color_inverse_square_range.rgb * (range_attenuation * normal_over_light_normal);
}
