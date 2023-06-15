use spirv_cross::{spirv, hlsl};
use spirv_cross::hlsl::HlslVertexAttributeRemap;
use std::fs::File;
use std::io::Write;
use std::string::String;
use std::collections::HashMap;

fn main(){
    let mut shader_bytes = include_bytes!(env!("shader_src.spv")).to_vec();
    shader_bytes.resize((shader_bytes.len()+3) / 4 * 4, 0);
    let shader_words: Vec<u32> = shader_bytes.chunks_exact(4).map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap())).collect();
    let module = spirv::Module::from_words(&shader_words);

    // Parse a SPIR-V module
    let mut ast = spirv::Ast::<hlsl::Target>::parse(&module).expect("Failed to parse the spv!");
    let mut options = hlsl::CompilerOptions::default();
    options.shader_model = hlsl::ShaderModel::V5_1;
    options.point_size_compat = false;
    options.point_coord_compat = false;
    options.vertex = hlsl::CompilerVertexOptions::default();
    options.force_storage_buffer_as_uav = false;
    options.nonwritable_uav_texture_as_srv = false;
    options.force_zero_initialized_variables = true;

    let mut vertex_attribute_remap = HashMap::<String, Vec<HlslVertexAttributeRemap>>::new();
    vertex_attribute_remap.insert(
        String::from("main_vs"),
        vec![
            HlslVertexAttributeRemap{location : 0, semantic : String::from("POSITION"), },
        ]
    );

    for entry_point in &ast.get_entry_points().unwrap() {
        options.entry_point = Some((entry_point.name.clone(), entry_point.execution_model));
        ast.set_compiler_options(&options).expect("Failed to set the hlsl compile options!");
        if let Some(attribute_remap) = vertex_attribute_remap.get(&entry_point.name) {
            for remap in attribute_remap {
                ast.add_vertex_attribute_remap(&remap).expect("Failed to add vertex attribute remap!");
            }
        }
        let filepath = format!("target/shader.{}.hlsl", entry_point.name);
        let mut file = File::create(filepath).expect("Failed to open the hlsl output file!");
        write!(file, "{}", ast.compile().expect("Failed to compile to hlsl!")).expect("Failed to write hlsl to the file!");
    }

}
