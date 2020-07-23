// Copyright © 2020 Mark Summerfield. All rights reserved.
// Licensed under the Apache License, Version 2.0.

use crate::prelude::*;
use crate::action::{on_ticker, on_timer, maybe_save, on_version, on_quit};
use iup::{Ihandle, IM, IUP};
use std::env;
use std::path::PathBuf;

pub struct Dialog {
    pub dialog: *mut Ihandle,
    label: *mut Ihandle,
    ticker_button: *mut Ihandle,
    save_button: *mut Ihandle,
    version_button: *mut Ihandle,
    quit_button: *mut Ihandle,
    timer: *mut Ihandle,
}

impl Dialog {
    pub fn new() -> Self {
        let mut dialog = Dialog {
            dialog: IUP.null_ihandle(),
            label: IUP.null_ihandle(),
            ticker_button: IUP.null_ihandle(),
            save_button: IUP.null_ihandle(),
            version_button: IUP.null_ihandle(),
            quit_button: IUP.null_ihandle(),
            timer: IUP.null_ihandle(),
        };
        dialog.make_widgets();
        dialog.make_layout();
        dialog.make_bindings();
        dialog.make_timer();
        dialog.make_attributes();
        dialog
    }

    fn make_widgets(&mut self) {
        let system = IUP.get_global(iup::SYSTEM);
        let system_version = IUP.get_global(iup::SYSTEMVERSION);
        self.label = IUP.label(&format!("Hello Rust IUP on\n{} {}",
                                        &system, &system_version));
        self.ticker_button = IUP.button("&Ticker", "");
        self.save_button = IUP.button("&Save", "");
        self.version_button = IUP.button("&Version", "");
        self.quit_button = IUP.button("&Quit", "");
    }

    fn make_layout(&mut self) {
        let hbox = IUP.hbox();
        IUP.append(hbox, self.ticker_button);
        IUP.append(hbox, self.save_button);
        IUP.append(hbox, self.version_button);
        IUP.append(hbox, self.quit_button);
        let vbox = IUP.vbox();
        IUP.append(vbox, self.label);
        IUP.append(vbox, hbox);
        self.dialog = IUP.dialog(vbox);
        IUP.set_attribute(self.dialog, iup::NAME, &DIALOG);
        IUP.set_attribute(self.dialog, iup::TITLE, "Hello Rust IUP");
        let exe = env::current_exe().unwrap_or(PathBuf::from("."));
        let image = exe.parent().unwrap_or(&exe).join("images/icon.png");
        let icon = IM.load_image(&image.as_path().to_string_lossy());
    }

    fn make_bindings(&mut self) {
        IUP.set_callback(self.ticker_button, iup::ACTION, on_ticker);
        IUP.set_callback(self.save_button, iup::ACTION, maybe_save);
        IUP.set_callback(self.version_button, iup::ACTION, on_version);
        IUP.set_callback(self.quit_button, iup::ACTION, on_quit);
    }

    fn make_timer(&mut self) {
        self.timer = IUP.timer();
        IUP.set_int(self.timer, &TICKER, FALSE);
        IUP.set_int(self.timer, iup::TIME, 300); // 300ms
        IUP.set_callback(self.timer, iup::ACTION_CB, on_timer);
    }

    fn make_attributes(&mut self) {
        // NOTE In a real application this would start as FALSE, but we want
        // to be able to test it in this example application.
        IUP.set_int(self.dialog, &UNSAVED_CHANGES, TRUE);

        IUP.set_ih(self.dialog, &LABEL, self.label);
        IUP.set_ih(self.dialog, &TIMER, self.timer);
        IUP.set_ih(self.timer, &LABEL, self.label);
    }
}
