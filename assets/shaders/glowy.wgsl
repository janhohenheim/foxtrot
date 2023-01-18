#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
// Brings in PI
#import bevy_pbr::utils

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
    // Polar coordinates? idk
    return vec2(x, y);
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
    // Increase contrast
    // => Face in the center of the sphere have normals pointing to the camera, which makes them brighter
    let glow = pow(n_dot_v, 10.);

    let black = vec3(0., 0., 0.);
    let orange = vec3(0.5, 0.1, 0.);
    // The higher glow is, the more orange the face becomes
    let color = mix(black, orange, glow);

    // reflection of vec from camera to face on vec normal to face
    // TODO: wtf is a reflection
    let reflection_direction = reflect(-v, n);
    let reflect_coords = dir_to_equirectangular(reflection_direction);
    let reflection = textureSample(texture, texture_sampler, reflect_coords).rgb;

    return vec4(reflection, 1.);
}