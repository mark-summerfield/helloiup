// Copyright © 2020 Mark Summerfield. All rights reserved.

use crate::app;
use iup::{Ihandle, IUP};

pub struct Dialog {
    pub dialog: *mut Ihandle,
    unsaved_changes: bool,
    label: *mut Ihandle,
    ok_button: *mut Ihandle,
    version_button: *mut Ihandle,
    quit_button: *mut Ihandle,
    timer: *mut Ihandle,
    timer_toggle: bool,
}

impl Dialog {
    pub fn new() -> Self {
        Dialog {
            dialog: IUP.null_ihandle(),
            unsaved_changes: false,
            label: IUP.null_ihandle(),
            ok_button: IUP.null_ihandle(),
            version_button: IUP.null_ihandle(),
            quit_button: IUP.null_ihandle(),
            timer: IUP.null_ihandle(),
            timer_toggle: false,
        }
    }

    pub fn save(&mut self) {
        if self.unsaved_changes {
            println!("save()");
            self.unsaved_changes = false;
        }
    }

    pub fn build(&mut self) {
        self.make_widgets();
        self.make_layout();
        self.make_bindings();
        self.make_timer();
    }

    fn make_timer(&mut self) {
        self.timer = IUP.timer();
        IUP.set_int(self.timer, iup::TIME, 500); // ½sec
        IUP.set_callback(self.timer, iup::ACTION_CB, on_timer);
    }

    fn make_widgets(&mut self) {
        self.label = IUP.label("Hello Rust IUP");
        self.ok_button = IUP.button("&OK", "");
        self.version_button = IUP.button("&Version", "");
        self.quit_button = IUP.button("&Quit", "");
    }

    fn make_layout(&mut self) {
        let hbox = IUP.hbox();
        IUP.append(hbox, self.ok_button);
        IUP.append(hbox, self.version_button);
        IUP.append(hbox, self.quit_button);
        let vbox = IUP.vbox();
        IUP.append(vbox, self.label);
        IUP.append(vbox, hbox);
        self.dialog = IUP.dialog(vbox);
        IUP.set_attribute(self.dialog, iup::TITLE, "Hello Rust IUP");
    }

    fn make_bindings(&mut self) {
        IUP.set_callback(self.ok_button, iup::ACTION, on_ok);
        IUP.set_callback(self.version_button, iup::ACTION, on_version);
        IUP.set_callback(self.quit_button, iup::ACTION, on_quit);
    }

    fn on_ok(&mut self) -> i32 {
        if self.timer_toggle {
            self.timer_toggle = false;
            IUP.set_attribute(self.timer, iup::RUN, iup::NO);
        } else {
            self.timer_toggle = true;
            IUP.set_attribute(self.timer, iup::RUN, iup::YES);
        }
        IUP.set_attribute(self.label, iup::TITLE, 
                          if self.timer_toggle { "Timer ON" }
                          else { "Timer OFF" });
        IUP.message("OK — Hello", "Changed label");
        IUP.set_attribute(self.dialog, iup::BRINGFRONT, iup::YES);
        self.save();
        iup::DEFAULT
    }

    fn on_timer(&mut self) -> i32 {
        match IUP.get_attribute(self.label, iup::TITLE) {
            Some(text) => {
                let title = if text.starts_with("Timer") {
                    "|".to_string()
                } else {
                    format!("{}|", text)
                };
                IUP.set_attribute(self.label, iup::TITLE, &title);
            }
            None => println!("Failed to retrieve label text"),
        }
        iup::DEFAULT
    }

    fn on_version(&mut self) -> i32 {
        IUP.version_show();
        iup::DEFAULT
    }
}

extern "C" fn on_ok(_ih: *mut Ihandle) -> i32 { app().dialog.on_ok() }
extern "C" fn on_timer(_ih: *mut Ihandle) -> i32 {
    app().dialog.on_timer()
}
extern "C" fn on_version(_ih: *mut Ihandle) -> i32 {
    app().dialog.on_version()
}
extern "C" fn on_quit(_ih: *mut Ihandle) -> i32 { iup::CLOSE }
