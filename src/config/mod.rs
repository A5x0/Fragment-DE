use std::path::PathBuf;

fn config_path() -> PathBuf {
    let exe = std::env::current_exe().unwrap();
    exe.parent().unwrap()               // target/debug/
        .parent().unwrap()              // target/
        .parent().unwrap()              // project root
        .join("config/config.yaml")
}

let path = config_path();
let contents = std::fs::read_to_string(&path)
    .expect(&format!("Failed to read config at {:?}", path));
