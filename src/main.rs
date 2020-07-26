// Copyright © 2020 Mark Summerfield. All rights reserved.
// Licensed under the Apache License, Version 2.0.
#![windows_subsystem = "windows"]

mod action;
mod dialog;
mod prelude;

use crate::action::maybe_save;
use crate::dialog::Dialog;
use iup::{set_library_path, IUP};

fn main() {
    set_library_path();
    let dialog = Dialog::new();
    if IUP.show_xy(dialog.dialog, iup::MOUSEPOS, iup::MOUSEPOS) {
        IUP.main_loop();
        maybe_save(dialog.dialog);
    } else {
        println!("Failed to show main window");
    }
    IUP.close();
}

