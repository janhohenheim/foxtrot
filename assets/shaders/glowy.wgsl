#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
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
    let glow = pow(n_dot_v, 10.0);

    let black = vec3(0.0, 0.0, 0.0);
    let orange = vec3(0.5, 0.1, 0.0);
    // The higher glow is, the more orange the face becomes
    let color = mix(black, orange, glow);

    return vec4(vec3(color), 1.0);
}