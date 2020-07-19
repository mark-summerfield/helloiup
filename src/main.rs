// Copyright © 2020 Mark Summerfield. All rights reserved.
#![windows_subsystem = "windows"]

mod app;
mod dialog;

use crate::app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
