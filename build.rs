use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create Discount build directory");
    for entry in fs::read_dir(source).expect("read vendored Discount source") {
        let entry = entry.expect("read Discount source entry");
        let target = destination.join(entry.file_name());
        if entry
            .file_type()
            .expect("read Discount entry type")
            .is_dir()
        {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).expect("copy Discount source file");
        }
    }
}

fn run(command: &mut Command, description: &str) {
    let status = command.status().unwrap_or_else(|error| {
        panic!("failed to {description}: {error}");
    });
    assert!(status.success(), "failed to {description}: {status}");
}

fn main() {
    let source = Path::new("vendor/discount-2.2.7d");
    let output = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set"));
    let build = output.join("discount-2.2.7d");

    if build.exists() {
        fs::remove_dir_all(&build).expect("remove old Discount build directory");
    }
    copy_tree(source, &build);

    let mut configure = Command::new("sh");
    configure
        .arg("configure.sh")
        .current_dir(&build)
        .env("CFLAGS", "-O3 -DNDEBUG");
    run(&mut configure, "configure Discount");

    let mut make = Command::new(env::var_os("MAKE").unwrap_or_else(|| "make".into()));
    make.arg("libmarkdown").current_dir(&build);
    run(&mut make, "build Discount");

    println!("cargo:rustc-link-search=native={}", build.display());
    println!("cargo:rustc-link-lib=static=markdown");
    println!("cargo:rerun-if-changed={}", source.display());
}
