#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

mod script;

use core::{fmt::Write};

use crate::script::*;

const O_RDONLY: usize = 00000000;
#[allow(dead_code)]
const AT_FDCWD: isize = -100;
const MAP_PRIVATE: usize = 0x02;
const MAP_FIXED: usize = 0x10;
const MAP_ANONYMOUS: usize = 0x20;

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
const MMAP_OFFSET_SHIFT: usize = 0;
#[cfg(any(target_arch = "arm"))]
const MMAP_OFFSET_SHIFT: usize = 12;

#[allow(unused)]
const WRITE: usize = 1;
const MMAP: usize = 9;
const PRCTL: usize = 157;
const EXECVE: usize = 59;

const MPROTECT: usize = 10;

const PROT_READ: usize = 0x1;
const PROT_WRITE: usize = 0x2;
const PROT_EXEC: usize = 0x4;
const PROT_GROWSDOWN: usize = 0x01000000;

const OPEN: usize = 2;
const CLOSE: usize = 3;
const AT_NULL: usize = 0;

const AT_PHDR: usize = 3;
const AT_PHENT: usize = 4;
const AT_PHNUM: usize = 5;
const AT_BASE: usize = 7;   // address of the interpreter (ld.so)
const AT_ENTRY: usize = 9;
const AT_EXECFN: usize = 31; // filename of the executed program

const PR_SET_NAME: usize = 15;

macro_rules! branch {
    ($stack_ptr:expr, $entry_pt:expr) => {
        core::arch::asm!(
            "mov rsp, {0}",
            "jmp {1}",
            in(reg) $stack_ptr,
            in(reg) $entry_pt,
            options(noreturn)
        );
    };
}

macro_rules! sc {
    ($nr:expr) => { syscall6($nr, 0, 0, 0, 0, 0, 0) };
    ($nr:expr, $a1:expr) => { syscall6($nr, $a1 as usize, 0, 0, 0, 0, 0) };
    ($nr:expr, $a1:expr, $a2:expr) => { syscall6($nr, $a1 as usize, $a2 as usize, 0, 0, 0, 0) };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => { syscall6($nr, $a1 as usize, $a2 as usize, $a3 as usize, 0, 0, 0) };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => { syscall6($nr, $a1 as usize, $a2 as usize, $a3 as usize, $a4 as usize, 0, 0) };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => { syscall6($nr, $a1 as usize, $a2 as usize, $a3 as usize, $a4 as usize, $a5 as usize, 0) };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => { syscall6($nr, $a1 as usize, $a2 as usize, $a3 as usize, $a4 as usize, $a5 as usize, $a6 as usize) };
}

