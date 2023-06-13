use spirv_builder::{MetadataPrintout, SpirvBuilder, SpirvMetadata};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shader_crate = "shader_src";
    let target = "spirv-unknown-spv1.5";
    SpirvBuilder::new(shader_crate, target)
        .spirv_metadata(SpirvMetadata::Full)
        .print_metadata(MetadataPrintout::Full)
        .build()?;

    Ok(())
}
