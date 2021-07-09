use std::error::Error;
use std::env;
use crate::app::App;

mod models;
mod app;

//sudo setcap CAP_SYS_TIME+ep ./time_machine

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let app = App::new()?;
    app.parse_arguments(&args)?;

    Ok(())
}