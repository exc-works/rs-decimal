use std::{env, fs, path::PathBuf};
fn main() {
    let directory =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../website/examples");
    println!("cargo:rerun-if-changed={}", directory.display());
    let mut paths: Vec<_> = fs::read_dir(directory)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.extension().is_some_and(|x| x == "json") && p.file_name().unwrap() != "index.json"
        })
        .collect();
    paths.sort();
    let mut code = String::from("fn run_examples() -> Result<(), Box<dyn std::error::Error>> {\n");
    for path in &paths {
        let data: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        assert_eq!(data["schema_version"], 1);
        let id = data["id"].as_str().unwrap();
        assert_eq!(path.file_stem().unwrap().to_str().unwrap(), id);
        let rust = data["rust"].as_str().unwrap();
        let expected = serde_json::to_string(&data["expected"]).unwrap();
        code.push_str(&format!("{{ {rust}\nlet expected:serde_json::Value=serde_json::from_str({expected:?})?; assert_eq!(snapshot(&value),expected,{id:?}); }}\n"));
    }
    code.push_str(&format!(
        "println!(\"{} native documentation examples passed\"); Ok(()) }}",
        paths.len()
    ));
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("examples.rs"),
        code,
    )
    .unwrap();
}
