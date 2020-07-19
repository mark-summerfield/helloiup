// Copyright © 2020 Mark Summerfield. All rights reserved.

mod app;
mod dialog;

use crate::app::App;
use std::ptr;

static mut _APP: *mut App = ptr::null_mut();

fn app() -> &'static mut App { unsafe { &mut *_APP } }

fn main() {
    let mut _app = App::new();
    unsafe { _APP = &mut _app; }
    app().run();
}
