// Copyright © 2020 Mark Summerfield. All rights reserved.
// Licensed under the Apache License, Version 2.0.

use crate::prelude::*;
use iup::{Ihandle, IUP};

pub(crate) extern "C" fn on_ticker(ih: *mut Ihandle) -> i32 {
    let dialog = IUP.get_dialog_child(ih, &DIALOG);
    let label = IUP.get_ih(ih, &LABEL);
    let timer = IUP.get_ih(ih, &TIMER);
    let mut ticking = IUP.get_int(timer, &TICKER);
    if ticking == TRUE {
        IUP.set_attribute(timer, iup::RUN, iup::NO);
        IUP.set_int(timer, &TICKER, FALSE);
    } else {
        IUP.set_attribute(timer, iup::RUN, iup::YES);
        IUP.set_int(timer, &TICKER, TRUE);
    }
    ticking = !ticking;
    IUP.set_attribute(label, iup::TITLE,
                      if ticking == TRUE { "Timer ON" }
                      else { "Timer OFF" });
    IUP.message("OK — Hello", "Changed label");
    IUP.set_attribute(dialog, iup::BRINGFRONT, iup::YES);
    iup::DEFAULT
}

pub(crate) extern "C" fn on_timer(ih: *mut Ihandle) -> i32 {
    let label = IUP.get_ih(ih, &LABEL);
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

pub(crate) extern "C" fn maybe_save(ih: *mut Ihandle) -> i32 {
    let dialog = IUP.get_dialog_child(ih, &DIALOG);
    if IUP.get_int(dialog, &UNSAVED_CHANGES) == TRUE {
        println!("save()");
        IUP.set_int(dialog, &UNSAVED_CHANGES, FALSE);
    }
    iup::DEFAULT
}

pub(crate) extern "C" fn on_version(_ih: *mut Ihandle) -> i32 {
    IUP.version_show();
    iup::DEFAULT
}

pub(crate) extern "C" fn on_quit(_ih: *mut Ihandle) -> i32 { iup::CLOSE }
