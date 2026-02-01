/// Used to organise system call numbers into an easily-matchable enumeration.
/// It's easier and cleaner to use cfg conditions here rather than in the huge
/// match in `translate_syscall_enter` and `translate_syscall_exit`.
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum SyscallGroup {
    Ignored = 0,
    Execve,
    Ptrace,
    Wait,
    Brk,
    GetCwd,
    Chdir,
    BindConnect,
    Accept,
    GetSockOrPeerName,
    #[allow(dead_code)]
    SocketCall,
    StandardSyscall, // syscalls that only require their path arguments to be translated
    Open,
    StatAt,
    ChmodAccessMkNodAt,
    InotifyAddWatch,
    DirLinkAttr,
    PivotRoot,
    LinkAt,
    Mount,
    OpenAt,
    Link,
    ReadLink,
    ReadLinkAt,
    Rename,
    RenameAt,
    SymLink,
    SymLinkAt,
    Uname,
    UnlinkMkdirAt,
}


// TODO: We also need to consider the unshare() system call. For example,
// the `CLONE_FS` flag may cause errors in our simulation of tracee's `cwd`
// field.

// TODO: modify the result of getdents64() so that we can handle binded entries.

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn syscall_group_from_sysnum(sysnum: i64) -> SyscallGroup {
    match sysnum {
        crate::sc::nr::EXECVE => SyscallGroup::Execve,
        crate::sc::nr::PTRACE => SyscallGroup::Ptrace,
        crate::sc::nr::WAIT4 => SyscallGroup::Wait,
        #[cfg(any(target_arch = "x86"))]
        crate::sc::nr::WAITPID => SyscallGroup::Wait,
        crate::sc::nr::BRK => SyscallGroup::Brk,
        crate::sc::nr::GETCWD => SyscallGroup::GetCwd,
        crate::sc::nr::FCHDIR | crate::sc::nr::CHDIR => SyscallGroup::Chdir,
        crate::sc::nr::BIND | crate::sc::nr::CONNECT => SyscallGroup::BindConnect,
        #[cfg(any(target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64"))]
        crate::sc::nr::ACCEPT => SyscallGroup::Accept,
        crate::sc::nr::ACCEPT4 => SyscallGroup::Accept,
        crate::sc::nr::GETSOCKNAME | crate::sc::nr::GETPEERNAME => SyscallGroup::GetSockOrPeerName,
        #[cfg(any(target_arch = "x86"))]
        crate::sc::nr::SOCKETCALL => SyscallGroup::SocketCall,

        // int syscall(const char *pathname, ...) follow symlink
        crate::sc::nr::ACCT
        | crate::sc::nr::CHROOT
        | crate::sc::nr::GETXATTR
        | crate::sc::nr::LISTXATTR
        | crate::sc::nr::REMOVEXATTR
        | crate::sc::nr::SETXATTR
        | crate::sc::nr::SWAPOFF
        | crate::sc::nr::SWAPON
        | crate::sc::nr::TRUNCATE
        | crate::sc::nr::UMOUNT2 => SyscallGroup::StandardSyscall,
        #[cfg(any(target_arch = "x86"))]
        crate::sc::nr::OLDSTAT | crate::sc::nr::UMOUNT => SyscallGroup::StandardSyscall,
        #[cfg(any(target_arch = "x86", target_arch = "arm"))]
        crate::sc::nr::CHOWN32 | crate::sc::nr::STAT64 | crate::sc::nr::STATFS64 | crate::sc::nr::TRUNCATE64 => {
            SyscallGroup::StandardSyscall
        }
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        crate::sc::nr::UTIME => SyscallGroup::StandardSyscall,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
        crate::sc::nr::ACCESS
        | crate::sc::nr::CHMOD
        | crate::sc::nr::CHOWN
        | crate::sc::nr::MKNOD
        | crate::sc::nr::CREAT
        | crate::sc::nr::STAT
        | crate::sc::nr::USELIB
        | crate::sc::nr::UTIMES => SyscallGroup::StandardSyscall,

        // int syscall(const char *pathname, int flags, ...)
        #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
        crate::sc::nr::OPEN => SyscallGroup::Open,

        // int syscall(int dirfd, const char *pathname, ... , int flags, ...)
        crate::sc::nr::FCHOWNAT | crate::sc::nr::UTIMENSAT | crate::sc::nr::NAME_TO_HANDLE_AT | crate::sc::nr::STATX => {
            SyscallGroup::StatAt
        }
        #[cfg(any(target_arch = "x86", target_arch = "arm"))]
        crate::sc::nr::FSTATAT64 => SyscallGroup::StatAt,
        #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
        crate::sc::nr::NEWFSTATAT => SyscallGroup::StatAt,

        // int syscall(int dirfd, const char *pathname, ...)
        crate::sc::nr::FCHMODAT | crate::sc::nr::FACCESSAT | crate::sc::nr::MKNODAT => SyscallGroup::ChmodAccessMkNodAt,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
        crate::sc::nr::FUTIMESAT => SyscallGroup::ChmodAccessMkNodAt,

        crate::sc::nr::INOTIFY_ADD_WATCH => SyscallGroup::InotifyAddWatch,

        // int syscall(const char *pathname, ...) not follow symlink
        crate::sc::nr::LGETXATTR | crate::sc::nr::LLISTXATTR | crate::sc::nr::LREMOVEXATTR | crate::sc::nr::LSETXATTR => {
            SyscallGroup::DirLinkAttr
        }
        #[cfg(any(target_arch = "x86"))]
        crate::sc::nr::OLDLSTAT => SyscallGroup::DirLinkAttr,
        #[cfg(any(target_arch = "x86", target_arch = "arm"))]
        crate::sc::nr::LCHOWN32 | crate::sc::nr::LSTAT64 => SyscallGroup::DirLinkAttr,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
        crate::sc::nr::LCHOWN | crate::sc::nr::LSTAT | crate::sc::nr::UNLINK | crate::sc::nr::RMDIR | crate::sc::nr::MKDIR => {
            SyscallGroup::DirLinkAttr
        }

        crate::sc::nr::PIVOT_ROOT => SyscallGroup::PivotRoot,
        crate::sc::nr::LINKAT => SyscallGroup::LinkAt,
        crate::sc::nr::MOUNT => SyscallGroup::Mount,
        crate::sc::nr::OPENAT => SyscallGroup::OpenAt,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
        crate::sc::nr::READLINK => SyscallGroup::ReadLink,
        crate::sc::nr::READLINKAT => SyscallGroup::ReadLinkAt,
        crate::sc::nr::UNLINKAT | crate::sc::nr::MKDIRAT => SyscallGroup::UnlinkMkdirAt,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
        crate::sc::nr::LINK => SyscallGroup::Link,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
        crate::sc::nr::RENAME => SyscallGroup::Rename,
        crate::sc::nr::RENAMEAT => SyscallGroup::RenameAt,
        #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
        crate::sc::nr::SYMLINK => SyscallGroup::SymLink,
        crate::sc::nr::SYMLINKAT => SyscallGroup::SymLinkAt,
        crate::sc::nr::UNAME => SyscallGroup::Uname,
        _ => SyscallGroup::Ignored,
    }
}
