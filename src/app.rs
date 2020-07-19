// Copyright © 2020 Mark Summerfield. All rights reserved.

use crate::dialog::Dialog;
use iup::IUP;

pub struct App { pub dialog: Dialog }

impl App {
    pub fn new() -> Self { App { dialog: Dialog::new() } }

    pub fn run(&mut self) {
        // could read size/pos from config
        self.dialog.build();
        if IUP.show_xy(self.dialog.dialog, iup::MOUSEPOS, iup::MOUSEPOS) {
            IUP.main_loop();
            self.dialog.save();
        } else {
            println!("Failed to show main window");
        }
        IUP.close();
    }
}
