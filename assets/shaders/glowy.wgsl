// Done following <https://www.youtube.com/watch?v=O6A_nVmpvhc>

#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
// Bring in consts like PI
#import bevy_pbr::utils
// Bring in tone-mapping functions
#import bevy_pbr::pbr_types
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions


@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
}

/// Convert world direction to (x, y) to sample HDRi
fn dir_to_equirectangular(dir: vec3<f32>) -> vec2<f32> {
    // atan2 returns the angle theta between the positive x axis and a coordinate pair (y, x) in -pi < theta < pi
    // Be careful: y comes before x
    let x = atan2(dir.z, dir.x) / (2. * PI) + 0.5; // 0-1
    let y = acos(dir.y) / PI; // 0-1
    // Polar coordinates? idk. All I know is that these are two normalized angles.
    return vec2(x, y);
}

/// Source: <https://registry.khronos.org/OpenGL-Refpages/gl4/html/refract.xhtml>
/// Params: incident vector i, surface normal n, and the ratio of indices of refraction eta.
/// Is prefixed with "own_" because the built-in refract function is only available in WASM for some reason.
fn own_refract(i: vec3<f32>, n: vec3<f32>, eta: f32) -> vec3<f32> {
    let k = 1.0 - eta * eta * (1.0 - dot(n, i) * dot(n, i));
    let k = max(k, 0.0);
    return eta * i - (eta * dot(n, i) + sqrt(k)) * n;
}

/// Returns RGB vector
fn get_texture_sample(direction: vec3<f32>) -> vec4<f32> {
    let coords = dir_to_equirectangular(direction);
    return textureSample(texture, texture_sampler, coords);
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    // vec normal to face
    var n = normalize(in.world_normal);
    // vec from face origin to camera
    var v = normalize(view.world_position.xyz - in.world_position.xyz);
    // 0: n and v are perp
    // 1: n and v are parallel
    let n_dot_v = max(dot(n, v), 0.0001);
    // Fresnel values describe how light is reflected at the surface between two mediums
    // See <https://en.wikipedia.org/wiki/Fresnel_equations>
    // This will make sure we have a kind of "halo" of light around our reflection, which the eye expects
    // since reflections are usually much brigther around the edges
    let fresnel = clamp(1. - n_dot_v, 0.0, 1.0);
    let fresnel = pow(fresnel, 5.) * 2.;

    // Increase contrast
    // => Face in the center of the sphere have normals pointing to the camera, which makes them brighter
    let glow = pow(n_dot_v, 10.) * 50.;

    let black = vec3(0., 0., 0.);
    let orange = vec3(0.5, 0.1, 0.);
    // The higher glow is, the more orange the face becomes
    let color = mix(black, orange, glow);

    // Only the R channel of a shifted normal
    // The shift is arbitrary in nature. Its function is to make it more interesting,
    // otherwise we would just be using the R channel of the normal we are looking at anyways.
    let bump = get_texture_sample(n * vec3(1., -0.5, 1.) - vec3(0., 0.5, 0.))[0];

    // reflect image like a mirror
    let reflection = get_texture_sample(reflect(-v, n)).rgb;


    // This n is shifted a bit stochastically so that refraction is heterogenous
    let bumped_n = n + bump * 2.;
    // refract image like a glass ball would
    let refraction = get_texture_sample(own_refract(-v, bumped_n, 1./1.52)).rgb;

    let alpha = textureSample(texture, texture_sampler, in.uv).a;

    /// The RGB of the refraction is multiplied with a gradient from center (orange) to edge (black)
    /// The RGB of the reflection is multiplied with a fresnel on the edge, making it only appear as a "sheen"
    let total = color * refraction + reflection * (fresnel + 0.05);
    return vec4<f32>(total, alpha);
}