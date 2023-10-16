struct VOutput{
    @location(0) v_color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
    ///각각 커스텀 저장소0번째에 벡터4 실수형을 갖고, 
    ///내장 위치포지션을 가진 구조체
};

//[[stage(vertex)]]
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32 ) -> VOutput {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(0.0,0.5),
        vec2<f32>(-0.5,-0.5),
        vec2<f32>(0.5,-0.5)
    );
    var color = array<vec3<f32>,3>(
        vec3<f32>(1.0,0.0,0.0), //R
        vec3<f32>(0.0,1.0,0.0), //G
        vec3<f32>(0.0,0.0,1.0)  //B
    );

    var out:VOutput;
    out.position = vec4<f32>(pos[in_vertex_index], 0.0,1.0);
    out.v_color = vec4<f32>(color[in_vertex_index], 1.0);
    return out;
}

//[[stage(fragment)]]
@fragment
fn fs_main(in: VOutput) -> @location(0) vec4<f32> {
    return in.v_color;
}