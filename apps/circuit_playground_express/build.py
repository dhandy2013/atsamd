#!/usr/bin/env python3
"""
Build and deploy a Rust Circuit Playground Express app.

The deploy part will only run if the board is connected via USB cable and is in
boot loader mode.
"""
import argparse
import getpass
import os
import shutil
import sys
import subprocess


def cmd(args):
    print(subprocess.list2cmdline(args))
    subprocess.check_call(args)


def build(app_name, *, example=False):
    if example:
        target_dir = 'target/thumbv6m-none-eabi/release/examples'
    else:
        raise NotImplementedError("binary crates not ready yet")
    cmd(['cargo', 'build', '--example', app_name, '--release'])
    app_elf_path = os.path.join(target_dir, app_name)
    app_bin_path = os.path.join(target_dir, app_name + '.bin')
    cmd(['arm-none-eabi-objcopy', '-O', 'binary', app_elf_path, app_bin_path])
    app_uf2_path = os.path.join(target_dir, app_name + '.uf2')
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
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('app_name', metavar='<app-name>',
                        help="Name of binary crate to build")
    parser.add_argument('--example', action='store_true',
                        help="Build a program under examples/")
    args = parser.parse_args()
    try:
        app_uf2_path = build(args.app_name, example=args.example)
        return deploy(app_uf2_path)
    except subprocess.CalledProcessError as err:
        print(err, file=sys.stderr)
        return err.returncode


if __name__ == '__main__':
    sys.exit(main())