unsafe fn syscall6(nr: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize) -> usize {
    let ret: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") nr => ret,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            in("r10") a4,
            in("r8") a5,
            in("r9") a6,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start(mut cursor: *const ()) {
    let mut traced = false;
    let mut reset_at_base = true;
    let mut at_base: Word = 0;
    let mut fd: Option<isize> = None;

    loop {
        unsafe {
        // check if cursor is null
        // TODO: Check LoadStatement flag is vaild: Converting memory regions
        // directly to references to enum in rust is dangerous because invalid
        // tags can lead to undefined behaviors.
        let stmt: &LoadStatement = match (cursor as *const LoadStatement).as_ref() {
            Some(stmt) => stmt,
            None => core::arch::asm!(
                        "mov rax, 60", // sys_exit
                        "mov rdi, 1",  // status 1
                        "syscall",
                        options(noreturn)
                    ),
        };
        match stmt {
            st @ (LoadStatement::OpenNext(open) | LoadStatement::Open(open)) => {
                if let LoadStatement::OpenNext(_) = st {
                    // close last fd
                    assert!(sc!(CLOSE, fd.unwrap() as usize, 0, 0) == 0);
                }
                // open file
                #[cfg(any(target_arch = "x86", target_arch = "arm", target_arch = "x86_64"))]
                let status = sc!(OPEN, open.string_address, O_RDONLY, 0) as isize;
                #[cfg(any(target_arch = "aarch64"))]
                let status =
                    sc!(OPENAT, AT_FDCWD, open.string_address, O_RDONLY, 0) as isize;
                assert!(status >= 0);
                fd = Some(status);
                reset_at_base = true
            }
            LoadStatement::MmapFile(mmap) => {
                // call mmap() with fd
                #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
                let status = sc!(
                    MMAP,
                    mmap.addr as usize,
                    mmap.length as usize,
                    mmap.prot as usize,
                    MAP_PRIVATE | MAP_FIXED,
                    fd.unwrap() as usize, // fd needs to be usize
                    (mmap.offset >> MMAP_OFFSET_SHIFT) as usize
                );
                #[cfg(any(target_arch = "arm", target_arch = "x86"))]
                let status = sc!(
                    MMAP2,
                    mmap.addr,
                    mmap.length,
                    mmap.prot,
                    MAP_PRIVATE | MAP_FIXED,
                    fd.unwrap(),
                    mmap.offset >> MMAP_OFFSET_SHIFT
                );
                assert_eq!(status, mmap.addr as _);
                // set the end of the space to 0, if needed.
                if mmap.clear_length != 0 {
                    let start = (mmap.addr + mmap.length - mmap.clear_length) as *mut u8;
                    for i in 0..mmap.clear_length {
                        *start.offset(i as isize) = 0u8;
                    }
                }
                // if value of AT_BASE need to be reset
                if reset_at_base {
                    at_base = mmap.addr;
                    reset_at_base = false;
                }
            }
            LoadStatement::MmapAnonymous(mmap) => {
                #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
                let status = sc!(
                    MMAP,
                    mmap.addr,
                    mmap.length,
                    mmap.prot,
                    MAP_PRIVATE | MAP_FIXED | MAP_ANONYMOUS,
                    (-1isize) as usize,
                    0
                );
                #[cfg(any(target_arch = "arm", target_arch = "x86"))]
                let status = sc!(
                    MMAP2,
                    mmap.addr,
                    mmap.length,
                    mmap.prot,
                    MAP_PRIVATE | MAP_FIXED | MAP_ANONYMOUS,
                    (-1isize) as usize,
                    0
                );

                assert!(status as isize >= 0);
            }
            LoadStatement::MakeStackExec(stack_exec) => {
                sc!(
                    MPROTECT,
                    stack_exec.start,
                    1,
                    PROT_READ | PROT_WRITE | PROT_EXEC | PROT_GROWSDOWN
                );
            }
            st @ (LoadStatement::StartTraced(start) | LoadStatement::Start(start)) => {
                if let LoadStatement::StartTraced(_) = st {
                    traced = true;
                }
                // close last fd
                assert!(sc!(CLOSE, fd.unwrap()) as isize >= 0);

                /* Right after execve, the stack content is as follow:
                 *
                 *   +------+--------+--------+--------+
                 *   | argc | argv[] | envp[] | auxv[] |
                 *   +------+--------+--------+--------+
                 */
                let mut cursor2: *mut Word = start.stack_pointer as _;
                let argc = *cursor2.offset(0);
                let at_execfn = *cursor2.offset(1);

                // skip argv[]
                cursor2 = cursor2.offset((argc + 1 + 1) as _);
                // the last element of argv should be a null pointer
                assert_eq!(*cursor2.offset(-1), 0);

                // skip envp[]
                while *cursor2 != 0 {
                    cursor2 = cursor2.offset(1)
                }
                cursor2 = cursor2.offset(1);

                // adjust auxv[]
                while *cursor2.offset(0) as usize != AT_NULL {
                    match *cursor2.offset(0) as usize {
                        AT_PHDR => *cursor2.offset(1) = start.at_phdr,
                        AT_PHENT => *cursor2.offset(1) = start.at_phent,
                        AT_PHNUM => *cursor2.offset(1) = start.at_phnum,
                        AT_ENTRY => *cursor2.offset(1) = start.at_entry,
                        AT_BASE => *cursor2.offset(1) = at_base,
                        AT_EXECFN => {
                            /* stmt->start.at_execfn can't be used for now since it is
                             * currently stored in a location that will be scratched
                             * by the process (below the final stack pointer).  */

                            *cursor2.offset(1) = at_execfn
                        }
                        _ => {}
                    }

                    cursor2 = cursor2.offset(2);
                }

                // get base name of executable path
                let get_basename = |string: *const u8| -> *const u8 {
                    let mut cursor = string;
                    while *cursor != 0 {
                        cursor = cursor.offset(1);
                    }
                    while *cursor != b'/' && cursor > string {
                        cursor = cursor.offset(-1);
                    }
                    if *cursor == b'/' {
                        cursor = cursor.offset(1);
                    }
                    cursor
                };
                let name = get_basename(start.at_execfn as _);
                sc!(PRCTL, PR_SET_NAME, name as usize, 0);

                // jump to new entry point
                if traced {
                    sc!(EXECVE, 1, start.stack_pointer, start.entry_point, 2, 3, 4);
                } else {
                    branch!(start.stack_pointer, start.entry_point);
                }
                unreachable!()
            }
        }
        // move cursor to next load statement
        cursor = (cursor as *const u8).offset(stmt.as_bytes().len() as _) as _;
        }
    }
}

#[allow(unused)]
struct Stderr {}

impl Write for Stderr {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bs = s.as_bytes();
        let mut count = 0;
        while count < bs.len() {
            unsafe {
                let status = sc!(WRITE, 2, bs.as_ptr().add(count), bs.len() - count);
                if (status as isize) < 0 {
                    return Err(core::fmt::Error);
                } else {
                    count += status;
                }
            }
        }
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> () {
    loop {}
}
