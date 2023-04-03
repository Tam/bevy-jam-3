#import utils::random

fn value_noise_1d (seed : f32) -> f32 {
	let i = floor(seed);
	let f = fract(seed);

	return mix(random_1d(i), random_1d(i + 1.), smoothstep(0., 1., f));
}

fn value_noise_2d (seed : vec2<f32>) -> f32 {
	let i = floor(seed);
	let f = fract(seed);

	let a = random_1d(i);
	let b = random_1d(i + vec2<f32>(1., 0.));
	let c = random_1d(i + vec2<f32>(0., 1.));
	let d = random_1d(i + vec2<f32>(1., 1.));

	let u = smoothstep(0., 1., f);

	return mix(a, b, u.x)
		 + (c - a) * u.y * (1. - u.x)
		 + (d - b) * u.x * u.y;
}
