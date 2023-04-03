#define_import_path utils::random

fn random_1d (seed : f32) -> f32 {
	return fract(sin(seed) * 100000.0);
}

// Author @patriciogv - 2015
// http://patriciogonzalezvivo.com
fn random_2d (seed : vec2<f32>) -> f32 {
	return fract(
		sin(dot(seed, vec2(12.9898, 78.223)))
		* 43758.5453123
	);
}
