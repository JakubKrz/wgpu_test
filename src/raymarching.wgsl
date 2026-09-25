struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vi: u32,
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vec2<f32>(
        f32((vi << 1u) & 2u),
        f32(vi & 2u),
    );
    out.clip_position = vec4<f32>(out.uv * 2.0 - 1.0, 0.0, 1.0);
    out.uv.y = 1.0 - out.uv.y;
    return out;
}
struct RaymarchUniform {
    aspect_ratio: f32,
    fov_scale: f32,
}
@group(0) @binding(0)
var<uniform> params: RaymarchUniform;

fn map(point: vec3<f32>) -> f32 {
    let ball_pos = vec3<f32>(0.0, 0.0, -0.5);
    let ball_pos2 = vec3(1.0, 2.0, -5.0);
    let r = 0.4;
    let distance1 = distance(point, ball_pos) - r;

    let distance2 = distance(point, ball_pos2) - r;
    return min(distance1, distance2);
}
@fragment
fn fs_main(vs: VertexOutput) -> @location(0) vec4<f32> {
    var color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    let x = (vs.uv.x - 0.5) * 2.0 * params.fov_scale * params.aspect_ratio;
    let y = (vs.uv.y - 0.5) * 2.0 * params.fov_scale;

    let origin = vec3(0.0, 0.0, 1.0);
    let direction = normalize(vec3(x, y, 0) - origin);
    let near_plane_distance = 1.0;//maybe add to params uniform
    let far_plane_distance = 100.0;// -||-
    var current_point = origin + direction * near_plane_distance;
    var total_distance = near_plane_distance;

    for (var i = 0; i < 100; i++) {
        let dist = map(current_point);
        if dist < 0.001 {
            color.g = f32(i) / 100.0;
            break;
        }
        if dist > far_plane_distance { break; }

        total_distance += dist;
        current_point = origin + direction * total_distance;
    }
    color.r = 0.0;
    color.b = total_distance / 10.0;
    //color.g = 0.0;
    return color;
}
