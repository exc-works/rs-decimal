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
    let mut code = String::from(
        "fn run_examples() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {\n",
    );
    code.push_str("let mut passed = 0usize;\n");
    let mut live_count = 0;
    for path in &paths {
        let data: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        assert_eq!(data["schema_version"], 1);
        let id = data["id"].as_str().unwrap();
        assert_eq!(path.file_stem().unwrap().to_str().unwrap(), id);
        let live = match data.get("native_profile") {
            None => false,
            Some(profile) => {
                assert_eq!(profile, "postgres-live");
                true
            }
        };
        if live {
            assert!(data["javascript"].as_str().unwrap().trim().is_empty());
            live_count += 1;
            code.push_str("#[cfg(feature = \"postgres-live\")]\n");
        }
        let rust = data["rust"].as_str().unwrap();
        let expected = serde_json::to_string(&data["expected"]).unwrap();
        code.push_str(&format!("{{ {rust}\nlet expected:serde_json::Value=serde_json::from_str({expected:?})?; assert_eq!(snapshot(&value),expected,{id:?}); passed += 1; }}\n"));
    }
    code.push_str(&format!(
        "let skipped = if cfg!(feature=\"postgres-live\") {{ 0 }} else {{ {live_count} }}; println!(\"{{passed}} native documentation examples passed; {{skipped}} PostgreSQL examples require --features postgres-live and DATABASE_URL\"); Ok(()) }}"
    ));
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("examples.rs"),
        code,
    )
    .unwrap();
}
