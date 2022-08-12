use std::cell::RefCell;
use std::rc::Rc;

use bitcoin_terminal_dashboard::app::App;
use bitcoin_terminal_dashboard::start_ui;
use eyre::Result;

fn main() -> Result<()> {
    let app = Rc::new(RefCell::new(App::new()));
    start_ui(app)?;
    Ok(())
}
