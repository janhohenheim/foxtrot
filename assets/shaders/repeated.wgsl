// Same imports as <https://github.com/bevyengine/bevy/blob/main/crates/bevy_pbr/src/render/pbr.wgsl>
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::pbr_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions

struct Repeats {
    horizontal: u32,
    vertical: u32,
    _wasm_padding1: u32,
    _wasm_padding2: u32,
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;
@group(1) @binding(2)
var<uniform> repeats: Repeats;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
}


fn get_texture_sample(coords: vec2<f32>) -> vec4<f32> {
    let repeated_coords = vec2<f32>(
        (coords.x % (1. / f32(repeats.horizontal))) * f32(repeats.horizontal),
        (coords.y % (1. / f32(repeats.vertical))) * f32(repeats.vertical)
    );
    return textureSample(texture, texture_sampler, repeated_coords);
}

/// Adapted from <https://github.com/bevyengine/bevy/blob/main/crates/bevy_pbr/src/render/pbr.wgsl#L30>
fn get_pbr_output(in: FragmentInput) -> vec4<f32> {
    var material = standard_material_new();
    material.perceptual_roughness = 1.0;

    var pbr_input = pbr_input_new();
    pbr_input.frag_coord = in.frag_coord;
    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = in.world_normal;
    pbr_input.material = material;
    pbr_input.world_normal = prepare_world_normal(
        in.world_normal,
        (material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u,
        in.is_front,
    );

    pbr_input.is_orthographic = view.projection[3].w == 1.0;

    pbr_input.N = apply_normal_mapping(
        material.flags,
        pbr_input.world_normal,
        in.uv,
    );
    pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);

    var output_color = pbr(pbr_input);

    output_color = tone_mapping(output_color);
    var output_rgb = output_color.rgb;
    output_rgb = pow(output_rgb, vec3<f32>(1.0 / 2.2));
    output_rgb = output_rgb + screen_space_dither(in.frag_coord.xy);
    // This conversion back to linear space is required because our output texture format is
    // SRGB; the GPU will assume our output is linear and will apply an SRGB conversion.
    output_rgb = pow(output_rgb, vec3<f32>(2.2));
    output_color = vec4(output_rgb, output_color.a);

    return output_color;
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let texture = get_texture_sample(in.uv);
    let pbr_output = get_pbr_output(in);


    return texture * pbr_output;
}