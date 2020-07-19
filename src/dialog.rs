// Copyright © 2020 Mark Summerfield. All rights reserved.

use iup::{Ihandle, IUP};

const DIALOG: &str = "DIALOG";
const LABEL: &str = "LABEL";
const TIMER: &str = "TIMER";
const TOGGLE: &str = "TOGGLE";
const ON: i32 = 1;
const OFF: i32 = 0;

pub struct Dialog {
    pub dialog: *mut Ihandle,
    label: *mut Ihandle,
    ok_button: *mut Ihandle,
    version_button: *mut Ihandle,
    quit_button: *mut Ihandle,
    timer: *mut Ihandle,
}

impl Dialog {
    pub fn empty() -> Self {
        Dialog {
            dialog: IUP.null_ihandle(),
            label: IUP.null_ihandle(),
            ok_button: IUP.null_ihandle(),
            version_button: IUP.null_ihandle(),
            quit_button: IUP.null_ihandle(),
            timer: IUP.null_ihandle(),
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
        IUP.set_attribute(self.timer, iup::NAME, &TIMER);
        IUP.set_int(self.timer, &TOGGLE, OFF);
        IUP.set_int(self.timer, iup::TIME, 500); // ½sec
        IUP.set_callback(self.timer, iup::ACTION_CB, on_timer);
    }

    fn make_widgets(&mut self) {
        self.label = IUP.label("Hello Rust IUP");
        IUP.set_attribute(self.label, iup::NAME, &LABEL);
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
        IUP.set_attribute(self.dialog, iup::NAME, &DIALOG);
        IUP.set_attribute(self.dialog, iup::TITLE, "Hello Rust IUP");
    }

    fn make_bindings(&mut self) {
        IUP.set_callback(self.ok_button, iup::ACTION, on_ok);
        IUP.set_callback(self.version_button, iup::ACTION, on_version);
        IUP.set_callback(self.quit_button, iup::ACTION, on_quit);
    }
}

extern "C" fn on_ok(ih: *mut Ihandle) -> i32 {
    let dialog = IUP.get_dialog_child(ih, &DIALOG);
    let label = IUP.get_dialog_child(ih, &LABEL);
    let timer = IUP.get_dialog_child(ih, &TIMER);
    let toggle = IUP.get_int(timer, &TOGGLE);
    println!("toggle={:?} toggle == ON={:?}", toggle, toggle == ON); // TODO
    if toggle == ON {
        IUP.set_int(timer, &TOGGLE, OFF);
        IUP.set_attribute(timer, iup::RUN, iup::NO);
    } else {
        IUP.set_int(timer, &TOGGLE, ON);
        IUP.set_attribute(timer, iup::RUN, iup::YES);
    }
    IUP.set_attribute(label, iup::TITLE, if toggle == ON { "Timer ON" }
                                         else { "Timer OFF" });
    IUP.message("OK — Hello", "Changed label");
    IUP.set_attribute(dialog, iup::BRINGFRONT, iup::YES);
    iup::DEFAULT
}

extern "C" fn on_timer(ih: *mut Ihandle) -> i32 {
    println!("on_timer"); // TODO not being called
    let label = IUP.get_dialog_child(ih, &LABEL);
    match IUP.get_attribute(label, iup::TITLE) {
        Some(text) => {
            let title = if text.starts_with("Timer") {
                "|".to_string()
            } else {
                format!("{}|", text)
            };
            IUP.set_attribute(label, iup::TITLE, &title);
        }
        None => println!("Failed to retrieve label text"),
    }
    iup::DEFAULT
}

extern "C" fn on_version(_ih: *mut Ihandle) -> i32 {
    IUP.version_show();
    iup::DEFAULT
}

pub fn save() {
    println!("save()");
}

extern "C" fn on_quit(_ih: *mut Ihandle) -> i32 { iup::CLOSE }
