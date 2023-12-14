use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::Lines,
};

use crate::shaders::normalized_name;

fn get_guid(lines: &mut Lines) -> Option<String> {
    for line in lines {
        if let Some(stripped) = line.strip_prefix("guid: ") {
            return Some(stripped.trim().to_string());
        }
    }

    None
}

pub async fn build_guid_table(
    input_shader_path: impl AsRef<Path>,
    output_shader_path: impl AsRef<Path>,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    let mut table = HashMap::new();

    let mut entries = tokio::fs::read_dir(input_shader_path).await?;

    while let Some(entry) = entries.next_entry().await? {
        if !entry.file_type().await?.is_file() {
            continue;
        }

        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();

        if !file_name.ends_with(".meta") {
            continue;
        }

        let file = tokio::fs::read_to_string(entry.path()).await?;

        let Some(guid) = get_guid(&mut file.lines()) else {
            println!("Warning: file {file_name:?} has no guid, skipping");
            continue;
        };

        let shader_name = file_name.split('.').next().unwrap();
        let shader_name = normalized_name(shader_name);

        let shader_path = output_shader_path.as_ref().join(shader_name);

        table.insert(guid, shader_path);
    }

    Ok(table)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[tokio::test]
    async fn test_build_guid_table() {
        let yaml = indoc! {"
            fileFormatVersion: 2
            guid: e8c0f036827360544a6df55e8d2866c8
            timeCreated: 1702147913
            licenseType: Free
            ShaderImporter:
              externalObjects: {}
              defaultTextures: []
              nonModifiableTextures: []
              userData:
              assetBundleName:
              assetBundleVariant:
        "};

        assert_eq!(
            get_guid(&mut yaml.lines()),
            Some("e8c0f036827360544a6df55e8d2866c8".to_string())
        );
    }
}
