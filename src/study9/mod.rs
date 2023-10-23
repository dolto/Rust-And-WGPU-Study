mod common;
mod transforms;
mod vertex;

fn vertex(p:[i8; 3], n: [i8; 3]) -> common::Vertex {
    common::Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        normal: [n[0] as f32, n[1] as f32, n[2] as f32, 1.0],
    }
}

fn create_vertices() -> Vec<common::Vertex> {
    let pos = vertex::cube_positions();
    let normal= vertex::cube_normals();
    
    let mut data:Vec<common::Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(vertex(pos[i], normal[i]));
    }
    data.to_vec()
}

pub fn study9_main(){
    let vertex_data = create_vertices();
    let light_data = common::light([1.0,0.0,0.0], [1.0, 1.0, 0.0], 0.1, 0.6, 0.3, 30.0);
    /// c: color 기본 색
    /// sc: specular color 반사광 색
    /// ai: ambient intencity 주변광 비율 
    /// di: diffuse intensity 난사광 비율
    /// si: specular intensity 반사광 비율
    /// ss: specular shininess 반사광의 재질?
    common::run(&vertex_data, light_data, "cube");
}