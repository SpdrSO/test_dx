use std::{
    fs::{read_dir, remove_dir, remove_dir_all},
    path::{Path, PathBuf},
    process::Command,
};

use fs_extra::{dir::CopyOptions, move_items};

fn main() {
    if Path::new("docs").exists() {
        remove_dir_all("docs").expect("failed to remove docs");
    }

    Command::new("dx")
        .arg("bundle")
        .arg("--release")
        .args(["--platform", "web"])
        .args(["--out-dir", "docs"])
        .status()
        .expect("failed to run dx");

    let items: Vec<PathBuf> = read_dir("docs/public")
        .expect("failed to read docs/public")
        .map(|entry| entry.expect("failed to read dir entry").path())
        .collect();

    let options = CopyOptions::new();
    move_items(&items, "docs", &options).expect("failed to move dir");

    remove_dir("docs/public").expect("failed to remove docs/public");
}
