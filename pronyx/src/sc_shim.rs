pub mod sc {
    pub mod nr {
        pub use libc::*;

        // map the ones that don't match libc's SYS_ prefix exactly
        // or are missing in certain libc versions
        pub use libc::SYS_execve as EXECVE;
        pub use libc::SYS_ptrace as PTRACE;
        pub use libc::SYS_wait4 as WAIT4;
        pub use libc::SYS_brk as BRK;
        pub use libc::SYS_getcwd as GETCWD;
        pub use libc::SYS_fchdir as FCHDIR;
        pub use libc::SYS_chdir as CHDIR;
        pub use libc::SYS_bind as BIND;
        pub use libc::SYS_connect as CONNECT;
        pub use libc::SYS_accept as ACCEPT;
        pub use libc::SYS_accept4 as ACCEPT4;
        pub use libc::SYS_getsockname as GETSOCKNAME;
        pub use libc::SYS_getpeername as GETPEERNAME;
        pub use libc::SYS_acct as ACCT;
        pub use libc::SYS_chroot as CHROOT;
        pub use libc::SYS_getxattr as GETXATTR;
        pub use libc::SYS_listxattr as LISTXATTR;
        pub use libc::SYS_removexattr as REMOVEXATTR;
        pub use libc::SYS_setxattr as SETXATTR;
        pub use libc::SYS_swapoff as SWAPOFF;
        pub use libc::SYS_swapon as SWAPON;
        pub use libc::SYS_truncate as TRUNCATE;
        pub use libc::SYS_umount2 as UMOUNT2;
        pub use libc::SYS_access as ACCESS;
        pub use libc::SYS_chmod as CHMOD;
        pub use libc::SYS_chown as CHOWN;
        pub use libc::SYS_mknod as MKNOD;
        pub use libc::SYS_creat as CREAT;
        pub use libc::SYS_stat as STAT;
        pub use libc::SYS_uselib as USELIB;
        pub use libc::SYS_utimes as UTIMES;
        pub use libc::SYS_open as OPEN;
        pub use libc::SYS_fchownat as FCHOWNAT;
        pub use libc::SYS_utimensat as UTIMENSAT;
        pub use libc::SYS_name_to_handle_at as NAME_TO_HANDLE_AT;
        
        // x86_64 specific / modern stat
        #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
        pub use libc::SYS_newfstatat as NEWFSTATAT;
        
        pub use libc::SYS_fchmodat as FCHMODAT;
        pub use libc::SYS_faccessat as FACCESSAT;
        pub use libc::SYS_mknodat as MKNODAT;
        pub use libc::SYS_inotify_add_watch as INOTIFY_ADD_WATCH;
        pub use libc::SYS_lgetxattr as LGETXATTR;
        pub use libc::SYS_llistxattr as LLISTXATTR;
        pub use libc::SYS_lremovexattr as LREMOVEXATTR;
        pub use libc::SYS_lsetxattr as LSETXATTR;
        pub use libc::SYS_lchown as LCHOWN;
        pub use libc::SYS_lstat as LSTAT;
        pub use libc::SYS_unlink as UNLINK;
        pub use libc::SYS_rmdir as RMDIR;
        pub use libc::SYS_mkdir as MKDIR;
        pub use libc::SYS_pivot_root as PIVOT_ROOT;
        pub use libc::SYS_linkat as LINKAT;
        pub use libc::SYS_mount as MOUNT;
        pub use libc::SYS_openat as OPENAT;
        pub use libc::SYS_readlink as READLINK;
        pub use libc::SYS_readlinkat as READLINKAT;
        pub use libc::SYS_unlinkat as UNLINKAT;
        pub use libc::SYS_mkdirat as MKDIRAT;
        pub use libc::SYS_link as LINK;
        pub use libc::SYS_rename as RENAME;
        pub use libc::SYS_renameat as RENAMEAT;
        pub use libc::SYS_symlink as SYMLINK;
        pub use libc::SYS_symlinkat as SYMLINKAT;
        pub use libc::SYS_uname as UNAME;
        pub use libc::SYS_nanosleep as NANOSLEEP;
        pub use libc::SYS_clock_nanosleep as CLOCK_NANOSLEEP;
        pub use libc::SYS_utime as UTIME;
        pub use libc::SYS_futimesat as FUTIMESAT;

        pub use libc::SYS_vfork as VFORK;
        pub use libc::SYS_clone as CLONE;

        // the 'statx' nightmare
        pub use libc::SYS_statx as STATX;
    }
}