use crate::process::tracee::Tracee;
use crate::register::Original;
use crate::register::regs::Register;
use crate::register::SysArgIndex;
use crate::utils::Config;

use syscalls::Sysno;

/// in pronyx, we don't need a massive lazy_static hashmap anymore.
/// the `syscalls` crate already knows how to turn a number into a name.
pub fn name_of_syscall(sysnum: usize) -> Option<&'static str> {
    // Sysno::new(sysnum) handles the arch-specific mapping automatically
    Sysno::new(sysnum).map(|s| s.name())
}

pub fn handle_syscall(tracee: &mut Tracee, config: &Config) {
    let sysnum = tracee.regs.get_sys_num(Original);
    let sysno = Sysno::new(sysnum);

    match sysno {
        // the "fake root" logic
        Some(Sysno::getuid) | Some(Sysno::geteuid) | 
        Some(Sysno::getgid) | Some(Sysno::getegid) |
        Some(Sysno::getresuid) | Some(Sysno::getresgid) => {
            if config.root_id {
                // we tell the guest they are root (0)
                // assuming your register wrapper has a set_ret or similar:
                tracee.regs.set(Register::SysResult, 0, "pronyx: spoofing root identity");
            }
        },

        // the "silent success" for chown
        Some(Sysno::chown) | Some(Sysno::fchown) | Some(Sysno::lchown) => {
            if config.root_id {
                tracee.regs.set(Register::SysResult, 0, "pronyx: spoofing root identity");
            }
        },

        // the link2symlink logic
        Some(Sysno::link) | Some(Sysno::linkat) => {
            if config.link2symlink {
                // 1. get the oldpath and newpath from the original linkat
                let oldpath = tracee.regs.get(Original, Register::SysArg(SysArgIndex::SysArg2));
                let newdirfd = tracee.regs.get(Original, Register::SysArg(SysArgIndex::SysArg3));
                let newpath = tracee.regs.get(Original, Register::SysArg(SysArgIndex::SysArg4));

                // 2. rewrite the syscall number
                tracee.regs.set(Register::SysNum, Sysno::symlinkat as u64, "pronyx: redirecting to symlinkat");

                // 3. remap the registers for symlinkat's expectations:
                // symlinkat arg1: target (was linkat's oldpath)
                tracee.regs.set(Register::SysArg(SysArgIndex::SysArg1), oldpath, "target");
                // symlinkat arg2: newdirfd (was linkat's newdirfd)
                tracee.regs.set(Register::SysArg(SysArgIndex::SysArg2), newdirfd, "newdirfd");
                // symlinkat arg3: linkpath (was linkat's newpath)
                tracee.regs.set(Register::SysArg(SysArgIndex::SysArg3), newpath, "linkpath");
            }
        },

        _ => {}
    }
}

pub fn print_syscall<M>(tracee: &mut Tracee, msg: M, config: &Config)
where
    M: std::fmt::Display,
{  
    handle_syscall(tracee, config);

    // if cfg!(debug_assertions) {
    //     // we use the same 'Original' version for the args as we do for the sysnum
    //     let sysnum = tracee.regs.get_sys_num(Original);
    //     let name = name_of_syscall(sysnum).unwrap_or("unknown_syscall");

    //     // use the get() method which is actually public and works
    //     let arg1 = tracee.regs.get(Original, Register::SysArg(SysArgIndex::SysArg1));
    //     let arg2 = tracee.regs.get(Original, Register::SysArg(SysArgIndex::SysArg2));

    //     println!(
    //         "\x1b[33m[pronyx]\x1b[0m [{}] {}: {}(0x{:x}, 0x{:x})",
    //         tracee.pid, 
    //         msg, 
    //         name, 
    //         arg1, 
    //         arg2
    //     );
    // }
}