use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const LOCALE_SUFFIXES: &[&str] = &[".es.md", ".pt-BR.md", ".de.md", ".ja.md", ".fr.md"];

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let docs_dir = manifest_dir.join("../../web/docs/docs");
    println!("cargo:rerun-if-changed={}", docs_dir.display());

    let mut docs = Vec::new();
    collect_docs(&docs_dir, &docs_dir, &mut docs).expect("collect docs");
    docs.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("out dir"));
    let generated = out_dir.join("embedded_docs.rs");
    let mut file = fs::File::create(&generated).expect("create embedded docs module");
    writeln!(file, "pub static EMBEDDED_DOCS: &[EmbeddedDoc] = &[\n").expect("write header");
    for doc in docs {
        writeln!(
            file,
            "    EmbeddedDoc {{ topic: {:?}, title: {:?}, relative_path: {:?}, body: include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/../../web/docs/docs/{}\")) }},",
            doc.topic,
            doc.title,
            doc.relative_path,
            doc.relative_path.replace('\\', "/"),
        )
        .expect("write doc entry");
    }
    writeln!(file, "];").expect("write footer");
}

fn collect_docs(root: &Path, dir: &Path, docs: &mut Vec<DocEntry>) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_docs(root, &path, docs)?;
            continue;
        }
        if !is_english_doc(&path) {
            continue;
        }
        let relative_path = path
            .strip_prefix(root)
            .map_err(|err| err.to_string())?
            .to_string_lossy()
            .replace('\\', "/");
        let body = fs::read_to_string(&path).map_err(|err| err.to_string())?;
        let title = extract_title(&body).unwrap_or_else(|| relative_path.clone());
        let topic = relative_path
            .strip_suffix(".md")
            .expect("markdown suffix")
            .to_string();
        docs.push(DocEntry {
            topic,
            title,
            relative_path,
        });
    }
    Ok(())
}

fn is_english_doc(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };
    if !file_name.ends_with(".md") {
        return false;
    }
    !LOCALE_SUFFIXES
        .iter()
        .any(|suffix| file_name.ends_with(suffix))
}

fn extract_title(body: &str) -> Option<String> {
    body.lines()
        .find_map(|line| line.strip_prefix("# ").map(str::trim))
        .filter(|title| !title.is_empty())
        .map(str::to_string)
}

struct DocEntry {
    topic: String,
    title: String,
    relative_path: String,
}
