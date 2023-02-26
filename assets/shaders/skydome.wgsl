// Same imports as <https://github.com/bevyengine/bevy/blob/main/crates/bevy_pbr/src/render/pbr.wgsl>
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
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
    // Polar coordinates? idk. All I know is that these are two normalized angles.
    return vec2(x, y);
}


fn get_texture_sample(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(texture, texture_sampler, uv);
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var n = normalize(in.world_normal);
    let uv = dir_to_equirectangular(n);
    return get_texture_sample(uv);
}
