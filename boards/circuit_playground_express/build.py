#!/usr/bin/env python3
"""
Build and deploy a Rust Circuit Playground Express app.

The deploy part will only run if the board is connected via USB cable and is in
boot loader mode.

Usage:
    build <app-name>

Where:
    There exists a Rust source file examples/<app-name>.rs
"""
import getpass
import os
import shutil
import sys
import subprocess


def cmd(args):
    print(subprocess.list2cmdline(args))
    subprocess.check_call(args)


def build(app_name):
    examples_target_dir = 'target/thumbv6m-none-eabi/release/examples'
    cmd(['cargo', 'build', '--example', app_name, '--release'])
    app_elf_path = os.path.join(examples_target_dir, app_name)
    app_bin_path = os.path.join(examples_target_dir, app_name + '.bin')
    cmd(['arm-none-eabi-objcopy', '-O', 'binary', app_elf_path, app_bin_path])
    app_uf2_path = os.path.join(examples_target_dir, app_name + '.uf2')
    cmd(['uf2conv.py', app_bin_path, '--output', app_uf2_path])
    return app_uf2_path


def deploy(app_uf2_path):
    boot_path = os.path.join('/run/media', getpass.getuser(), 'CPLAYBOOT')
    if not os.path.isdir(boot_path):
        print("Not deploying, boot path doesn't exist:", boot_path)
        return 1
    print("Copying", app_uf2_path, "to", boot_path)
    shutil.copy(app_uf2_path, boot_path)
    return 0


def main():
    if len(sys.argv) <= 1 or sys.argv[1] in ('-h', '--help'):
        print(__doc__)
        return 0
    app_name = sys.argv[1]
    app_uf2_path = build(app_name)
    return deploy(app_uf2_path)


if __name__ == '__main__':
    sys.exit(main())
