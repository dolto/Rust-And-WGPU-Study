struct Output{
    @builtin(position) position: vec4<f32>,
    @location(0) v_color: vec4<f32>  
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32 ) 
    -> Output {
        var pos = array<vec2<f32>, 6>( //여기저기에 점을 찍음
            vec2<f32>(-0.5, 0.7),
            vec2<f32>(0.3, 0.6),
            vec2<f32>(0.5, 0.3),
            vec2<f32>(0.4, -0.5),
            vec2<f32>(-0.4, -0.4),
            vec2<f32>(-0.3, 0.2)
        );

        var color: array<vec3<f32>,6> = array<vec3<f32>,6>(
            vec3<f32>(1.0,0.0,0.0),
            vec3<f32>(0.0,1.0,0.0),
            vec3<f32>(0.0,0.0,1.0),
            vec3<f32>(1.0,0.0,0.0),
            vec3<f32>(0.0,1.0,0.0),
            vec3<f32>(0.0,0.0,1.0)
        );
        
        var output: Output;
        output.position = vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
        output.v_color = vec4<f32>(color[in_vertex_index], 1.0);
        return output;
    }

@fragment
fn fs_main(@location(0) v_color: vec4<f32>) 
-> @location(0) vec4<f32>{
    return v_color; //점 색깔만 지정해둠
}