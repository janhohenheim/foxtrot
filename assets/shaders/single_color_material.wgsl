@group(2) @binding(0) var<uniform> material_color: vec4<f32>;

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return material_color;
}
