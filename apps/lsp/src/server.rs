use std::error::Error;

use lsp_server::Connection;
use palmscript::IdeLspSession;

pub fn run() -> Result<(), Box<dyn Error>> {
    let (connection, io_threads) = Connection::stdio();
    let mut session = IdeLspSession::new();

    for message in &connection.receiver {
        for outbound in session.handle_message(message.clone()) {
            connection.sender.send(outbound)?;
        }
        if session.should_exit() {
            break;
        }
    }

    io_threads.join()?;
    Ok(())
}
