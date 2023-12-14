use std::path::Path;

use anyhow::Context;

pub fn normalized_name(name: &str) -> String {
    name.split('_').collect::<Vec<_>>().join("/")
}

pub async fn create_subfolders_and_add_suffix(
    input_path: &Path,
    output_path: &Path,
    suffix: &str,
) -> anyhow::Result<()> {
    // traverse input path for all .shader files, not .shader.meta, and make a subdirectory in
    // output path by splitting the name by _, so HDRP_Lit.shader will have a folder in output path
    // called HDRP/Lit

    let mut entries = tokio::fs::read_dir(input_path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
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

        if !file_name.ends_with(".shader") {
            continue;
        }
        let Some(shader_name) = file_name.split('.').next() else {
            continue;
        };

        add_suffix(
            std::path::Path::new(input_path).join(file_name).as_path(),
            suffix,
        )
        .await?;

        let output_path = output_path.join(normalized_name(shader_name));
        tokio::fs::create_dir_all(&output_path).await?;
    }

    Ok(())
}

async fn add_suffix(input_path: &Path, suffix: &str) -> anyhow::Result<()> {
    // in the first line of every file, it looks like Shader "Custom/LightningBoltShaderMesh" {
    // change it to Shader "Custom/LightningBoltShaderMesh1" { where 1 is the suffix

    let file = tokio::fs::read_to_string(input_path).await?;
    let mut lines = file.lines();
    let first_line = lines.next().context("File is empty")?;
    let mut first_line = first_line.split('"');
    let mut shader_name = first_line
        .nth(1)
        .context("Shader name not found")?
        .to_owned();
    shader_name = format!("{shader_name}{suffix}");

    let mut output = format!("Shader \"{shader_name}\"");

    for line in lines {
        output = format!("{output}\n{line}");
    }

    tokio::fs::write(input_path, output).await?;

    Ok(())
}
