//! inlined winapi definitions
//! winapi-rs 46aa861d606b21ded202a606b1351214ea899fa3^
#![allow(bad_style, overflowing_literals, unused_macros, unused_imports, dead_code)]

// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
#[macro_use]
pub mod macros {
    macro_rules! STRUCT {
		(#[debug] $($rest:tt)*) => (
			STRUCT!{#[cfg_attr(feature = "impl-debug", derive(Debug))] $($rest)*}
		);
		($(#[$attrs:meta])* struct $name:ident {
			$($field:ident: $ftype:ty,)+
		}) => (
			#[repr(C)] #[derive(Copy)] $(#[$attrs])*
			pub struct $name {
				$(pub $field: $ftype,)+
			}
			impl Clone for $name {
				#[inline]
				fn clone(&self) -> $name { *self }
			}
			#[cfg(feature = "impl-default")]
			impl Default for $name {
				#[inline]
				fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
			}
		);
	}
}

pub mod shared {
    pub mod minwindef {
        use self::winapi::ctypes::{
            c_char, c_float, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void,
        };
        use crate::winapi_inline as winapi;

        pub type DWORD = c_ulong;
        pub type BOOL = c_int;
        pub type WORD = c_ushort;
        pub type LPDWORD = *mut DWORD;

        STRUCT! {#[debug] struct FILETIME {
            dwLowDateTime: DWORD,
            dwHighDateTime: DWORD,
        }}
    }
    pub mod winerror {
        use self::winapi::shared::minwindef::DWORD;
        use crate::winapi_inline as winapi;

        pub const NO_ERROR: DWORD = 0;
    }
}

pub mod um {
    pub mod wincon {
        use self::winapi::shared::minwindef::{BOOL, DWORD, WORD};
        pub use self::winapi::um::wincontypes::{COORD, PCOORD, PSMALL_RECT, SMALL_RECT};
        use self::winapi::um::winnt::HANDLE;
        use crate::winapi_inline as winapi;

        pub const FOREGROUND_BLUE: WORD = 0x0001;
        pub const FOREGROUND_GREEN: WORD = 0x0002;
        pub const FOREGROUND_RED: WORD = 0x0004;
        pub const FOREGROUND_INTENSITY: WORD = 0x0008;
        pub const BACKGROUND_BLUE: WORD = 0x0010;
        pub const BACKGROUND_GREEN: WORD = 0x0020;
        pub const BACKGROUND_RED: WORD = 0x0040;
        pub const BACKGROUND_INTENSITY: WORD = 0x0080;

        STRUCT! {struct CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: COORD,
            dwCursorPosition: COORD,
            wAttributes: WORD,
            srWindow: SMALL_RECT,
            dwMaximumWindowSize: COORD,
        }}
        pub type PCONSOLE_SCREEN_BUFFER_INFO = *mut CONSOLE_SCREEN_BUFFER_INFO;

