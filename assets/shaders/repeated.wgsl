#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::utils
#import bevy_pbr::shadows

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coords: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
}


fn get_texture_sample(coords: vec2<f32>) -> vec3<f32> {
    let num_repeats = 10.;
    let repeated_coords = (coords % (1./num_repeats)) * num_repeats;
    return textureSample(texture, texture_sampler, repeated_coords).rgb;
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let texture = get_texture_sample(in.uv);
    let shadow = fetch_directional_shadow(0u, in.world_position, in.world_normal);
    let product = texture * (shadow * 0.8 + 0.2);
    return vec4(product, 0.);
}