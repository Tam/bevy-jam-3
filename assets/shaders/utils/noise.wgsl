#define_import_path utils::noise

/// Originally by Ian MacEwan, Ashima Arts
/// https://github.com/ashima/webgl-noise
/// Noise functions ported to WGSL by tam (github.com/tam)

// Helpers
// =========================================================================

fn mod289_2d (x : vec2<f32>) -> vec2<f32> {
	return x - floor(x * (1. / 289.)) * 289.;
}

fn mod289_3d (x : vec3<f32>) -> vec3<f32> {
	return x - floor(x * (1. / 289.)) * 289.;
}

fn permute (x : vec3<f32>) -> vec3<f32> {
	return mod289_3d(((x * 34.) + 10.) * x);
}

// Simplex Noise
// =========================================================================

const SIMPLEX_NOISE_C : vec4<f32> = vec4<f32>(
	0.211324865405187, // (3.0-sqrt(3.0))/6.0
	0.366025403784439, // 0.5*(sqrt(3.0)-1.0)
   -0.577350269189626, // -1.0 + 2.0 * C.x
	0.024390243902439, // 1.0 / 41.0
);

fn simplex_noise_2d (point : vec2<f32>) -> f32 {
	// First corner
	var i  = floor(point + dot(point, SIMPLEX_NOISE_C.yy));
	let x0 = point - i + dot(i, SIMPLEX_NOISE_C.xx);

	// Other corners
	var i1 : vec2<f32> = vec2<f32>(0.);
	if (x0.x > x0.y) {
		i1 = vec2<f32>(1., 0.);
	} else {
		i1 = vec2<f32>(0., 1.);
	}

	var x12 : vec4<f32> = x0.xyxy + SIMPLEX_NOISE_C.xxzz;
	x12.x -= i1.x;
	x12.y -= i1.y;

	// Permutations
	i = mod289_2d(i); // Avoid truncation effects in permutation
	let p = permute(permute(i.y + vec3<f32>(0., i1.y, 1.))
		  + i.x + vec3<f32>(0., i1.x, 1.));

	var m = max(0.5 - vec3<f32>(
		dot(x0, x0),
		dot(x12.xy, x12.xy),
		dot(x12.zw, x12.zw),
	), vec3<f32>(0.));
	m = m * m;
	m = m * m;

	// Gradients: 41 points uniformly over a line, mapped onto a diamond.
	// The ring size 17 * 17 = 289 is close to a multiple of 41 (41 * 7 = 287)
	let x = 2. * fract(p * SIMPLEX_NOISE_C.www) - 1.;
	let h = abs(x) - 0.5;
	let ox = floor(x + 0.5);
	let a0 = x - ox;

	// Normalise gradients implicitly by scaling m
	// Approximation of m *= inversesqrt(a0 * a0 + h * h)
	m *= 1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h);

	// Compute final noise value at point
	var g : vec3<f32> = vec3<f32>(0.);
	g.x = a0.x * x0.x + h.x * x0.y;
	let yz = a0.yz * x12.xz + h.yz * x12.yw;
	g.y = yz.x;
	g.z = yz.y;

	return 130. * dot(m, g);
}
