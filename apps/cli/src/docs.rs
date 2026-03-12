use crate::args::DocsArgs;

#[derive(Clone, Copy, Debug)]
pub struct EmbeddedDoc {
    pub topic: &'static str,
    pub title: &'static str,
    pub relative_path: &'static str,
    pub body: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/embedded_docs.rs"));

pub fn render(args: &DocsArgs) -> Result<String, String> {
    if args.all {
        return Ok(render_all());
    }
    if args.list {
        return Ok(render_list());
    }
    if let Some(topic) = args.topic.as_deref() {
        let doc = find_doc(topic)?;
        return Ok(render_single(doc));
    }
    Ok(render_usage())
}

fn render_usage() -> String {
    let mut output = String::new();
    output.push_str("PalmScript embedded English docs\n");
    output.push_str("usage:\n");
    output.push_str("  palmscript docs --list\n");
    output.push_str("  palmscript docs <topic>\n");
    output.push_str("  palmscript docs --all\n\n");
    output
        .push_str("`--all` prints the full embedded public English docs snapshot in one stream.\n");
    output.push_str("Use `--list` first when you need the exact topic path.\n\n");
    output.push_str(&render_list());
    output
}

fn render_list() -> String {
    let mut output = String::new();
    output.push_str(&format!("doc_count={}\n", EMBEDDED_DOCS.len()));
    for doc in EMBEDDED_DOCS {
        output.push_str(&format!(
            "topic={} title={:?} path={}\n",
            doc.topic, doc.title, doc.relative_path
        ));
    }
    output
}

fn render_single(doc: &EmbeddedDoc) -> String {
    let mut output = String::new();
    output.push_str(&format!(
        "topic={}\ntitle={}\npath={}\n\n",
        doc.topic, doc.title, doc.relative_path
    ));
    output.push_str(doc.body);
    if !doc.body.ends_with('\n') {
        output.push('\n');
    }
    output
}

fn render_all() -> String {
    let mut output = String::new();
    output.push_str("PalmScript embedded English docs\n");
    output.push_str(&format!("doc_count={}\n\n", EMBEDDED_DOCS.len()));
    output.push_str("topics:\n");
    for doc in EMBEDDED_DOCS {
        output.push_str(&format!("- {} :: {}\n", doc.topic, doc.title));
    }
    output.push('\n');
    for doc in EMBEDDED_DOCS {
        output.push_str(&format!("===== BEGIN DOC {} =====\n", doc.topic));
        output.push_str(&format!(
            "title={}\npath={}\n\n",
            doc.title, doc.relative_path
        ));
        output.push_str(doc.body);
        if !doc.body.ends_with('\n') {
            output.push('\n');
        }
        output.push_str(&format!("===== END DOC {} =====\n\n", doc.topic));
    }
    output
}

fn find_doc(topic: &str) -> Result<&'static EmbeddedDoc, String> {
    EMBEDDED_DOCS
        .iter()
        .find(|doc| doc.topic == topic)
        .ok_or_else(|| {
            format!(
                "unknown docs topic `{topic}`. Use `palmscript docs --list` to inspect embedded topics."
            )
        })
}
