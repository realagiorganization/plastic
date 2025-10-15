use std::fs;
use std::path::Path;

use predicates::prelude::*;
use tempfile::TempDir;

fn setup_temp_root() -> TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    let roms_dir = tmp.path().join("test_roms");
    fs::create_dir_all(&roms_dir).expect("create test_roms");
    fs::write(roms_dir.join("alpha.nes"), [0u8; 16]).expect("write alpha");
    fs::create_dir_all(roms_dir.join("nested")).expect("nested");
    fs::write(roms_dir.join("nested").join("beta.nes"), [1u8; 32]).expect("write beta");
    tmp
}

#[cfg_attr(
    target_os = "windows",
    ignore = "Windows symlinks often require privileges"
)]
#[test]
fn links_rom_with_relative_path() {
    let tmp = setup_temp_root();
    let root = tmp.path();

    let mut cmd = assert_cmd::Command::cargo_bin("cargo-rom").expect("binary");
    cmd.arg("link")
        .arg("test_roms/alpha.nes")
        .env("PLASTIC_ROM_ROOT", root);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Linked"));

    assert_link_points_to(
        &root.join("test_roms/.startup.nes"),
        &root.join("test_roms/alpha.nes"),
    );
}

#[cfg_attr(
    target_os = "windows",
    ignore = "Windows symlinks often require privileges"
)]
#[test]
fn can_relink_to_different_rom_by_name() {
    let tmp = setup_temp_root();
    let root = tmp.path();
    let link_path = root.join("test_roms/.startup.nes");

    let mut first = assert_cmd::Command::cargo_bin("cargo-rom").expect("binary");
    first
        .arg("link")
        .arg("alpha.nes")
        .env("PLASTIC_ROM_ROOT", root);
    first.assert().success();

    assert_link_points_to(&link_path, &root.join("test_roms/alpha.nes"));

    let mut second = assert_cmd::Command::cargo_bin("cargo-rom").expect("binary");
    second
        .arg("link")
        .arg("beta.nes")
        .env("PLASTIC_ROM_ROOT", root);
    second.assert().success();

    assert_link_points_to(&link_path, &root.join("test_roms/nested/beta.nes"));
}

#[test]
fn errors_for_missing_rom() {
    let tmp = setup_temp_root();
    let root = tmp.path();

    let mut cmd = assert_cmd::Command::cargo_bin("cargo-rom").expect("binary");
    cmd.arg("link")
        .arg("missing.nes")
        .env("PLASTIC_ROM_ROOT", root);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not find ROM"));
}

#[cfg(unix)]
fn assert_link_points_to(link: &Path, target: &Path) {
    let metadata = fs::symlink_metadata(link).expect("link metadata");
    assert!(
        metadata.file_type().is_symlink(),
        ".startup.nes should be a symlink"
    );
    let linked = fs::read_link(link).expect("read link");
    assert_eq!(
        linked,
        target.canonicalize().expect("canonical target"),
        "link should target rom"
    );
}

#[cfg(not(unix))]
fn assert_link_points_to(link: &Path, target: &Path) {
    // On non-Unix platforms (e.g. Windows without privileges) the command may fall back to copying.
    let metadata = fs::metadata(link).expect("metadata");
    assert!(
        metadata.is_file(),
        ".startup.nes should exist as a regular file"
    );
    let expected = fs::read(target).expect("read target");
    let actual = fs::read(link).expect("read link");
    assert_eq!(actual, expected, "link file contents should match target");
}
