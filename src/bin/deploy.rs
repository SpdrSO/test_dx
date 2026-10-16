use std::{
    fs::{remove_dir, remove_dir_all},
    path::Path,
    process::Command,
};

use fs_extra::dir::{move_dir, CopyOptions};

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

    move_dir(
        "docs/public",
        "docs",
        &CopyOptions::new().content_only(true),
    )
    .expect("failed to move dir");

    remove_dir("docs/public").expect("failed to remove docs/public");
}
