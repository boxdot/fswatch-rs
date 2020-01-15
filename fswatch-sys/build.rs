use cc::Build;
use std::{env, path::PathBuf};

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap();
    let root_dir = project_dir.parent().unwrap();
    let src = root_dir.join("vendor/fswatch/libfswatch/src/libfswatch");

    Build::new()
        .file(src.join("c/cevent.cpp"))
        .file(src.join("c/libfswatch.cpp"))
        .file(src.join("c/libfswatch_log.cpp"))
        .file(src.join("c++/event.cpp"))
        .file(src.join("c++/filter.cpp"))
        .file(src.join("c++/libfswatch_exception.cpp"))
        .file(src.join("c++/monitor.cpp"))
        .file(src.join("c++/monitor_factory.cpp"))
        .file(src.join("c++/path_utils.cpp"))
        .file(src.join("c++/poll_monitor.cpp"))
        .file(src.join("c++/string/string_utils.cpp"))
        .include(&src)
        .include(&src.join("c"))
        .include(&src.join("c++"))
        .cpp(true)
        .flag("-std=c++11")
        .define("HAVE_STRUCT_STAT_ST_MTIME", None)
        .warnings(false)
        .compile("libfswatch");
}
