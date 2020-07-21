#!/usr/bin/env python3
# Copyright © 2020 Mark Summerfield. All rights reserved.
# Licensed under the Apache License, Version 2.0.

import contextlib
import os
import pathlib
import shutil
import subprocess
import sys

APP = pathlib.Path(__file__).resolve().parent.name
WIN = sys.platform != 'linux'
DIST = 'dist/' + ('windows' if WIN else 'linux')
ARGS = ['cargo', 'build', '--release', '--'] + sys.argv[1:]

if WIN:
    TARGET_DIR = f'C:\\tmp\\targets\\{APP}'
    EXE = f'{APP}.exe'
    pathlib.Path(TARGET_DIR).mkdir(exist_ok=True)
    with contextlib.suppress(FileNotFoundError):
        (pathlib.Path(TARGET_DIR) / EXE).unlink()
    env = os.environ.copy()
    env['CARGO_TARGET_DIR'] = TARGET_DIR
    reply = subprocess.run(ARGS, env=env)
else:
    reply = subprocess.run(ARGS)
if reply.returncode == 0: # built
    shutil.rmtree(DIST, ignore_errors=True)
    pathlib.Path(DIST).mkdir(parents=True)
    if WIN:
        shutil.copy2(f'{TARGET_DIR}\\release\\{EXE}', DIST)
        iup = r'R:\rs\iup\iup\windows'
        shutil.copytree(iup, f'{DIST}/iup/windows/')
        exe = f'{DIST}/{EXE}'
        try:
            subprocess.run(['rcedit.exe', exe, '--set-icon',
                            'images\\icon.ico'])
        except FileNotFoundError:
            print("Can't find rcedit.exe so can't add icon to .exe")
        subprocess.run(exe)
    else:
        shutil.copy2(f'target/release/{APP}', DIST)
        iup = pathlib.Path.home() / 'app/rs/iup/iup/linux'
        shutil.copytree(iup, f'{DIST}/iup/linux/')
        subprocess.run(f'{DIST}/{APP}')
