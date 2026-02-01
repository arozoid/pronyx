# pronyx

**Please take the PRoot Usage Survey for 2023!** [![Survey](https://img.shields.io/badge/survey-2023-green?style=flat-square)](https://www.surveymonkey.com/r/7GVXS7W)

---

[![Tests](https://img.shields.io/github/actions/workflow/status/proot-me/proot-rs/tests.yml?style=flat-square)](https://github.com/proot-me/pronyx/actions/workflows/tests.yml)
[![Releases](https://img.shields.io/github/v/release/proot-me/proot-rs?sort=semver&style=flat-square)](https://github.com/proot-me/pronyx/releases)

**pronyx** is a fork of PRoot-RS, a Rust implementation of PRoot, a ptrace-based sandbox.

The goal of this project is to build a new PRoot engine that solves the normal problems of PRoot for Onyx, a CLI that encapsulates Linux instances within boxes.

Currently, this fork is simply an updated version of PRoot-RS, however, updates will be coming soon.

## Usage

```
proot-rs 0.1.0
chroot, mount --bind, and binfmt_misc without privilege/setup.

USAGE:
    proot-rs [OPTIONS] [--] [command]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --bind <bind>...     Make the content of *host_path* accessible in the guest rootfs. Format:
                             host_path:guest_path
    -w, --cwd <cwd>          Set the initial working directory to *path*. [default: /]
    -r, --rootfs <rootfs>    Use *path* as the new guest root file-system. [default: /]

ARGS:
    <command>...  
```

