use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(
    name = "cargo rom",
    version,
    about = "ROM management helper for Plastic",
    author
)]
struct Cli {
    /// Override the project root (defaults to autodetect by walking up from CWD)
    #[arg(long, global = true, env = "PLASTIC_ROM_ROOT")]
    root: Option<PathBuf>,
    /// Override the link path (defaults to <root>/test_roms/.startup.nes)
    #[arg(long, global = true, env = "PLASTIC_ROM_LINK")]
    link: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// List available ROMs found under the test_roms directory
    List,
    /// Link test_roms/.startup.nes to the provided ROM
    Link {
        /// Path or name of the ROM to link
        rom: String,
    },
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let cli = Cli::parse();

    let root = if let Some(path) = cli.root {
        path
    } else {
        find_project_root().context("Failed to infer project root. Use --root.")?
    };

    let link_path = cli
        .link
        .unwrap_or_else(|| root.join("test_roms").join(".startup.nes"));

    match cli.command {
        Command::List => list_roms(&root),
        Command::Link { rom } => link_rom(&root, &link_path, &rom),
    }
}

fn list_roms(root: &Path) -> Result<()> {
    let roms_dir = root.join("test_roms");
    if !roms_dir.exists() {
        return Err(anyhow!("Expected ROM directory at {}", roms_dir.display()));
    }

    for entry in WalkDir::new(&roms_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        if entry
            .path()
            .extension()
            .and_then(OsStr::to_str)
            .map(|ext| ext.eq_ignore_ascii_case("nes"))
            .unwrap_or(false)
        {
            let rel = entry
                .path()
                .strip_prefix(root)
                .unwrap_or_else(|_| entry.path());
            println!("{}", rel.display());
        }
    }

    Ok(())
}

fn link_rom(root: &Path, link_path: &Path, rom_input: &str) -> Result<()> {
    let rom_path = resolve_rom_path(root, rom_input)?;

    if let Some(parent) = link_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to ensure {}", parent.display()))?;
    }

    if link_path.exists() || fs::symlink_metadata(link_path).is_ok() {
        fs::remove_file(link_path)
            .with_context(|| format!("Failed to remove existing {}", link_path.display()))?;
    }

    create_symlink(&rom_path, link_path)
        .with_context(|| format!("Failed to create link {}", link_path.display()))?;

    println!("Linked {} -> {}", link_path.display(), rom_path.display());

    Ok(())
}

fn resolve_rom_path(root: &Path, input: &str) -> Result<PathBuf> {
    let candidates = [
        PathBuf::from(input),
        root.join(input),
        root.join("test_roms").join(input),
    ];

    for candidate in candidates {
        if candidate.exists() {
            return candidate
                .canonicalize()
                .with_context(|| format!("Failed to canonicalize {}", candidate.display()));
        }
    }

    let roms_dir = root.join("test_roms");
    if roms_dir.exists() {
        for entry in WalkDir::new(&roms_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            if entry.file_name() == input {
                return entry
                    .path()
                    .canonicalize()
                    .with_context(|| format!("Failed to canonicalize {}", entry.path().display()));
            }
        }
    }

    Err(anyhow!(
        "Could not find ROM '{}' relative to {}",
        input,
        root.display()
    ))
}

fn find_project_root() -> Result<PathBuf> {
    let mut dir = env::current_dir().context("Failed to read current directory")?;
    loop {
        if dir.join("test_roms").is_dir() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    Err(anyhow!(
        "Unable to locate project root from current directory"
    ))
}

#[cfg(unix)]
fn create_symlink(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(src, dst)
}

#[cfg(windows)]
fn create_symlink(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(src, dst)
}
