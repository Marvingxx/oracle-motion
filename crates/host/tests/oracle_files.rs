//! A document of another Oracle app imported into Oracle Motion (engine `oracle_files`), through the
//! fully wired session: it comes in as its drawing, and follows the document's saves.

use std::path::Path;

use oracle_motion_engine::Session;
use oracle_motion_project::ItemKind;
use serde_json::json;

fn footage(s: &Session) -> oracle_motion_project::Footage {
    s.project.items.values().find_map(|it| if let ItemKind::Footage(f) = &it.kind { Some(f.clone()) } else { None }).unwrap()
}

/// An Oracle Vector logo imports as its drawing, and follows the document's saves (skipped when
/// Oracle Vector's engine isn't on this computer).
#[test]
fn an_oracle_vector_logo_imports_as_its_drawing_and_follows_its_saves() {
    let Some(cli) = oracle_suite_link::App::Vector.cli() else { return };
    let dir = std::env::temp_dir().join(format!("oracle-motion-oracle-files-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let logo = dir.join("logo.oracle-vector");
    let save = |width: u32| {
        let svg = dir.join("in.svg");
        std::fs::write(&svg, format!(r##"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="20"><rect width="{width}" height="20" fill="#d31c3b"/></svg>"##)).unwrap();
        assert!(std::process::Command::new(&cli).arg("convert").arg(&svg).arg(&logo).status().unwrap().success());
    };
    save(40);
    let mut s = oracle_motion_host::session();
    let r = s.execute("file.import", json!({"paths": [logo.to_string_lossy()]})).unwrap();
    assert!(r["errors"].as_array().is_none_or(Vec::is_empty), "{r}");
    let f = footage(&s);
    assert_eq!(Path::new(&f.path).extension().and_then(|e| e.to_str()), Some("svg"), "{}", f.path);
    assert_eq!(oracle_suite_link::stable_source(Path::new(&f.path)), Some(std::path::absolute(&logo).unwrap()));
    assert_eq!(f.width, 40);
    // The watch sees the file once, then the document saved wider.
    s.execute("footage.reloadChanged", json!({})).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(30));
    save(80);
    s.execute("footage.reloadChanged", json!({})).unwrap();
    assert_eq!(footage(&s).width, 80);
    let _ = std::fs::remove_dir_all(&dir);
}
