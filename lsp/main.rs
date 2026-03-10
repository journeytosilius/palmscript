mod server;

fn main() {
    if let Err(err) = server::run() {
        eprintln!("palmscript-lsp: {err}");
        std::process::exit(1);
    }
}
