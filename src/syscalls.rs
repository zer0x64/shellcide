// Generated file containing Linux x86_64 syscalls.
// Source: https://filippo.io/linux-syscall-table/

#[derive(Clone, Copy)]
pub struct SyscallArg {
    pub reg: &'static str,
    pub arg_type: &'static str,
}

#[derive(Clone, Copy)]
pub struct SyscallInfo {
    pub nr: usize,
    pub name: &'static str,
    pub man_url: &'static str,
    pub entry_point: &'static str,
    pub args: &'static [SyscallArg],
}

pub static SYSCALLS: &[SyscallInfo] = &[
    SyscallInfo {
        nr: 0,
        name: "read",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/read.2.en.html",
        entry_point: "sys_read",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "char *buf",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t count",
            },
        ],
    },
    SyscallInfo {
        nr: 1,
        name: "write",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/write.2.en.html",
        entry_point: "sys_write",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *buf",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t count",
            },
        ],
    },
    SyscallInfo {
        nr: 2,
        name: "open",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/open.2.en.html",
        entry_point: "sys_open",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int flags",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "umode_t mode",
            },
        ],
    },
    SyscallInfo {
        nr: 3,
        name: "close",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/close.2.en.html",
        entry_point: "sys_close",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "unsigned int fd",
        }],
    },
    SyscallInfo {
        nr: 4,
        name: "stat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/stat.2.en.html",
        entry_point: "sys_newstat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct stat *statbuf",
            },
        ],
    },
    SyscallInfo {
        nr: 5,
        name: "fstat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fstat.2.en.html",
        entry_point: "sys_newfstat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct stat *statbuf",
            },
        ],
    },
    SyscallInfo {
        nr: 6,
        name: "lstat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/lstat.2.en.html",
        entry_point: "sys_newlstat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct stat *statbuf",
            },
        ],
    },
    SyscallInfo {
        nr: 7,
        name: "poll",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/poll.2.en.html",
        entry_point: "sys_poll",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "struct pollfd *ufds",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int nfds",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int timeout",
            },
        ],
    },
    SyscallInfo {
        nr: 8,
        name: "lseek",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/lseek.2.en.html",
        entry_point: "sys_lseek",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "off_t offset",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int whence",
            },
        ],
    },
    SyscallInfo {
        nr: 9,
        name: "mmap",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mmap.2.en.html",
        entry_point: "sys_ksys_mmap_pgoff",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long addr",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long prot",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long flags",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long fd",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "unsigned long pgoff",
            },
        ],
    },
    SyscallInfo {
        nr: 10,
        name: "mprotect",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mprotect.2.en.html",
        entry_point: "sys_mprotect",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long start",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long prot",
            },
        ],
    },
    SyscallInfo {
        nr: 11,
        name: "munmap",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/munmap.2.en.html",
        entry_point: "sys_munmap",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long addr",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t len",
            },
        ],
    },
    SyscallInfo {
        nr: 12,
        name: "brk",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/brk.2.en.html",
        entry_point: "sys_brk",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "unsigned long brk",
        }],
    },
    SyscallInfo {
        nr: 13,
        name: "rt_sigaction",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/rt_sigaction.2.en.html",
        entry_point: "sys_rt_sigaction",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct sigaction *",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct sigaction *",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "size_t",
            },
        ],
    },
    SyscallInfo {
        nr: 14,
        name: "rt_sigprocmask",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/rt_sigprocmask.2.en.html",
        entry_point: "sys_rt_sigprocmask",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int how",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "sigset_t *set",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "sigset_t *oset",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "size_t sigsetsize",
            },
        ],
    },
    SyscallInfo {
        nr: 15,
        name: "rt_sigreturn",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/rt_sigreturn.2.en.html",
        entry_point: "sys_rt_sigreturn",
        args: &[],
    },
    SyscallInfo {
        nr: 16,
        name: "ioctl",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/ioctl.2.en.html",
        entry_point: "sys_ioctl",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int cmd",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long arg",
            },
        ],
    },
    SyscallInfo {
        nr: 17,
        name: "pread64",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pread64.2.en.html",
        entry_point: "sys_pread64",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "char *buf",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t count",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "loff_t pos",
            },
        ],
    },
    SyscallInfo {
        nr: 18,
        name: "pwrite64",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pwrite64.2.en.html",
        entry_point: "sys_pwrite64",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *buf",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t count",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "loff_t pos",
            },
        ],
    },
    SyscallInfo {
        nr: 19,
        name: "readv",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/readv.2.en.html",
        entry_point: "sys_readv",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct iovec *vec",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long vlen",
            },
        ],
    },
    SyscallInfo {
        nr: 20,
        name: "writev",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/writev.2.en.html",
        entry_point: "sys_writev",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct iovec *vec",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long vlen",
            },
        ],
    },
    SyscallInfo {
        nr: 21,
        name: "access",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/access.2.en.html",
        entry_point: "sys_access",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int mode",
            },
        ],
    },
    SyscallInfo {
        nr: 22,
        name: "pipe",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pipe.2.en.html",
        entry_point: "sys_pipe",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int *fildes",
        }],
    },
    SyscallInfo {
        nr: 23,
        name: "select",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/select.2.en.html",
        entry_point: "sys_select",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int n",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "fd_set *inp",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "fd_set *outp",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "fd_set *exp",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "struct __kernel_old_timeval *tvp",
            },
        ],
    },
    SyscallInfo {
        nr: 24,
        name: "sched_yield",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sched_yield.2.en.html",
        entry_point: "sys_sched_yield",
        args: &[],
    },
    SyscallInfo {
        nr: 25,
        name: "mremap",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mremap.2.en.html",
        entry_point: "sys_mremap",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long addr",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long old_len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long new_len",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long flags",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long new_addr",
            },
        ],
    },
    SyscallInfo {
        nr: 26,
        name: "msync",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/msync.2.en.html",
        entry_point: "sys_msync",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long start",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 27,
        name: "mincore",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mincore.2.en.html",
        entry_point: "sys_mincore",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long start",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned char * vec",
            },
        ],
    },
    SyscallInfo {
        nr: 28,
        name: "madvise",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/madvise.2.en.html",
        entry_point: "sys_madvise",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long start",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int behavior",
            },
        ],
    },
    SyscallInfo {
        nr: 29,
        name: "shmget",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/shmget.2.en.html",
        entry_point: "sys_shmget",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "key_t key",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t size",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int flag",
            },
        ],
    },
    SyscallInfo {
        nr: 30,
        name: "shmat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/shmat.2.en.html",
        entry_point: "sys_shmat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int shmid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "char *shmaddr",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int shmflg",
            },
        ],
    },
    SyscallInfo {
        nr: 31,
        name: "shmctl",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/shmctl.2.en.html",
        entry_point: "sys_shmctl",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int shmid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int cmd",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct shmid_ds *buf",
            },
        ],
    },
    SyscallInfo {
        nr: 32,
        name: "dup",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/dup.2.en.html",
        entry_point: "sys_dup",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "unsigned int fildes",
        }],
    },
    SyscallInfo {
        nr: 33,
        name: "dup2",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/dup2.2.en.html",
        entry_point: "sys_dup2",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int oldfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int newfd",
            },
        ],
    },
    SyscallInfo {
        nr: 34,
        name: "pause",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pause.2.en.html",
        entry_point: "sys_pause",
        args: &[],
    },
    SyscallInfo {
        nr: 35,
        name: "nanosleep",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/nanosleep.2.en.html",
        entry_point: "sys_nanosleep",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "struct __kernel_timespec *rqtp",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct __kernel_timespec *rmtp",
            },
        ],
    },
    SyscallInfo {
        nr: 36,
        name: "getitimer",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getitimer.2.en.html",
        entry_point: "sys_getitimer",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int which",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct __kernel_old_itimerval *value",
            },
        ],
    },
    SyscallInfo {
        nr: 37,
        name: "alarm",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/alarm.2.en.html",
        entry_point: "sys_alarm",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "unsigned int seconds",
        }],
    },
    SyscallInfo {
        nr: 38,
        name: "setitimer",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setitimer.2.en.html",
        entry_point: "sys_setitimer",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int which",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct __kernel_old_itimerval *value",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct __kernel_old_itimerval *ovalue",
            },
        ],
    },
    SyscallInfo {
        nr: 39,
        name: "getpid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getpid.2.en.html",
        entry_point: "sys_getpid",
        args: &[],
    },
    SyscallInfo {
        nr: 40,
        name: "sendfile",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sendfile.2.en.html",
        entry_point: "sys_sendfile64",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int out_fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int in_fd",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "loff_t *offset",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "size_t count",
            },
        ],
    },
    SyscallInfo {
        nr: 41,
        name: "socket",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/socket.2.en.html",
        entry_point: "sys_socket",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int",
            },
        ],
    },
    SyscallInfo {
        nr: 42,
        name: "connect",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/connect.2.en.html",
        entry_point: "sys_connect",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sockaddr *",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int",
            },
        ],
    },
    SyscallInfo {
        nr: 43,
        name: "accept",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/accept.2.en.html",
        entry_point: "sys_accept",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sockaddr *",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int *",
            },
        ],
    },
    SyscallInfo {
        nr: 44,
        name: "sendto",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sendto.2.en.html",
        entry_point: "sys_sendto",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "void *",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "struct sockaddr *",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "int",
            },
        ],
    },
    SyscallInfo {
        nr: 45,
        name: "recvfrom",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/recvfrom.2.en.html",
        entry_point: "sys_recvfrom",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "void *",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "struct sockaddr *",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "int *",
            },
        ],
    },
    SyscallInfo {
        nr: 46,
        name: "sendmsg",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sendmsg.2.en.html",
        entry_point: "sys_sendmsg",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct user_msghdr *msg",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned flags",
            },
        ],
    },
    SyscallInfo {
        nr: 47,
        name: "recvmsg",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/recvmsg.2.en.html",
        entry_point: "sys_recvmsg",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct user_msghdr *msg",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned flags",
            },
        ],
    },
    SyscallInfo {
        nr: 48,
        name: "shutdown",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/shutdown.2.en.html",
        entry_point: "sys_shutdown",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int",
            },
        ],
    },
    SyscallInfo {
        nr: 49,
        name: "bind",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/bind.2.en.html",
        entry_point: "sys_bind",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sockaddr *",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int",
            },
        ],
    },
    SyscallInfo {
        nr: 50,
        name: "listen",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/listen.2.en.html",
        entry_point: "sys_listen",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int",
            },
        ],
    },
    SyscallInfo {
        nr: 51,
        name: "getsockname",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getsockname.2.en.html",
        entry_point: "sys_getsockname",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sockaddr *",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int *",
            },
        ],
    },
    SyscallInfo {
        nr: 52,
        name: "getpeername",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getpeername.2.en.html",
        entry_point: "sys_getpeername",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sockaddr *",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int *",
            },
        ],
    },
    SyscallInfo {
        nr: 53,
        name: "socketpair",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/socketpair.2.en.html",
        entry_point: "sys_socketpair",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int *",
            },
        ],
    },
    SyscallInfo {
        nr: 54,
        name: "setsockopt",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setsockopt.2.en.html",
        entry_point: "sys_setsockopt",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int level",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int optname",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "char *optval",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int optlen",
            },
        ],
    },
    SyscallInfo {
        nr: 55,
        name: "getsockopt",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getsockopt.2.en.html",
        entry_point: "sys_getsockopt",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int level",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int optname",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "char *optval",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int *optlen",
            },
        ],
    },
    SyscallInfo {
        nr: 56,
        name: "clone",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/clone.2.en.html",
        entry_point: "sys_clone",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int *",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int *",
            },
        ],
    },
    SyscallInfo {
        nr: 57,
        name: "fork",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fork.2.en.html",
        entry_point: "sys_fork",
        args: &[],
    },
    SyscallInfo {
        nr: 58,
        name: "vfork",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/vfork.2.en.html",
        entry_point: "sys_vfork",
        args: &[],
    },
    SyscallInfo {
        nr: 59,
        name: "execve",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/execve.2.en.html",
        entry_point: "sys_execve",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *const *argv",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const char *const *envp",
            },
        ],
    },
    SyscallInfo {
        nr: 60,
        name: "exit",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/exit.2.en.html",
        entry_point: "sys_exit",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int error_code",
        }],
    },
    SyscallInfo {
        nr: 61,
        name: "wait4",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/wait4.2.en.html",
        entry_point: "sys_wait4",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int *stat_addr",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int options",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "struct rusage *ru",
            },
        ],
    },
    SyscallInfo {
        nr: 62,
        name: "kill",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/kill.2.en.html",
        entry_point: "sys_kill",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int sig",
            },
        ],
    },
    SyscallInfo {
        nr: 63,
        name: "uname",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/uname.2.en.html",
        entry_point: "sys_newuname",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "struct new_utsname *name",
        }],
    },
    SyscallInfo {
        nr: 64,
        name: "semget",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/semget.2.en.html",
        entry_point: "sys_semget",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "key_t key",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int nsems",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int semflg",
            },
        ],
    },
    SyscallInfo {
        nr: 65,
        name: "semop",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/semop.2.en.html",
        entry_point: "sys_semop",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int semid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sembuf *sops",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned nsops",
            },
        ],
    },
    SyscallInfo {
        nr: 66,
        name: "semctl",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/semctl.2.en.html",
        entry_point: "sys_semctl",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int semid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int semnum",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int cmd",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long arg",
            },
        ],
    },
    SyscallInfo {
        nr: 67,
        name: "shmdt",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/shmdt.2.en.html",
        entry_point: "sys_shmdt",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "char *shmaddr",
        }],
    },
    SyscallInfo {
        nr: 68,
        name: "msgget",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/msgget.2.en.html",
        entry_point: "sys_msgget",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "key_t key",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int msgflg",
            },
        ],
    },
    SyscallInfo {
        nr: 69,
        name: "msgsnd",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/msgsnd.2.en.html",
        entry_point: "sys_msgsnd",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int msqid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct msgbuf *msgp",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t msgsz",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int msgflg",
            },
        ],
    },
    SyscallInfo {
        nr: 70,
        name: "msgrcv",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/msgrcv.2.en.html",
        entry_point: "sys_msgrcv",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int msqid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct msgbuf *msgp",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t msgsz",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "long msgtyp",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int msgflg",
            },
        ],
    },
    SyscallInfo {
        nr: 71,
        name: "msgctl",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/msgctl.2.en.html",
        entry_point: "sys_msgctl",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int msqid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int cmd",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct msqid_ds *buf",
            },
        ],
    },
    SyscallInfo {
        nr: 72,
        name: "fcntl",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fcntl.2.en.html",
        entry_point: "sys_fcntl",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int cmd",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long arg",
            },
        ],
    },
    SyscallInfo {
        nr: 73,
        name: "flock",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/flock.2.en.html",
        entry_point: "sys_flock",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int cmd",
            },
        ],
    },
    SyscallInfo {
        nr: 74,
        name: "fsync",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fsync.2.en.html",
        entry_point: "sys_fsync",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "unsigned int fd",
        }],
    },
    SyscallInfo {
        nr: 75,
        name: "fdatasync",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fdatasync.2.en.html",
        entry_point: "sys_fdatasync",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "unsigned int fd",
        }],
    },
    SyscallInfo {
        nr: 76,
        name: "truncate",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/truncate.2.en.html",
        entry_point: "sys_truncate",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "long length",
            },
        ],
    },
    SyscallInfo {
        nr: 77,
        name: "ftruncate",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/ftruncate.2.en.html",
        entry_point: "sys_ftruncate",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "off_t length",
            },
        ],
    },
    SyscallInfo {
        nr: 78,
        name: "getdents",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getdents.2.en.html",
        entry_point: "sys_getdents",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct linux_dirent *dirent",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int count",
            },
        ],
    },
    SyscallInfo {
        nr: 79,
        name: "getcwd",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getcwd.2.en.html",
        entry_point: "sys_getcwd",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "char *buf",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long size",
            },
        ],
    },
    SyscallInfo {
        nr: 80,
        name: "chdir",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/chdir.2.en.html",
        entry_point: "sys_chdir",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "const char *filename",
        }],
    },
    SyscallInfo {
        nr: 81,
        name: "fchdir",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fchdir.2.en.html",
        entry_point: "sys_fchdir",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "unsigned int fd",
        }],
    },
    SyscallInfo {
        nr: 82,
        name: "rename",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/rename.2.en.html",
        entry_point: "sys_rename",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *oldname",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *newname",
            },
        ],
    },
    SyscallInfo {
        nr: 83,
        name: "mkdir",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mkdir.2.en.html",
        entry_point: "sys_mkdir",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *pathname",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "umode_t mode",
            },
        ],
    },
    SyscallInfo {
        nr: 84,
        name: "rmdir",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/rmdir.2.en.html",
        entry_point: "sys_rmdir",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "const char *pathname",
        }],
    },
    SyscallInfo {
        nr: 85,
        name: "creat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/creat.2.en.html",
        entry_point: "sys_creat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *pathname",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "umode_t mode",
            },
        ],
    },
    SyscallInfo {
        nr: 86,
        name: "link",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/link.2.en.html",
        entry_point: "sys_link",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *oldname",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *newname",
            },
        ],
    },
    SyscallInfo {
        nr: 87,
        name: "unlink",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/unlink.2.en.html",
        entry_point: "sys_unlink",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "const char *pathname",
        }],
    },
    SyscallInfo {
        nr: 88,
        name: "symlink",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/symlink.2.en.html",
        entry_point: "sys_symlink",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *old",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *new",
            },
        ],
    },
    SyscallInfo {
        nr: 89,
        name: "readlink",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/readlink.2.en.html",
        entry_point: "sys_readlink",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "char *buf",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int bufsiz",
            },
        ],
    },
    SyscallInfo {
        nr: 90,
        name: "chmod",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/chmod.2.en.html",
        entry_point: "sys_chmod",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "umode_t mode",
            },
        ],
    },
    SyscallInfo {
        nr: 91,
        name: "fchmod",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fchmod.2.en.html",
        entry_point: "sys_fchmod",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "umode_t mode",
            },
        ],
    },
    SyscallInfo {
        nr: 92,
        name: "chown",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/chown.2.en.html",
        entry_point: "sys_chown",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "uid_t user",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "gid_t group",
            },
        ],
    },
    SyscallInfo {
        nr: 93,
        name: "fchown",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fchown.2.en.html",
        entry_point: "sys_fchown",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "uid_t user",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "gid_t group",
            },
        ],
    },
    SyscallInfo {
        nr: 94,
        name: "lchown",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/lchown.2.en.html",
        entry_point: "sys_lchown",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "uid_t user",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "gid_t group",
            },
        ],
    },
    SyscallInfo {
        nr: 95,
        name: "umask",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/umask.2.en.html",
        entry_point: "sys_umask",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int mask",
        }],
    },
    SyscallInfo {
        nr: 96,
        name: "gettimeofday",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/gettimeofday.2.en.html",
        entry_point: "sys_gettimeofday",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "struct __kernel_old_timeval *tv",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct timezone *tz",
            },
        ],
    },
    SyscallInfo {
        nr: 97,
        name: "getrlimit",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getrlimit.2.en.html",
        entry_point: "sys_getrlimit",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int resource",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct rlimit *rlim",
            },
        ],
    },
    SyscallInfo {
        nr: 98,
        name: "getrusage",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getrusage.2.en.html",
        entry_point: "sys_getrusage",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int who",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct rusage *ru",
            },
        ],
    },
    SyscallInfo {
        nr: 99,
        name: "sysinfo",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sysinfo.2.en.html",
        entry_point: "sys_sysinfo",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "struct sysinfo *info",
        }],
    },
    SyscallInfo {
        nr: 100,
        name: "times",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/times.2.en.html",
        entry_point: "sys_times",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "struct tms *tbuf",
        }],
    },
    SyscallInfo {
        nr: 101,
        name: "ptrace",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/ptrace.2.en.html",
        entry_point: "sys_ptrace",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "long request",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "long pid",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long addr",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long data",
            },
        ],
    },
    SyscallInfo {
        nr: 102,
        name: "getuid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getuid.2.en.html",
        entry_point: "sys_getuid",
        args: &[],
    },
    SyscallInfo {
        nr: 103,
        name: "syslog",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/syslog.2.en.html",
        entry_point: "sys_syslog",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int type",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "char *buf",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int len",
            },
        ],
    },
    SyscallInfo {
        nr: 104,
        name: "getgid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getgid.2.en.html",
        entry_point: "sys_getgid",
        args: &[],
    },
    SyscallInfo {
        nr: 105,
        name: "setuid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setuid.2.en.html",
        entry_point: "sys_setuid",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "uid_t uid",
        }],
    },
    SyscallInfo {
        nr: 106,
        name: "setgid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setgid.2.en.html",
        entry_point: "sys_setgid",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "gid_t gid",
        }],
    },
    SyscallInfo {
        nr: 107,
        name: "geteuid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/geteuid.2.en.html",
        entry_point: "sys_geteuid",
        args: &[],
    },
    SyscallInfo {
        nr: 108,
        name: "getegid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getegid.2.en.html",
        entry_point: "sys_getegid",
        args: &[],
    },
    SyscallInfo {
        nr: 109,
        name: "setpgid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setpgid.2.en.html",
        entry_point: "sys_setpgid",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "pid_t pgid",
            },
        ],
    },
    SyscallInfo {
        nr: 110,
        name: "getppid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getppid.2.en.html",
        entry_point: "sys_getppid",
        args: &[],
    },
    SyscallInfo {
        nr: 111,
        name: "getpgrp",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getpgrp.2.en.html",
        entry_point: "sys_getpgrp",
        args: &[],
    },
    SyscallInfo {
        nr: 112,
        name: "setsid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setsid.2.en.html",
        entry_point: "sys_setsid",
        args: &[],
    },
    SyscallInfo {
        nr: 113,
        name: "setreuid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setreuid.2.en.html",
        entry_point: "sys_setreuid",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "uid_t ruid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "uid_t euid",
            },
        ],
    },
    SyscallInfo {
        nr: 114,
        name: "setregid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setregid.2.en.html",
        entry_point: "sys_setregid",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "gid_t rgid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "gid_t egid",
            },
        ],
    },
    SyscallInfo {
        nr: 115,
        name: "getgroups",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getgroups.2.en.html",
        entry_point: "sys_getgroups",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int gidsetsize",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "gid_t *grouplist",
            },
        ],
    },
    SyscallInfo {
        nr: 116,
        name: "setgroups",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setgroups.2.en.html",
        entry_point: "sys_setgroups",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int gidsetsize",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "gid_t *grouplist",
            },
        ],
    },
    SyscallInfo {
        nr: 117,
        name: "setresuid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setresuid.2.en.html",
        entry_point: "sys_setresuid",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "uid_t ruid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "uid_t euid",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "uid_t suid",
            },
        ],
    },
    SyscallInfo {
        nr: 118,
        name: "getresuid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getresuid.2.en.html",
        entry_point: "sys_getresuid",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "uid_t *ruid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "uid_t *euid",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "uid_t *suid",
            },
        ],
    },
    SyscallInfo {
        nr: 119,
        name: "setresgid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setresgid.2.en.html",
        entry_point: "sys_setresgid",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "gid_t rgid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "gid_t egid",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "gid_t sgid",
            },
        ],
    },
    SyscallInfo {
        nr: 120,
        name: "getresgid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getresgid.2.en.html",
        entry_point: "sys_getresgid",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "gid_t *rgid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "gid_t *egid",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "gid_t *sgid",
            },
        ],
    },
    SyscallInfo {
        nr: 121,
        name: "getpgid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getpgid.2.en.html",
        entry_point: "sys_getpgid",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "pid_t pid",
        }],
    },
    SyscallInfo {
        nr: 122,
        name: "setfsuid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setfsuid.2.en.html",
        entry_point: "sys_setfsuid",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "uid_t uid",
        }],
    },
    SyscallInfo {
        nr: 123,
        name: "setfsgid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setfsgid.2.en.html",
        entry_point: "sys_setfsgid",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "gid_t gid",
        }],
    },
    SyscallInfo {
        nr: 124,
        name: "getsid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getsid.2.en.html",
        entry_point: "sys_getsid",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "pid_t pid",
        }],
    },
    SyscallInfo {
        nr: 125,
        name: "capget",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/capget.2.en.html",
        entry_point: "sys_capget",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "cap_user_header_t header",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "cap_user_data_t dataptr",
            },
        ],
    },
    SyscallInfo {
        nr: 126,
        name: "capset",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/capset.2.en.html",
        entry_point: "sys_capset",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "cap_user_header_t header",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const cap_user_data_t data",
            },
        ],
    },
    SyscallInfo {
        nr: 127,
        name: "rt_sigpending",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/rt_sigpending.2.en.html",
        entry_point: "sys_rt_sigpending",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "sigset_t *set",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t sigsetsize",
            },
        ],
    },
    SyscallInfo {
        nr: 128,
        name: "rt_sigtimedwait",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/rt_sigtimedwait.2.en.html",
        entry_point: "sys_rt_sigtimedwait",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const sigset_t *uthese",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "siginfo_t *uinfo",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const struct __kernel_timespec *uts",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "size_t sigsetsize",
            },
        ],
    },
    SyscallInfo {
        nr: 129,
        name: "rt_sigqueueinfo",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/rt_sigqueueinfo.2.en.html",
        entry_point: "sys_rt_sigqueueinfo",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int sig",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "siginfo_t *uinfo",
            },
        ],
    },
    SyscallInfo {
        nr: 130,
        name: "rt_sigsuspend",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/rt_sigsuspend.2.en.html",
        entry_point: "sys_rt_sigsuspend",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "sigset_t *unewset",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t sigsetsize",
            },
        ],
    },
    SyscallInfo {
        nr: 131,
        name: "sigaltstack",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sigaltstack.2.en.html",
        entry_point: "sys_sigaltstack",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const struct sigaltstack *uss",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sigaltstack *uoss",
            },
        ],
    },
    SyscallInfo {
        nr: 132,
        name: "utime",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/utime.2.en.html",
        entry_point: "sys_utime",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "char *filename",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct utimbuf *times",
            },
        ],
    },
    SyscallInfo {
        nr: 133,
        name: "mknod",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mknod.2.en.html",
        entry_point: "sys_mknod",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "umode_t mode",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned dev",
            },
        ],
    },
    SyscallInfo {
        nr: 134,
        name: "uselib",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/uselib.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 135,
        name: "personality",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/personality.2.en.html",
        entry_point: "sys_personality",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "unsigned int personality",
        }],
    },
    SyscallInfo {
        nr: 136,
        name: "ustat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/ustat.2.en.html",
        entry_point: "sys_ustat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned dev",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct ustat *ubuf",
            },
        ],
    },
    SyscallInfo {
        nr: 137,
        name: "statfs",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/statfs.2.en.html",
        entry_point: "sys_statfs",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char * path",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct statfs *buf",
            },
        ],
    },
    SyscallInfo {
        nr: 138,
        name: "fstatfs",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fstatfs.2.en.html",
        entry_point: "sys_fstatfs",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct statfs *buf",
            },
        ],
    },
    SyscallInfo {
        nr: 139,
        name: "sysfs",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sysfs.2.en.html",
        entry_point: "sys_sysfs",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int option",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long arg1",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long arg2",
            },
        ],
    },
    SyscallInfo {
        nr: 140,
        name: "getpriority",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getpriority.2.en.html",
        entry_point: "sys_getpriority",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int which",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int who",
            },
        ],
    },
    SyscallInfo {
        nr: 141,
        name: "setpriority",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setpriority.2.en.html",
        entry_point: "sys_setpriority",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int which",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int who",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int niceval",
            },
        ],
    },
    SyscallInfo {
        nr: 142,
        name: "sched_setparam",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sched_setparam.2.en.html",
        entry_point: "sys_sched_setparam",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sched_param *param",
            },
        ],
    },
    SyscallInfo {
        nr: 143,
        name: "sched_getparam",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sched_getparam.2.en.html",
        entry_point: "sys_sched_getparam",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sched_param *param",
            },
        ],
    },
    SyscallInfo {
        nr: 144,
        name: "sched_setscheduler",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sched_setscheduler.2.en.html",
        entry_point: "sys_sched_setscheduler",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int policy",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct sched_param *param",
            },
        ],
    },
    SyscallInfo {
        nr: 145,
        name: "sched_getscheduler",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sched_getscheduler.2.en.html",
        entry_point: "sys_sched_getscheduler",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "pid_t pid",
        }],
    },
    SyscallInfo {
        nr: 146,
        name: "sched_get_priority_max",
        man_url:
            "https://manpages.debian.org/unstable/manpages-dev/sched_get_priority_max.2.en.html",
        entry_point: "sys_sched_get_priority_max",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int policy",
        }],
    },
    SyscallInfo {
        nr: 147,
        name: "sched_get_priority_min",
        man_url:
            "https://manpages.debian.org/unstable/manpages-dev/sched_get_priority_min.2.en.html",
        entry_point: "sys_sched_get_priority_min",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int policy",
        }],
    },
    SyscallInfo {
        nr: 148,
        name: "sched_rr_get_interval",
        man_url:
            "https://manpages.debian.org/unstable/manpages-dev/sched_rr_get_interval.2.en.html",
        entry_point: "sys_sched_rr_get_interval",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct __kernel_timespec *interval",
            },
        ],
    },
    SyscallInfo {
        nr: 149,
        name: "mlock",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mlock.2.en.html",
        entry_point: "sys_mlock",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long start",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t len",
            },
        ],
    },
    SyscallInfo {
        nr: 150,
        name: "munlock",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/munlock.2.en.html",
        entry_point: "sys_munlock",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long start",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t len",
            },
        ],
    },
    SyscallInfo {
        nr: 151,
        name: "mlockall",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mlockall.2.en.html",
        entry_point: "sys_mlockall",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int flags",
        }],
    },
    SyscallInfo {
        nr: 152,
        name: "munlockall",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/munlockall.2.en.html",
        entry_point: "sys_munlockall",
        args: &[],
    },
    SyscallInfo {
        nr: 153,
        name: "vhangup",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/vhangup.2.en.html",
        entry_point: "sys_vhangup",
        args: &[],
    },
    SyscallInfo {
        nr: 154,
        name: "modify_ldt",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/modify_ldt.2.en.html",
        entry_point: "sys_modify_ldt",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int func",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "void *ptr",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long bytecount",
            },
        ],
    },
    SyscallInfo {
        nr: 155,
        name: "pivot_root",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pivot_root.2.en.html",
        entry_point: "sys_pivot_root",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *new_root",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *put_old",
            },
        ],
    },
    SyscallInfo {
        nr: 156,
        name: "_sysctl",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/_sysctl.2.en.html",
        entry_point: "sys_ni_syscall",
        args: &[],
    },
    SyscallInfo {
        nr: 157,
        name: "prctl",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/prctl.2.en.html",
        entry_point: "sys_prctl",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int option",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long arg2",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long arg3",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long arg4",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long arg5",
            },
        ],
    },
    SyscallInfo {
        nr: 158,
        name: "arch_prctl",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/arch_prctl.2.en.html",
        entry_point: "sys_arch_prctl",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int option",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long arg2",
            },
        ],
    },
    SyscallInfo {
        nr: 159,
        name: "adjtimex",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/adjtimex.2.en.html",
        entry_point: "sys_adjtimex",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "struct __kernel_timex *txc_p",
        }],
    },
    SyscallInfo {
        nr: 160,
        name: "setrlimit",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setrlimit.2.en.html",
        entry_point: "sys_setrlimit",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int resource",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct rlimit *rlim",
            },
        ],
    },
    SyscallInfo {
        nr: 161,
        name: "chroot",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/chroot.2.en.html",
        entry_point: "sys_chroot",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "const char *filename",
        }],
    },
    SyscallInfo {
        nr: 162,
        name: "sync",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sync.2.en.html",
        entry_point: "sys_sync",
        args: &[],
    },
    SyscallInfo {
        nr: 163,
        name: "acct",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/acct.2.en.html",
        entry_point: "sys_acct",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "const char *name",
        }],
    },
    SyscallInfo {
        nr: 164,
        name: "settimeofday",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/settimeofday.2.en.html",
        entry_point: "sys_settimeofday",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "struct __kernel_old_timeval *tv",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct timezone *tz",
            },
        ],
    },
    SyscallInfo {
        nr: 165,
        name: "mount",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mount.2.en.html",
        entry_point: "sys_mount",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "char *dev_name",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "char *dir_name",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "char *type",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long flags",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "void *data",
            },
        ],
    },
    SyscallInfo {
        nr: 166,
        name: "umount2",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/umount2.2.en.html",
        entry_point: "sys_umount",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "char *name",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 167,
        name: "swapon",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/swapon.2.en.html",
        entry_point: "sys_swapon",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *specialfile",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int swap_flags",
            },
        ],
    },
    SyscallInfo {
        nr: 168,
        name: "swapoff",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/swapoff.2.en.html",
        entry_point: "sys_swapoff",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "const char *specialfile",
        }],
    },
    SyscallInfo {
        nr: 169,
        name: "reboot",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/reboot.2.en.html",
        entry_point: "sys_reboot",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int magic1",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int magic2",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int cmd",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "void *arg",
            },
        ],
    },
    SyscallInfo {
        nr: 170,
        name: "sethostname",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sethostname.2.en.html",
        entry_point: "sys_sethostname",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "char *name",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int len",
            },
        ],
    },
    SyscallInfo {
        nr: 171,
        name: "setdomainname",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setdomainname.2.en.html",
        entry_point: "sys_setdomainname",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "char *name",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int len",
            },
        ],
    },
    SyscallInfo {
        nr: 172,
        name: "iopl",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/iopl.2.en.html",
        entry_point: "sys_iopl",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "unsigned int level",
        }],
    },
    SyscallInfo {
        nr: 173,
        name: "ioperm",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/ioperm.2.en.html",
        entry_point: "sys_ioperm",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long from",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long num",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int on",
            },
        ],
    },
    SyscallInfo {
        nr: 174,
        name: "create_module",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/create_module.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 175,
        name: "init_module",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/init_module.2.en.html",
        entry_point: "sys_init_module",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "void *umod",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const char *uargs",
            },
        ],
    },
    SyscallInfo {
        nr: 176,
        name: "delete_module",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/delete_module.2.en.html",
        entry_point: "sys_delete_module",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *name_user",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 177,
        name: "get_kernel_syms",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/get_kernel_syms.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 178,
        name: "query_module",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/query_module.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 179,
        name: "quotactl",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/quotactl.2.en.html",
        entry_point: "sys_quotactl",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int cmd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *special",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "qid_t id",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "void *addr",
            },
        ],
    },
    SyscallInfo {
        nr: 180,
        name: "nfsservctl",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/nfsservctl.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 181,
        name: "getpmsg",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getpmsg.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 182,
        name: "putpmsg",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/putpmsg.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 183,
        name: "afs_syscall",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/afs_syscall.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 184,
        name: "tuxcall",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/tuxcall.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 185,
        name: "security",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/security.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 186,
        name: "gettid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/gettid.2.en.html",
        entry_point: "sys_gettid",
        args: &[],
    },
    SyscallInfo {
        nr: 187,
        name: "readahead",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/readahead.2.en.html",
        entry_point: "sys_readahead",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "loff_t offset",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t count",
            },
        ],
    },
    SyscallInfo {
        nr: 188,
        name: "setxattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setxattr.2.en.html",
        entry_point: "sys_setxattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *name",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const void *value",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "size_t size",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 189,
        name: "lsetxattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/lsetxattr.2.en.html",
        entry_point: "sys_lsetxattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *name",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const void *value",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "size_t size",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 190,
        name: "fsetxattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fsetxattr.2.en.html",
        entry_point: "sys_fsetxattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *name",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const void *value",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "size_t size",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 191,
        name: "getxattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getxattr.2.en.html",
        entry_point: "sys_getxattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *name",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "void *value",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "size_t size",
            },
        ],
    },
    SyscallInfo {
        nr: 192,
        name: "lgetxattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/lgetxattr.2.en.html",
        entry_point: "sys_lgetxattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *name",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "void *value",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "size_t size",
            },
        ],
    },
    SyscallInfo {
        nr: 193,
        name: "fgetxattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fgetxattr.2.en.html",
        entry_point: "sys_fgetxattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *name",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "void *value",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "size_t size",
            },
        ],
    },
    SyscallInfo {
        nr: 194,
        name: "listxattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/listxattr.2.en.html",
        entry_point: "sys_listxattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "char *list",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t size",
            },
        ],
    },
    SyscallInfo {
        nr: 195,
        name: "llistxattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/llistxattr.2.en.html",
        entry_point: "sys_llistxattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "char *list",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t size",
            },
        ],
    },
    SyscallInfo {
        nr: 196,
        name: "flistxattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/flistxattr.2.en.html",
        entry_point: "sys_flistxattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "char *list",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t size",
            },
        ],
    },
    SyscallInfo {
        nr: 197,
        name: "removexattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/removexattr.2.en.html",
        entry_point: "sys_removexattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *name",
            },
        ],
    },
    SyscallInfo {
        nr: 198,
        name: "lremovexattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/lremovexattr.2.en.html",
        entry_point: "sys_lremovexattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *name",
            },
        ],
    },
    SyscallInfo {
        nr: 199,
        name: "fremovexattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fremovexattr.2.en.html",
        entry_point: "sys_fremovexattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *name",
            },
        ],
    },
    SyscallInfo {
        nr: 200,
        name: "tkill",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/tkill.2.en.html",
        entry_point: "sys_tkill",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int sig",
            },
        ],
    },
    SyscallInfo {
        nr: 201,
        name: "time",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/time.2.en.html",
        entry_point: "sys_time",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "__kernel_old_time_t *tloc",
        }],
    },
    SyscallInfo {
        nr: 202,
        name: "futex",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/futex.2.en.html",
        entry_point: "sys_futex",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "u32 *uaddr",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int op",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "u32 val",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const struct __kernel_timespec *utime",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "u32 *uaddr2",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "u32 val3",
            },
        ],
    },
    SyscallInfo {
        nr: 203,
        name: "sched_setaffinity",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sched_setaffinity.2.en.html",
        entry_point: "sys_sched_setaffinity",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long *user_mask_ptr",
            },
        ],
    },
    SyscallInfo {
        nr: 204,
        name: "sched_getaffinity",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sched_getaffinity.2.en.html",
        entry_point: "sys_sched_getaffinity",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long *user_mask_ptr",
            },
        ],
    },
    SyscallInfo {
        nr: 205,
        name: "set_thread_area",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/set_thread_area.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 206,
        name: "io_setup",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/io_setup.2.en.html",
        entry_point: "sys_io_setup",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned nr_reqs",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "aio_context_t *ctx",
            },
        ],
    },
    SyscallInfo {
        nr: 207,
        name: "io_destroy",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/io_destroy.2.en.html",
        entry_point: "sys_io_destroy",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "aio_context_t ctx",
        }],
    },
    SyscallInfo {
        nr: 208,
        name: "io_getevents",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/io_getevents.2.en.html",
        entry_point: "sys_io_getevents",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "aio_context_t ctx_id",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "long min_nr",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "long nr",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "struct io_event *events",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "struct __kernel_timespec *timeout",
            },
        ],
    },
    SyscallInfo {
        nr: 209,
        name: "io_submit",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/io_submit.2.en.html",
        entry_point: "sys_io_submit",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "aio_context_t",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "long",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct iocb * *",
            },
        ],
    },
    SyscallInfo {
        nr: 210,
        name: "io_cancel",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/io_cancel.2.en.html",
        entry_point: "sys_io_cancel",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "aio_context_t ctx_id",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct iocb *iocb",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct io_event *result",
            },
        ],
    },
    SyscallInfo {
        nr: 211,
        name: "get_thread_area",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/get_thread_area.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 212,
        name: "lookup_dcookie",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/lookup_dcookie.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 213,
        name: "epoll_create",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/epoll_create.2.en.html",
        entry_point: "sys_epoll_create",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int size",
        }],
    },
    SyscallInfo {
        nr: 214,
        name: "epoll_ctl_old",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/epoll_ctl_old.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 215,
        name: "epoll_wait_old",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/epoll_wait_old.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 216,
        name: "remap_file_pages",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/remap_file_pages.2.en.html",
        entry_point: "sys_remap_file_pages",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long start",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long size",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long prot",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long pgoff",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long flags",
            },
        ],
    },
    SyscallInfo {
        nr: 217,
        name: "getdents64",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getdents64.2.en.html",
        entry_point: "sys_getdents64",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct linux_dirent64 *dirent",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int count",
            },
        ],
    },
    SyscallInfo {
        nr: 218,
        name: "set_tid_address",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/set_tid_address.2.en.html",
        entry_point: "sys_set_tid_address",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int *tidptr",
        }],
    },
    SyscallInfo {
        nr: 219,
        name: "restart_syscall",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/restart_syscall.2.en.html",
        entry_point: "sys_restart_syscall",
        args: &[],
    },
    SyscallInfo {
        nr: 220,
        name: "semtimedop",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/semtimedop.2.en.html",
        entry_point: "sys_semtimedop",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int semid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sembuf *sops",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned nsops",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const struct __kernel_timespec *timeout",
            },
        ],
    },
    SyscallInfo {
        nr: 221,
        name: "fadvise64",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fadvise64.2.en.html",
        entry_point: "sys_fadvise64",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "loff_t offset",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t len",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int advice",
            },
        ],
    },
    SyscallInfo {
        nr: 222,
        name: "timer_create",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/timer_create.2.en.html",
        entry_point: "sys_timer_create",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "clockid_t which_clock",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sigevent *timer_event_spec",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "timer_t * created_timer_id",
            },
        ],
    },
    SyscallInfo {
        nr: 223,
        name: "timer_settime",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/timer_settime.2.en.html",
        entry_point: "sys_timer_settime",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "timer_t timer_id",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int flags",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const struct __kernel_itimerspec *new_setting",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "struct __kernel_itimerspec *old_setting",
            },
        ],
    },
    SyscallInfo {
        nr: 224,
        name: "timer_gettime",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/timer_gettime.2.en.html",
        entry_point: "sys_timer_gettime",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "timer_t timer_id",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct __kernel_itimerspec *setting",
            },
        ],
    },
    SyscallInfo {
        nr: 225,
        name: "timer_getoverrun",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/timer_getoverrun.2.en.html",
        entry_point: "sys_timer_getoverrun",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "timer_t timer_id",
        }],
    },
    SyscallInfo {
        nr: 226,
        name: "timer_delete",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/timer_delete.2.en.html",
        entry_point: "sys_timer_delete",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "timer_t timer_id",
        }],
    },
    SyscallInfo {
        nr: 227,
        name: "clock_settime",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/clock_settime.2.en.html",
        entry_point: "sys_clock_settime",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "clockid_t which_clock",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct __kernel_timespec *tp",
            },
        ],
    },
    SyscallInfo {
        nr: 228,
        name: "clock_gettime",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/clock_gettime.2.en.html",
        entry_point: "sys_clock_gettime",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "clockid_t which_clock",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct __kernel_timespec *tp",
            },
        ],
    },
    SyscallInfo {
        nr: 229,
        name: "clock_getres",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/clock_getres.2.en.html",
        entry_point: "sys_clock_getres",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "clockid_t which_clock",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct __kernel_timespec *tp",
            },
        ],
    },
    SyscallInfo {
        nr: 230,
        name: "clock_nanosleep",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/clock_nanosleep.2.en.html",
        entry_point: "sys_clock_nanosleep",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "clockid_t which_clock",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int flags",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const struct __kernel_timespec *rqtp",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "struct __kernel_timespec *rmtp",
            },
        ],
    },
    SyscallInfo {
        nr: 231,
        name: "exit_group",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/exit_group.2.en.html",
        entry_point: "sys_exit_group",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int error_code",
        }],
    },
    SyscallInfo {
        nr: 232,
        name: "epoll_wait",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/epoll_wait.2.en.html",
        entry_point: "sys_epoll_wait",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int epfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct epoll_event *events",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int maxevents",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int timeout",
            },
        ],
    },
    SyscallInfo {
        nr: 233,
        name: "epoll_ctl",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/epoll_ctl.2.en.html",
        entry_point: "sys_epoll_ctl",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int epfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int op",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "struct epoll_event *event",
            },
        ],
    },
    SyscallInfo {
        nr: 234,
        name: "tgkill",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/tgkill.2.en.html",
        entry_point: "sys_tgkill",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t tgid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int sig",
            },
        ],
    },
    SyscallInfo {
        nr: 235,
        name: "utimes",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/utimes.2.en.html",
        entry_point: "sys_utimes",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "char *filename",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct __kernel_old_timeval *utimes",
            },
        ],
    },
    SyscallInfo {
        nr: 236,
        name: "vserver",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/vserver.2.en.html",
        entry_point: "",
        args: &[],
    },
    SyscallInfo {
        nr: 237,
        name: "mbind",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mbind.2.en.html",
        entry_point: "sys_mbind",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long start",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long mode",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const unsigned long *nmask",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long maxnode",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "unsigned flags",
            },
        ],
    },
    SyscallInfo {
        nr: 238,
        name: "set_mempolicy",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/set_mempolicy.2.en.html",
        entry_point: "sys_set_mempolicy",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int mode",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const unsigned long *nmask",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long maxnode",
            },
        ],
    },
    SyscallInfo {
        nr: 239,
        name: "get_mempolicy",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/get_mempolicy.2.en.html",
        entry_point: "sys_get_mempolicy",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int *policy",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long *nmask",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long maxnode",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long addr",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long flags",
            },
        ],
    },
    SyscallInfo {
        nr: 240,
        name: "mq_open",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mq_open.2.en.html",
        entry_point: "sys_mq_open",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *name",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int oflag",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "umode_t mode",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "struct mq_attr *attr",
            },
        ],
    },
    SyscallInfo {
        nr: 241,
        name: "mq_unlink",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mq_unlink.2.en.html",
        entry_point: "sys_mq_unlink",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "const char *name",
        }],
    },
    SyscallInfo {
        nr: 242,
        name: "mq_timedsend",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mq_timedsend.2.en.html",
        entry_point: "sys_mq_timedsend",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "mqd_t mqdes",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *msg_ptr",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t msg_len",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int msg_prio",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "const struct __kernel_timespec *abs_timeout",
            },
        ],
    },
    SyscallInfo {
        nr: 243,
        name: "mq_timedreceive",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mq_timedreceive.2.en.html",
        entry_point: "sys_mq_timedreceive",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "mqd_t mqdes",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "char *msg_ptr",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t msg_len",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int *msg_prio",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "const struct __kernel_timespec *abs_timeout",
            },
        ],
    },
    SyscallInfo {
        nr: 244,
        name: "mq_notify",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mq_notify.2.en.html",
        entry_point: "sys_mq_notify",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "mqd_t mqdes",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct sigevent *notification",
            },
        ],
    },
    SyscallInfo {
        nr: 245,
        name: "mq_getsetattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mq_getsetattr.2.en.html",
        entry_point: "sys_mq_getsetattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "mqd_t mqdes",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct mq_attr *mqstat",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct mq_attr *omqstat",
            },
        ],
    },
    SyscallInfo {
        nr: 246,
        name: "kexec_load",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/kexec_load.2.en.html",
        entry_point: "sys_kexec_load",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long entry",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long nr_segments",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct kexec_segment *segments",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long flags",
            },
        ],
    },
    SyscallInfo {
        nr: 247,
        name: "waitid",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/waitid.2.en.html",
        entry_point: "sys_waitid",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int which",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct siginfo *infop",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int options",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "struct rusage *ru",
            },
        ],
    },
    SyscallInfo {
        nr: 248,
        name: "add_key",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/add_key.2.en.html",
        entry_point: "sys_add_key",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *_type",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *_description",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const void *_payload",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "size_t plen",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "key_serial_t destringid",
            },
        ],
    },
    SyscallInfo {
        nr: 249,
        name: "request_key",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/request_key.2.en.html",
        entry_point: "sys_request_key",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *_type",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *_description",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const char *_callout_info",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "key_serial_t destringid",
            },
        ],
    },
    SyscallInfo {
        nr: 250,
        name: "keyctl",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/keyctl.2.en.html",
        entry_point: "sys_keyctl",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int cmd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long arg2",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long arg3",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long arg4",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long arg5",
            },
        ],
    },
    SyscallInfo {
        nr: 251,
        name: "ioprio_set",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/ioprio_set.2.en.html",
        entry_point: "sys_ioprio_set",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int which",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int who",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int ioprio",
            },
        ],
    },
    SyscallInfo {
        nr: 252,
        name: "ioprio_get",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/ioprio_get.2.en.html",
        entry_point: "sys_ioprio_get",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int which",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int who",
            },
        ],
    },
    SyscallInfo {
        nr: 253,
        name: "inotify_init",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/inotify_init.2.en.html",
        entry_point: "sys_inotify_init",
        args: &[],
    },
    SyscallInfo {
        nr: 254,
        name: "inotify_add_watch",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/inotify_add_watch.2.en.html",
        entry_point: "sys_inotify_add_watch",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "u32 mask",
            },
        ],
    },
    SyscallInfo {
        nr: 255,
        name: "inotify_rm_watch",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/inotify_rm_watch.2.en.html",
        entry_point: "sys_inotify_rm_watch",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "__s32 wd",
            },
        ],
    },
    SyscallInfo {
        nr: 256,
        name: "migrate_pages",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/migrate_pages.2.en.html",
        entry_point: "sys_migrate_pages",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long maxnode",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const unsigned long *from",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const unsigned long *to",
            },
        ],
    },
    SyscallInfo {
        nr: 257,
        name: "openat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/openat.2.en.html",
        entry_point: "sys_openat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int flags",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "umode_t mode",
            },
        ],
    },
    SyscallInfo {
        nr: 258,
        name: "mkdirat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mkdirat.2.en.html",
        entry_point: "sys_mkdirat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char * pathname",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "umode_t mode",
            },
        ],
    },
    SyscallInfo {
        nr: 259,
        name: "mknodat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mknodat.2.en.html",
        entry_point: "sys_mknodat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char * filename",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "umode_t mode",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned dev",
            },
        ],
    },
    SyscallInfo {
        nr: 260,
        name: "fchownat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fchownat.2.en.html",
        entry_point: "sys_fchownat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "uid_t user",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "gid_t group",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int flag",
            },
        ],
    },
    SyscallInfo {
        nr: 261,
        name: "futimesat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/futimesat.2.en.html",
        entry_point: "sys_futimesat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct __kernel_old_timeval *utimes",
            },
        ],
    },
    SyscallInfo {
        nr: 262,
        name: "newfstatat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/newfstatat.2.en.html",
        entry_point: "sys_newfstatat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct stat *statbuf",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int flag",
            },
        ],
    },
    SyscallInfo {
        nr: 263,
        name: "unlinkat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/unlinkat.2.en.html",
        entry_point: "sys_unlinkat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char * pathname",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int flag",
            },
        ],
    },
    SyscallInfo {
        nr: 264,
        name: "renameat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/renameat.2.en.html",
        entry_point: "sys_renameat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int olddfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char * oldname",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int newdfd",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const char * newname",
            },
        ],
    },
    SyscallInfo {
        nr: 265,
        name: "linkat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/linkat.2.en.html",
        entry_point: "sys_linkat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int olddfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *oldname",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int newdfd",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const char *newname",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 266,
        name: "symlinkat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/symlinkat.2.en.html",
        entry_point: "sys_symlinkat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char * oldname",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int newdfd",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const char * newname",
            },
        ],
    },
    SyscallInfo {
        nr: 267,
        name: "readlinkat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/readlinkat.2.en.html",
        entry_point: "sys_readlinkat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "char *buf",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int bufsiz",
            },
        ],
    },
    SyscallInfo {
        nr: 268,
        name: "fchmodat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fchmodat.2.en.html",
        entry_point: "sys_fchmodat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "umode_t mode",
            },
        ],
    },
    SyscallInfo {
        nr: 269,
        name: "faccessat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/faccessat.2.en.html",
        entry_point: "sys_faccessat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int mode",
            },
        ],
    },
    SyscallInfo {
        nr: 270,
        name: "pselect6",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pselect6.2.en.html",
        entry_point: "sys_pselect6",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "fd_set *",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "fd_set *",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "fd_set *",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "struct __kernel_timespec *",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "void *",
            },
        ],
    },
    SyscallInfo {
        nr: 271,
        name: "ppoll",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/ppoll.2.en.html",
        entry_point: "sys_ppoll",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "struct pollfd *",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct __kernel_timespec *",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const sigset_t *",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "size_t",
            },
        ],
    },
    SyscallInfo {
        nr: 272,
        name: "unshare",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/unshare.2.en.html",
        entry_point: "sys_unshare",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "unsigned long unshare_flags",
        }],
    },
    SyscallInfo {
        nr: 273,
        name: "set_robust_list",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/set_robust_list.2.en.html",
        entry_point: "sys_set_robust_list",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "struct robust_list_head *head",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t len",
            },
        ],
    },
    SyscallInfo {
        nr: 274,
        name: "get_robust_list",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/get_robust_list.2.en.html",
        entry_point: "sys_get_robust_list",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct robust_list_head * *head_ptr",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t *len_ptr",
            },
        ],
    },
    SyscallInfo {
        nr: 275,
        name: "splice",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/splice.2.en.html",
        entry_point: "sys_splice",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd_in",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "loff_t *off_in",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int fd_out",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "loff_t *off_out",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "size_t len",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 276,
        name: "tee",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/tee.2.en.html",
        entry_point: "sys_tee",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fdin",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int fdout",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t len",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 277,
        name: "sync_file_range",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sync_file_range.2.en.html",
        entry_point: "sys_sync_file_range",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "loff_t offset",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "loff_t nbytes",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 278,
        name: "vmsplice",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/vmsplice.2.en.html",
        entry_point: "sys_vmsplice",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct iovec *iov",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long nr_segs",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 279,
        name: "move_pages",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/move_pages.2.en.html",
        entry_point: "sys_move_pages",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long nr_pages",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const void * *pages",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const int *nodes",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int *status",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 280,
        name: "utimensat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/utimensat.2.en.html",
        entry_point: "sys_utimensat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct __kernel_timespec *utimes",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 281,
        name: "epoll_pwait",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/epoll_pwait.2.en.html",
        entry_point: "sys_epoll_pwait",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int epfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct epoll_event *events",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int maxevents",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int timeout",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "const sigset_t *sigmask",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "size_t sigsetsize",
            },
        ],
    },
    SyscallInfo {
        nr: 282,
        name: "signalfd",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/signalfd.2.en.html",
        entry_point: "sys_signalfd",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int ufd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "sigset_t *user_mask",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t sizemask",
            },
        ],
    },
    SyscallInfo {
        nr: 283,
        name: "timerfd_create",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/timerfd_create.2.en.html",
        entry_point: "sys_timerfd_create",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int clockid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 284,
        name: "eventfd",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/eventfd.2.en.html",
        entry_point: "sys_eventfd",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "unsigned int count",
        }],
    },
    SyscallInfo {
        nr: 285,
        name: "fallocate",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fallocate.2.en.html",
        entry_point: "sys_fallocate",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int mode",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "loff_t offset",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "loff_t len",
            },
        ],
    },
    SyscallInfo {
        nr: 286,
        name: "timerfd_settime",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/timerfd_settime.2.en.html",
        entry_point: "sys_timerfd_settime",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int ufd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int flags",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const struct __kernel_itimerspec *utmr",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "struct __kernel_itimerspec *otmr",
            },
        ],
    },
    SyscallInfo {
        nr: 287,
        name: "timerfd_gettime",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/timerfd_gettime.2.en.html",
        entry_point: "sys_timerfd_gettime",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int ufd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct __kernel_itimerspec *otmr",
            },
        ],
    },
    SyscallInfo {
        nr: 288,
        name: "accept4",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/accept4.2.en.html",
        entry_point: "sys_accept4",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sockaddr *",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int *",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int",
            },
        ],
    },
    SyscallInfo {
        nr: 289,
        name: "signalfd4",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/signalfd4.2.en.html",
        entry_point: "sys_signalfd4",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int ufd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "sigset_t *user_mask",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t sizemask",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 290,
        name: "eventfd2",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/eventfd2.2.en.html",
        entry_point: "sys_eventfd2",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int count",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 291,
        name: "epoll_create1",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/epoll_create1.2.en.html",
        entry_point: "sys_epoll_create1",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int flags",
        }],
    },
    SyscallInfo {
        nr: 292,
        name: "dup3",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/dup3.2.en.html",
        entry_point: "sys_dup3",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int oldfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int newfd",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 293,
        name: "pipe2",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pipe2.2.en.html",
        entry_point: "sys_pipe2",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int *fildes",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 294,
        name: "inotify_init1",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/inotify_init1.2.en.html",
        entry_point: "sys_inotify_init1",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int flags",
        }],
    },
    SyscallInfo {
        nr: 295,
        name: "preadv",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/preadv.2.en.html",
        entry_point: "sys_preadv",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct iovec *vec",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long vlen",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long pos_l",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long pos_h",
            },
        ],
    },
    SyscallInfo {
        nr: 296,
        name: "pwritev",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pwritev.2.en.html",
        entry_point: "sys_pwritev",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct iovec *vec",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long vlen",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long pos_l",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long pos_h",
            },
        ],
    },
    SyscallInfo {
        nr: 297,
        name: "rt_tgsigqueueinfo",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/rt_tgsigqueueinfo.2.en.html",
        entry_point: "sys_rt_tgsigqueueinfo",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t tgid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "pid_t  pid",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int sig",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "siginfo_t *uinfo",
            },
        ],
    },
    SyscallInfo {
        nr: 298,
        name: "perf_event_open",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/perf_event_open.2.en.html",
        entry_point: "sys_perf_event_open",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "struct perf_event_attr *attr_uptr",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int cpu",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int group_fd",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long flags",
            },
        ],
    },
    SyscallInfo {
        nr: 299,
        name: "recvmmsg",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/recvmmsg.2.en.html",
        entry_point: "sys_recvmmsg",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct mmsghdr *msg",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int vlen",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned flags",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "struct __kernel_timespec *timeout",
            },
        ],
    },
    SyscallInfo {
        nr: 300,
        name: "fanotify_init",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fanotify_init.2.en.html",
        entry_point: "sys_fanotify_init",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int flags",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int event_f_flags",
            },
        ],
    },
    SyscallInfo {
        nr: 301,
        name: "fanotify_mark",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fanotify_mark.2.en.html",
        entry_point: "sys_fanotify_mark",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fanotify_fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int flags",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int mask_1",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int mask_2",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "const char  * pathname",
            },
        ],
    },
    SyscallInfo {
        nr: 302,
        name: "prlimit64",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/prlimit64.2.en.html",
        entry_point: "sys_prlimit64",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int resource",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const struct rlimit64 *new_rlim",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "struct rlimit64 *old_rlim",
            },
        ],
    },
    SyscallInfo {
        nr: 303,
        name: "name_to_handle_at",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/name_to_handle_at.2.en.html",
        entry_point: "sys_name_to_handle_at",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *name",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct file_handle *handle",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "void *mnt_id",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int flag",
            },
        ],
    },
    SyscallInfo {
        nr: 304,
        name: "open_by_handle_at",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/open_by_handle_at.2.en.html",
        entry_point: "sys_open_by_handle_at",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int mountdirfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct file_handle *handle",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 305,
        name: "clock_adjtime",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/clock_adjtime.2.en.html",
        entry_point: "sys_clock_adjtime",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "clockid_t which_clock",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct __kernel_timex *tx",
            },
        ],
    },
    SyscallInfo {
        nr: 306,
        name: "syncfs",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/syncfs.2.en.html",
        entry_point: "sys_syncfs",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int fd",
        }],
    },
    SyscallInfo {
        nr: 307,
        name: "sendmmsg",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sendmmsg.2.en.html",
        entry_point: "sys_sendmmsg",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct mmsghdr *msg",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int vlen",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned flags",
            },
        ],
    },
    SyscallInfo {
        nr: 308,
        name: "setns",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setns.2.en.html",
        entry_point: "sys_setns",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int nstype",
            },
        ],
    },
    SyscallInfo {
        nr: 309,
        name: "getcpu",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getcpu.2.en.html",
        entry_point: "sys_getcpu",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned *cpu",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned *node",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct getcpu_cache *cache",
            },
        ],
    },
    SyscallInfo {
        nr: 310,
        name: "process_vm_readv",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/process_vm_readv.2.en.html",
        entry_point: "sys_process_vm_readv",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct iovec *lvec",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long liovcnt",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const struct iovec *rvec",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long riovcnt",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "unsigned long flags",
            },
        ],
    },
    SyscallInfo {
        nr: 311,
        name: "process_vm_writev",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/process_vm_writev.2.en.html",
        entry_point: "sys_process_vm_writev",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct iovec *lvec",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long liovcnt",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const struct iovec *rvec",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long riovcnt",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "unsigned long flags",
            },
        ],
    },
    SyscallInfo {
        nr: 312,
        name: "kcmp",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/kcmp.2.en.html",
        entry_point: "sys_kcmp",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid1",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "pid_t pid2",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int type",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long idx1",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long idx2",
            },
        ],
    },
    SyscallInfo {
        nr: 313,
        name: "finit_module",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/finit_module.2.en.html",
        entry_point: "sys_finit_module",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *uargs",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 314,
        name: "sched_setattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sched_setattr.2.en.html",
        entry_point: "sys_sched_setattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sched_attr *attr",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 315,
        name: "sched_getattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/sched_getattr.2.en.html",
        entry_point: "sys_sched_getattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct sched_attr *attr",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int size",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 316,
        name: "renameat2",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/renameat2.2.en.html",
        entry_point: "sys_renameat2",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int olddfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *oldname",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int newdfd",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const char *newname",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 317,
        name: "seccomp",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/seccomp.2.en.html",
        entry_point: "sys_seccomp",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int op",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int flags",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "void *uargs",
            },
        ],
    },
    SyscallInfo {
        nr: 318,
        name: "getrandom",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getrandom.2.en.html",
        entry_point: "sys_getrandom",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "char *buf",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t count",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 319,
        name: "memfd_create",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/memfd_create.2.en.html",
        entry_point: "sys_memfd_create",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *uname_ptr",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 320,
        name: "kexec_file_load",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/kexec_file_load.2.en.html",
        entry_point: "sys_kexec_file_load",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int kernel_fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int initrd_fd",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long cmdline_len",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const char *cmdline_ptr",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long flags",
            },
        ],
    },
    SyscallInfo {
        nr: 321,
        name: "bpf",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/bpf.2.en.html",
        entry_point: "sys_bpf",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int cmd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "union bpf_attr *attr",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int size",
            },
        ],
    },
    SyscallInfo {
        nr: 322,
        name: "execveat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/execveat.2.en.html",
        entry_point: "sys_execveat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const char *const *argv",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const char *const *envp",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 323,
        name: "userfaultfd",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/userfaultfd.2.en.html",
        entry_point: "sys_userfaultfd",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int flags",
        }],
    },
    SyscallInfo {
        nr: 324,
        name: "membarrier",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/membarrier.2.en.html",
        entry_point: "sys_membarrier",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int cmd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int flags",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int cpu_id",
            },
        ],
    },
    SyscallInfo {
        nr: 325,
        name: "mlock2",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mlock2.2.en.html",
        entry_point: "sys_mlock2",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long start",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 326,
        name: "copy_file_range",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/copy_file_range.2.en.html",
        entry_point: "sys_copy_file_range",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fd_in",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "loff_t *off_in",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int fd_out",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "loff_t *off_out",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "size_t len",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 327,
        name: "preadv2",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/preadv2.2.en.html",
        entry_point: "sys_preadv2",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct iovec *vec",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long vlen",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long pos_l",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long pos_h",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "rwf_t flags",
            },
        ],
    },
    SyscallInfo {
        nr: 328,
        name: "pwritev2",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pwritev2.2.en.html",
        entry_point: "sys_pwritev2",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct iovec *vec",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long vlen",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long pos_l",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned long pos_h",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "rwf_t flags",
            },
        ],
    },
    SyscallInfo {
        nr: 329,
        name: "pkey_mprotect",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pkey_mprotect.2.en.html",
        entry_point: "sys_pkey_mprotect",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long start",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long prot",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int pkey",
            },
        ],
    },
    SyscallInfo {
        nr: 330,
        name: "pkey_alloc",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pkey_alloc.2.en.html",
        entry_point: "sys_pkey_alloc",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long flags",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long init_val",
            },
        ],
    },
    SyscallInfo {
        nr: 331,
        name: "pkey_free",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pkey_free.2.en.html",
        entry_point: "sys_pkey_free",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "int pkey",
        }],
    },
    SyscallInfo {
        nr: 332,
        name: "statx",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/statx.2.en.html",
        entry_point: "sys_statx",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned flags",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned mask",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "struct statx *buffer",
            },
        ],
    },
    SyscallInfo {
        nr: 333,
        name: "io_pgetevents",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/io_pgetevents.2.en.html",
        entry_point: "sys_io_pgetevents",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "aio_context_t ctx_id",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "long min_nr",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "long nr",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "struct io_event *events",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "struct __kernel_timespec *timeout",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "const struct __aio_sigset *sig",
            },
        ],
    },
    SyscallInfo {
        nr: 334,
        name: "rseq",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/rseq.2.en.html",
        entry_point: "sys_rseq",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "struct rseq *rseq",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "uint32_t rseq_len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int flags",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "uint32_t sig",
            },
        ],
    },
    SyscallInfo {
        nr: 335,
        name: "uretprobe",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/uretprobe.2.en.html",
        entry_point: "sys_uretprobe",
        args: &[],
    },
    SyscallInfo {
        nr: 424,
        name: "pidfd_send_signal",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pidfd_send_signal.2.en.html",
        entry_point: "sys_pidfd_send_signal",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int pidfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int sig",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "siginfo_t *info",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 425,
        name: "io_uring_setup",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/io_uring_setup.2.en.html",
        entry_point: "sys_io_uring_setup",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "u32 entries",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct io_uring_params *p",
            },
        ],
    },
    SyscallInfo {
        nr: 426,
        name: "io_uring_enter",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/io_uring_enter.2.en.html",
        entry_point: "sys_io_uring_enter",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "u32 to_submit",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "u32 min_complete",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "u32 flags",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "const void *argp",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "size_t argsz",
            },
        ],
    },
    SyscallInfo {
        nr: 427,
        name: "io_uring_register",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/io_uring_register.2.en.html",
        entry_point: "sys_io_uring_register",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int op",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "void *arg",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int nr_args",
            },
        ],
    },
    SyscallInfo {
        nr: 428,
        name: "open_tree",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/open_tree.2.en.html",
        entry_point: "sys_open_tree",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned flags",
            },
        ],
    },
    SyscallInfo {
        nr: 429,
        name: "move_mount",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/move_mount.2.en.html",
        entry_point: "sys_move_mount",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int from_dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *from_path",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int to_dfd",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const char *to_path",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned int ms_flags",
            },
        ],
    },
    SyscallInfo {
        nr: 430,
        name: "fsopen",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fsopen.2.en.html",
        entry_point: "sys_fsopen",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const char *fs_name",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 431,
        name: "fsconfig",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fsconfig.2.en.html",
        entry_point: "sys_fsconfig",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fs_fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int cmd",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const char *key",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const void *value",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "int aux",
            },
        ],
    },
    SyscallInfo {
        nr: 432,
        name: "fsmount",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fsmount.2.en.html",
        entry_point: "sys_fsmount",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int fs_fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int flags",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int ms_flags",
            },
        ],
    },
    SyscallInfo {
        nr: 433,
        name: "fspick",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fspick.2.en.html",
        entry_point: "sys_fspick",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 434,
        name: "pidfd_open",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pidfd_open.2.en.html",
        entry_point: "sys_pidfd_open",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "pid_t pid",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 435,
        name: "clone3",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/clone3.2.en.html",
        entry_point: "sys_clone3",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "struct clone_args *uargs",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t size",
            },
        ],
    },
    SyscallInfo {
        nr: 436,
        name: "close_range",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/close_range.2.en.html",
        entry_point: "sys_close_range",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int max_fd",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 437,
        name: "openat2",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/openat2.2.en.html",
        entry_point: "sys_openat2",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct open_how *how",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "size_t size",
            },
        ],
    },
    SyscallInfo {
        nr: 438,
        name: "pidfd_getfd",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/pidfd_getfd.2.en.html",
        entry_point: "sys_pidfd_getfd",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int pidfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "int fd",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 439,
        name: "faccessat2",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/faccessat2.2.en.html",
        entry_point: "sys_faccessat2",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int mode",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 440,
        name: "process_madvise",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/process_madvise.2.en.html",
        entry_point: "sys_process_madvise",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int pidfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const struct iovec *vec",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t vlen",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int behavior",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 441,
        name: "epoll_pwait2",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/epoll_pwait2.2.en.html",
        entry_point: "sys_epoll_pwait2",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int epfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct epoll_event *events",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int maxevents",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const struct __kernel_timespec *timeout",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "const sigset_t *sigmask",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "size_t sigsetsize",
            },
        ],
    },
    SyscallInfo {
        nr: 442,
        name: "mount_setattr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mount_setattr.2.en.html",
        entry_point: "sys_mount_setattr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int flags",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "struct mount_attr *uattr",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "size_t usize",
            },
        ],
    },
    SyscallInfo {
        nr: 443,
        name: "quotactl_fd",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/quotactl_fd.2.en.html",
        entry_point: "sys_quotactl_fd",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int cmd",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "qid_t id",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "void *addr",
            },
        ],
    },
    SyscallInfo {
        nr: 444,
        name: "landlock_create_ruleset",
        man_url:
            "https://manpages.debian.org/unstable/manpages-dev/landlock_create_ruleset.2.en.html",
        entry_point: "sys_landlock_create_ruleset",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const struct landlock_ruleset_attr *attr",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t size",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "__u32 flags",
            },
        ],
    },
    SyscallInfo {
        nr: 445,
        name: "landlock_add_rule",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/landlock_add_rule.2.en.html",
        entry_point: "sys_landlock_add_rule",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int ruleset_fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "enum landlock_rule_type rule_type",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "const void *rule_attr",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "__u32 flags",
            },
        ],
    },
    SyscallInfo {
        nr: 446,
        name: "landlock_restrict_self",
        man_url:
            "https://manpages.debian.org/unstable/manpages-dev/landlock_restrict_self.2.en.html",
        entry_point: "sys_landlock_restrict_self",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int ruleset_fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "__u32 flags",
            },
        ],
    },
    SyscallInfo {
        nr: 447,
        name: "memfd_secret",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/memfd_secret.2.en.html",
        entry_point: "sys_memfd_secret",
        args: &[SyscallArg {
            reg: "%rdi",
            arg_type: "unsigned int flags",
        }],
    },
    SyscallInfo {
        nr: 448,
        name: "process_mrelease",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/process_mrelease.2.en.html",
        entry_point: "sys_process_mrelease",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int pidfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 449,
        name: "futex_waitv",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/futex_waitv.2.en.html",
        entry_point: "sys_futex_waitv",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "struct futex_waitv *waiters",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int nr_futexes",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int flags",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "struct __kernel_timespec *timeout",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "clockid_t clockid",
            },
        ],
    },
    SyscallInfo {
        nr: 450,
        name: "set_mempolicy_home_node",
        man_url:
            "https://manpages.debian.org/unstable/manpages-dev/set_mempolicy_home_node.2.en.html",
        entry_point: "sys_set_mempolicy_home_node",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long start",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long home_node",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned long flags",
            },
        ],
    },
    SyscallInfo {
        nr: 451,
        name: "cachestat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/cachestat.2.en.html",
        entry_point: "sys_cachestat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int fd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct cachestat_range *cstat_range",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "struct cachestat *cstat",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 452,
        name: "fchmodat2",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/fchmodat2.2.en.html",
        entry_point: "sys_fchmodat2",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *filename",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "umode_t mode",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 453,
        name: "map_shadow_stack",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/map_shadow_stack.2.en.html",
        entry_point: "sys_map_shadow_stack",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long addr",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long size",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 454,
        name: "futex_wake",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/futex_wake.2.en.html",
        entry_point: "sys_futex_wake",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "void *uaddr",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long mask",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int nr",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 455,
        name: "futex_wait",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/futex_wait.2.en.html",
        entry_point: "sys_futex_wait",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "void *uaddr",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned long val",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long mask",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int flags",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "struct __kernel_timespec *timespec",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "clockid_t clockid",
            },
        ],
    },
    SyscallInfo {
        nr: 456,
        name: "futex_requeue",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/futex_requeue.2.en.html",
        entry_point: "sys_futex_requeue",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "struct futex_waitv *waiters",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "unsigned int flags",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "int nr_wake",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "int nr_requeue",
            },
        ],
    },
    SyscallInfo {
        nr: 457,
        name: "statmount",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/statmount.2.en.html",
        entry_point: "sys_statmount",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const struct mnt_id_req *req",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct statmount *buf",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t bufsize",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 458,
        name: "listmount",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/listmount.2.en.html",
        entry_point: "sys_listmount",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "const struct mnt_id_req *req",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "u64 *mnt_ids",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "size_t nr_mnt_ids",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "unsigned int flags",
            },
        ],
    },
    SyscallInfo {
        nr: 459,
        name: "lsm_get_self_attr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/lsm_get_self_attr.2.en.html",
        entry_point: "sys_lsm_get_self_attr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int attr",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct lsm_ctx *ctx",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "u32 *size",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "u32 flags",
            },
        ],
    },
    SyscallInfo {
        nr: 460,
        name: "lsm_set_self_attr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/lsm_set_self_attr.2.en.html",
        entry_point: "sys_lsm_set_self_attr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned int attr",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "struct lsm_ctx *ctx",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "u32 size",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "u32 flags",
            },
        ],
    },
    SyscallInfo {
        nr: 461,
        name: "lsm_list_modules",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/lsm_list_modules.2.en.html",
        entry_point: "sys_lsm_list_modules",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "u64 *ids",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "u32 *size",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "u32 flags",
            },
        ],
    },
    SyscallInfo {
        nr: 462,
        name: "mseal",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/mseal.2.en.html",
        entry_point: "sys_mseal",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "unsigned long start",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "size_t len",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned long flags",
            },
        ],
    },
    SyscallInfo {
        nr: 463,
        name: "setxattrat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/setxattrat.2.en.html",
        entry_point: "sys_setxattrat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int at_flags",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const char *name",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "const struct xattr_args *args",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "size_t size",
            },
        ],
    },
    SyscallInfo {
        nr: 464,
        name: "getxattrat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/getxattrat.2.en.html",
        entry_point: "sys_getxattrat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int at_flags",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const char *name",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "struct xattr_args *args",
            },
            SyscallArg {
                reg: "%r9",
                arg_type: "size_t size",
            },
        ],
    },
    SyscallInfo {
        nr: 465,
        name: "listxattrat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/listxattrat.2.en.html",
        entry_point: "sys_listxattrat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int at_flags",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "char *list",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "size_t size",
            },
        ],
    },
    SyscallInfo {
        nr: 466,
        name: "removexattrat",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/removexattrat.2.en.html",
        entry_point: "sys_removexattrat",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned int at_flags",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "const char *name",
            },
        ],
    },
    SyscallInfo {
        nr: 467,
        name: "open_tree_attr",
        man_url: "https://manpages.debian.org/unstable/manpages-dev/open_tree_attr.2.en.html",
        entry_point: "sys_open_tree_attr",
        args: &[
            SyscallArg {
                reg: "%rdi",
                arg_type: "int dfd",
            },
            SyscallArg {
                reg: "%rsi",
                arg_type: "const char *path",
            },
            SyscallArg {
                reg: "%rdx",
                arg_type: "unsigned flags",
            },
            SyscallArg {
                reg: "%r10",
                arg_type: "struct mount_attr *uattr",
            },
            SyscallArg {
                reg: "%r8",
                arg_type: "size_t usize",
            },
        ],
    },
];
