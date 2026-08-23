use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn te_interp(
        expression: *const libc::c_char,
        error: *mut libc::c_int,
    ) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
unsafe extern "C" fn readline(mut prompt: *const libc::c_char) -> *mut libc::c_char {
    fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, prompt);
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut line: *mut libc::c_char = fgets(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        stdin,
    );
    if line.is_null() && feof(stdin) != 0 {
        return 0 as *mut libc::c_char
    } else if line.is_null() {
        perror(b"fgets\0" as *const u8 as *const libc::c_char);
        return 0 as *mut libc::c_char;
    }
    let mut len: size_t = strlen(line);
    if len < 1 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_char;
    }
    if *line.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '\n' as i32
    {
        *line
            .offset(
                len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        len = (len as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    line = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if line.is_null() {
        return 0 as *mut libc::c_char;
    }
    strcpy(line, buf.as_mut_ptr());
    return line;
}
unsafe extern "C" fn add_history(mut line: *const libc::c_char) {}
unsafe extern "C" fn eval(mut str: *const libc::c_char) -> libc::c_int {
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_double = te_interp(str, &mut err);
    if err != 0 as libc::c_int {
        printf(b"Error at position %i\n\0" as *const u8 as *const libc::c_char, err);
        return -(1 as libc::c_int);
    } else {
        printf(b"%g\n\0" as *const u8 as *const libc::c_char, r);
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn repl() {
    loop {
        let mut line: *mut libc::c_char = readline(
            b"> \0" as *const u8 as *const libc::c_char,
        );
        if line.is_null() {
            break;
        }
        if strcmp(line, b"q\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(line, b"quit\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            free(line as *mut libc::c_void);
            break;
        } else {
            if eval(line) != -(1 as libc::c_int) {
                add_history(line);
            }
            free(line as *mut libc::c_void);
        }
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc == 3 as libc::c_int
        && strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"-e\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        if eval(*argv.offset(2 as libc::c_int as isize)) == -(1 as libc::c_int) {
            return 1 as libc::c_int
        } else {
            return 0 as libc::c_int
        }
    } else if argc == 1 as libc::c_int {
        repl();
        return 0 as libc::c_int;
    } else {
        printf(
            b"Usage: %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        printf(
            b"       %s -e <expression>\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
