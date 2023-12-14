use std::path::PathBuf;

use anstyle::{AnsiColor, Color, Style};
use anyhow::Context;
use clap::{Args, Command, FromArgMatches as _};

mod guid;
mod materials;
mod shaders;

const BUILD_VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_SHA: Option<&'static str> = option_env!("BUILD_SHA");

#[derive(Args, Debug)]
#[command(styles=get_styles())]
struct AppArgs {
    #[arg(long, required = true)]
    shaders_path: PathBuf,

    #[arg(
        long,
        help = "output path for shaders, if not specified, will be shaders_path + \"Out\"\n"
    )]
    output_shaders_path: Option<PathBuf>,

    #[arg(long, required = true)]
    material_path: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let version = if let Some(build_sha) = BUILD_SHA {
        format!("{BUILD_VERSION} ({build_sha})")
    } else {
        BUILD_VERSION.to_string()
    };
    let version: &'static str = Box::leak(version.into_boxed_str());
    let cli = Command::new("lethalfixshaders")
        .version(version)
        .about("Fixes shader names and sorts materials into their shader path for Lethal Company");

    let cli = AppArgs::augment_args(cli);

    let matches = AppArgs::from_arg_matches(&cli.get_matches()).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    let shaders_metadata = tokio::fs::metadata(&matches.material_path)
        .await
        .with_context(|| {
            format!(
                "Failed to read shaders path {}, does it exist?",
                &matches.material_path.to_string_lossy()
            )
        })?;
    if !shaders_metadata.is_dir() {
        anyhow::bail!(
            "Input path {} is not a directory!",
            matches.material_path.to_string_lossy()
        );
    }

    let materials_metadata = tokio::fs::metadata(&matches.material_path)
        .await
        .with_context(|| {
            format!(
                "Failed to read materials path {}, does it exist?",
                &matches.material_path.to_string_lossy()
            )
        })?;
    if !materials_metadata.is_dir() {
        anyhow::bail!(
            "Input path {} is not a directory!",
            matches.material_path.to_string_lossy()
        );
    }

    let output_shaders_path = if let Some(path) = matches.output_shaders_path {
        path
    } else {
        PathBuf::from(matches.shaders_path.to_string_lossy().to_string() + "Out")
    };

    tokio::fs::create_dir_all(&output_shaders_path).await?;

    shaders::create_subfolders_and_add_suffix(&matches.shaders_path, &output_shaders_path, "1")
        .await?;

    materials::copy_materials_to_shader(
        &matches.shaders_path,
        &output_shaders_path,
        &matches.material_path,
    )
    .await?;

    Ok(())
}

#[must_use]
pub const fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
        )
        .header(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
        )
        .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .invalid(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .error(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .valid(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::White))))
}
