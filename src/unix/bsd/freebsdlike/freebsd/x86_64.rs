pub type c_long = i64;
pub type c_ulong = u64;
pub type time_t = i64;
pub type suseconds_t = i64;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        //        #[cfg(not(target_os_version = "12"))] XXX
        //pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        //        #[cfg(target_os_version = "12")] XXX
        pub st_mode: ::mode_t,
        //        #[cfg(target_os_version = "12")] XXX
        pub st_pad0: ::uint16_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        //        #[cfg(target_os_version = "12")] XXX
        pub st_pad1: ::uint32_t,
        pub st_rdev: ::dev_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        //        #[cfg(target_os_version = "12")] XXX
        pub st_birthtime: ::time_t,
        //        #[cfg(target_os_version = "12")] XXX
        pub st_birthtime_nsec: ::c_long,
        pub st_size: ::off_t,
        pub st_blocks: ::blkcnt_t,
        pub st_blksize: ::blksize_t,
        pub st_flags: ::fflags_t,
        //        #[cfg(target_os_version = "12")] XXX
        pub st_gen: ::uint64_t,
        //        #[cfg(not(target_os_version = "12"))] XXX
        //pub st_gen: ::uint32_t,
        //        #[cfg(not(target_os_version = "12"))] XXX
        //pub st_lspare: ::int32_t,
        //        #[cfg(not(target_os_version = "12"))] XXX
        //pub st_birthtime: ::time_t,
        //        #[cfg(not(target_os_version = "12"))] XXX
        //pub st_birthtime_nsec: ::c_long,
        //        #[cfg(target_os_version = "12")] XXX
        pub st_spare: [::uint64_t; 10],
    }
}

pub const MAP_32BIT: ::c_int = 0x00080000;