        pub const ENABLE_PROCESSED_INPUT: DWORD = 0x0001;
        pub const ENABLE_LINE_INPUT: DWORD = 0x0002;
        pub const ENABLE_ECHO_INPUT: DWORD = 0x0004;
        pub const ENABLE_WINDOW_INPUT: DWORD = 0x0008;
        pub const ENABLE_MOUSE_INPUT: DWORD = 0x0010;
        pub const ENABLE_INSERT_MODE: DWORD = 0x0020;
        pub const ENABLE_QUICK_EDIT_MODE: DWORD = 0x0040;
        pub const ENABLE_EXTENDED_FLAGS: DWORD = 0x0080;
        pub const ENABLE_AUTO_POSITION: DWORD = 0x0100;
        pub const ENABLE_VIRTUAL_TERMINAL_INPUT: DWORD = 0x0200;
        pub const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001;
        pub const ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002;
        pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;
        pub const DISABLE_NEWLINE_AUTO_RETURN: DWORD = 0x0008;
        pub const ENABLE_LVB_GRID_WORLDWIDE: DWORD = 0x0010;
        extern "system" {
            pub fn GetConsoleScreenBufferInfo(
                hConsoleOutput: HANDLE,
                lpConsoleScreenBufferInfo: PCONSOLE_SCREEN_BUFFER_INFO,
            ) -> BOOL;

            pub fn SetConsoleTextAttribute(hConsoleOutput: HANDLE, wAttributes: WORD) -> BOOL;
        }
    }
    pub mod wincontypes {
        use self::winapi::um::winnt::SHORT;
        use crate::winapi_inline as winapi;
        STRUCT! {struct COORD {
            X: SHORT,
            Y: SHORT,
        }}
        pub type PCOORD = *mut COORD;
        STRUCT! {struct SMALL_RECT {
            Left: SHORT,
            Top: SHORT,
            Right: SHORT,
            Bottom: SHORT,
        }}
        pub type PSMALL_RECT = *mut SMALL_RECT;
    }

    pub mod consoleapi {
        use self::winapi::shared::minwindef::{BOOL, DWORD, LPDWORD};
        use self::winapi::um::winnt::HANDLE;
        use crate::winapi_inline as winapi;
        extern "system" {
            pub fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;

            pub fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
        }
    }

    pub mod errhandlingapi {
        use self::winapi::shared::minwindef::DWORD;
        use crate::winapi_inline as winapi;

        extern "system" {
            pub fn GetLastError() -> DWORD;
        }
    }

    pub mod fileapi {
        use self::winapi::shared::minwindef::{BOOL, DWORD, FILETIME};
        use self::winapi::um::winnt::HANDLE;
        use crate::winapi_inline as winapi;

        STRUCT! {struct BY_HANDLE_FILE_INFORMATION {
            dwFileAttributes: DWORD,
            ftCreationTime: FILETIME,
            ftLastAccessTime: FILETIME,
            ftLastWriteTime: FILETIME,
            dwVolumeSerialNumber: DWORD,
            nFileSizeHigh: DWORD,
            nFileSizeLow: DWORD,
            nNumberOfLinks: DWORD,
            nFileIndexHigh: DWORD,
            nFileIndexLow: DWORD,
        }}
        pub type PBY_HANDLE_FILE_INFORMATION = *mut BY_HANDLE_FILE_INFORMATION;
        pub type LPBY_HANDLE_FILE_INFORMATION = *mut BY_HANDLE_FILE_INFORMATION;
        extern "system" {
            pub fn GetFileInformationByHandle(
                hFile: HANDLE,
                lpFileInformation: LPBY_HANDLE_FILE_INFORMATION,
            ) -> BOOL;

            pub fn GetFileType(hFile: HANDLE) -> DWORD;
        }
    }

    pub mod winnt {
        use self::winapi::ctypes::{
            __int64, __uint64, c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void, wchar_t,
        };
        use self::winapi::shared::minwindef::DWORD;
        use crate::winapi_inline as winapi;

        pub type SHORT = c_short;
        pub type HANDLE = *mut c_void;

        pub const FILE_ATTRIBUTE_READONLY: DWORD = 0x00000001;
        pub const FILE_ATTRIBUTE_HIDDEN: DWORD = 0x00000002;
        pub const FILE_ATTRIBUTE_SYSTEM: DWORD = 0x00000004;
        pub const FILE_ATTRIBUTE_DIRECTORY: DWORD = 0x00000010;
        pub const FILE_ATTRIBUTE_ARCHIVE: DWORD = 0x00000020;
        pub const FILE_ATTRIBUTE_DEVICE: DWORD = 0x00000040;
        pub const FILE_ATTRIBUTE_NORMAL: DWORD = 0x00000080;
        pub const FILE_ATTRIBUTE_TEMPORARY: DWORD = 0x00000100;
        pub const FILE_ATTRIBUTE_SPARSE_FILE: DWORD = 0x00000200;
        pub const FILE_ATTRIBUTE_REPARSE_POINT: DWORD = 0x00000400;
        pub const FILE_ATTRIBUTE_COMPRESSED: DWORD = 0x00000800;
        pub const FILE_ATTRIBUTE_OFFLINE: DWORD = 0x00001000;
        pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: DWORD = 0x00002000;
        pub const FILE_ATTRIBUTE_ENCRYPTED: DWORD = 0x00004000;
        pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: DWORD = 0x00008000;
        pub const FILE_ATTRIBUTE_VIRTUAL: DWORD = 0x00010000;
        pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: DWORD = 0x00020000;
        pub const FILE_ATTRIBUTE_EA: DWORD = 0x00040000;
        pub const FILE_ATTRIBUTE_PINNED: DWORD = 0x00080000;
        pub const FILE_ATTRIBUTE_UNPINNED: DWORD = 0x00100000;
        pub const FILE_ATTRIBUTE_RECALL_ON_OPEN: DWORD = 0x00040000;
        pub const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: DWORD = 0x00400000;
    }

    pub mod winbase {
        use self::winapi::shared::minwindef::DWORD;
        use crate::winapi_inline as winapi;

        pub const FILE_FLAG_WRITE_THROUGH: DWORD = 0x80000000;
        pub const FILE_FLAG_OVERLAPPED: DWORD = 0x40000000;
        pub const FILE_FLAG_NO_BUFFERING: DWORD = 0x20000000;
        pub const FILE_FLAG_RANDOM_ACCESS: DWORD = 0x10000000;
        pub const FILE_FLAG_SEQUENTIAL_SCAN: DWORD = 0x08000000;
        pub const FILE_FLAG_DELETE_ON_CLOSE: DWORD = 0x04000000;
        pub const FILE_FLAG_BACKUP_SEMANTICS: DWORD = 0x02000000;
        pub const FILE_FLAG_POSIX_SEMANTICS: DWORD = 0x01000000;
        pub const FILE_FLAG_SESSION_AWARE: DWORD = 0x00800000;
        pub const FILE_FLAG_OPEN_REPARSE_POINT: DWORD = 0x00200000;
        pub const FILE_FLAG_OPEN_NO_RECALL: DWORD = 0x00100000;
        pub const FILE_FLAG_FIRST_PIPE_INSTANCE: DWORD = 0x00080000;
        pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: DWORD = 0x00040000;

        pub const FILE_TYPE_UNKNOWN: DWORD = 0x0000;
        pub const FILE_TYPE_DISK: DWORD = 0x0001;
        pub const FILE_TYPE_CHAR: DWORD = 0x0002;
        pub const FILE_TYPE_PIPE: DWORD = 0x0003;
        pub const FILE_TYPE_REMOTE: DWORD = 0x8000;
    }
}

/// Built in primitive types provided by the C language
pub mod ctypes {
    pub use std::os::raw::c_void;
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub type c_float = f32;
    pub type c_double = f64;
    pub type __int8 = i8;
    pub type __uint8 = u8;
    pub type __int16 = i16;
    pub type __uint16 = u16;
    pub type __int32 = i32;
    pub type __uint32 = u32;
    pub type __int64 = i64;
    pub type __uint64 = u64;
    pub type wchar_t = u16;
}
