// Copyright © 2020 Mark Summerfield. All rights reserved.

use iup::{Ihandle, IUP};
use std::ffi::c_void;

const DIALOG: &str = ":DIALOG";
const LABEL: &str = ":LABEL";
const TIMER: &str = ":TIMER";
const TICKER: &str = ":TICKER";
const TICKING_ON: i32 = 1;
const TICKING_OFF: i32 = 0;

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
    pub fn empty() -> Self {
        Dialog {
            dialog: IUP.null_ihandle(),
            label: IUP.null_ihandle(),
            ticker_button: IUP.null_ihandle(),
            save_button: IUP.null_ihandle(),
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
        self.make_attributes();
    }

    fn make_timer(&mut self) {
        self.timer = IUP.timer();
        IUP.set_int(self.timer, &TICKER, TICKING_OFF);
        IUP.set_int(self.timer, iup::TIME, 300); // 300ms
        IUP.set_callback(self.timer, iup::ACTION_CB, on_timer);
    }

    fn make_widgets(&mut self) {
        self.label = IUP.label("Hello Rust IUP");
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
    }

    fn make_bindings(&mut self) {
        IUP.set_callback(self.ticker_button, iup::ACTION, on_ticker);
        IUP.set_callback(self.save_button, iup::ACTION, on_save);
        IUP.set_callback(self.version_button, iup::ACTION, on_version);
        IUP.set_callback(self.quit_button, iup::ACTION, on_quit);
    }

    fn make_attributes(&mut self) {
        IUP.set_attribute_ptr(self.dialog, &LABEL,
                              self.label as *mut c_void);
        IUP.set_attribute_ptr(self.dialog, &TIMER,
                              self.timer as *mut c_void);
        IUP.set_attribute_ptr(self.timer, &LABEL,
                              self.label as *mut c_void);
    }
}

pub fn save() {
    println!("save()");
}

extern "C" fn on_ticker(ih: *mut Ihandle) -> i32 {
    let dialog = IUP.get_dialog_child(ih, &DIALOG);
    let label = IUP.get_attribute_ptr(ih, &LABEL) as *mut Ihandle;
    let timer = IUP.get_attribute_ptr(ih, &TIMER) as *mut Ihandle;
    let mut ticking = IUP.get_int(timer, &TICKER);
    if ticking == TICKING_ON {
        IUP.set_attribute(timer, iup::RUN, iup::NO);
        IUP.set_int(timer, &TICKER, TICKING_OFF);
    } else {
        IUP.set_attribute(timer, iup::RUN, iup::YES);
        IUP.set_int(timer, &TICKER, TICKING_ON);
    }
    ticking = !ticking;
    IUP.set_attribute(label, iup::TITLE,
                      if ticking == TICKING_ON { "Timer ON" }
                      else { "Timer OFF" });
    IUP.message("OK — Hello", "Changed label");
    IUP.set_attribute(dialog, iup::BRINGFRONT, iup::YES);
    iup::DEFAULT
}

extern "C" fn on_timer(ih: *mut Ihandle) -> i32 {
    let label = IUP.get_attribute_ptr(ih, &LABEL) as *mut Ihandle;
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

extern "C" fn on_save(_ih: *mut Ihandle) -> i32 {
    save();
    iup::DEFAULT
}

extern "C" fn on_version(_ih: *mut Ihandle) -> i32 {
    IUP.version_show();
    iup::DEFAULT
}

extern "C" fn on_quit(_ih: *mut Ihandle) -> i32 { iup::CLOSE }
