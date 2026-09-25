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
fn map(point: vec3<f32>) -> f32 {
    let ball_pos = vec3<f32>(0.0, 0.0, 4.0);
    let r = 0.4;
    let distance = distance(point, ball_pos) - r;
    return distance;
}
@fragment
fn fs_main(vs: VertexOutput) -> @location(0) vec4<f32> {
    var color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    let x = (vs.uv.x - 0.5);
    let y = (vs.uv.y - 0.5);
    //TODO 
    //screen resolution - get form cpu
    let origin = vec3<f32>(x, y, -1.0);
    let dir = vec3<f32>(0.0, 0.0, 1.0);
    var cur_point = origin;
    for (var i = 0; i < 100; i++) {
        let dist = map(cur_point);
        if dist < 0.001 {
            color.g = f32(i) / 100.0;
            break;
        }
        cur_point += dir * dist;
    }
    color.b = 0.05;
    color.r = 0.0;
    //color.g = 0.0;
    return color;
}
