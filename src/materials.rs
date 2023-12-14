use std::path::Path;

use crate::guid;

pub async fn copy_materials_to_shader(
    input_shader_path: &Path,
    output_shader_path: &Path,
    input_material_path: &Path,
) -> anyhow::Result<()> {
    let input_path = std::path::Path::new(input_material_path);

    let mut input_path = tokio::fs::read_dir(input_path).await?;

    let guid_table = guid::build_guid_table(input_shader_path, output_shader_path).await?;

    while let Some(path) = input_path.next_entry().await? {
        let path = path.path();
        if !path.is_file() {
            continue;
        }

        let Some(file_name) = path.file_name() else {
            println!("File {path:?} has no file name, skipping");
            continue;
        };

        let Some(file_name) = file_name.to_str() else {
            println!("File name is not valid UTF-8: {file_name:?}, skipping");
            continue;
        };

        if !file_name.ends_with(".mat") {
            continue;
        }

        let file = tokio::fs::read_to_string(&path).await?;

        let mut lines = file.lines();

        let mut guid = None;

        for line in &mut lines {
            if let Some(stripped) = line.strip_prefix("  m_Shader: {fileID: 4800000, guid: ") {
                guid = Some(stripped.split(',').next().unwrap().to_string());
                break;
            }
        }

        let Some(guid) = guid else {
            println!("Warning: file {file_name:?} has no guid, skipping");
            continue;
        };

        let Some(output_path) = guid_table.get(&guid) else {
            println!("Warning: guid {guid:?} not found in shader's guid table, skipping");
            continue;
        };

        let output_path = output_path.join(file_name);
        let output_path = output_path.as_path();

        tokio::fs::copy(&path, output_path).await?;

        let meta_path = path.with_extension("mat.meta");
        let output_meta_path = output_path.with_extension("mat.meta");

        tokio::fs::copy(&meta_path, &output_meta_path).await?;
    }

    Ok(())
}
