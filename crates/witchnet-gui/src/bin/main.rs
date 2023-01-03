use std::env;

use witchnet_gui::interface::app;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    app::app(args)
}