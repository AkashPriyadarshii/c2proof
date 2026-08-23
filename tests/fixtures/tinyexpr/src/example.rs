use ::libc;
extern "C" {
    fn te_interp(
        expression: *const libc::c_char,
        error: *mut libc::c_int,
    ) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: *const libc::c_char = b"sqrt(5^2+7^2+11^2+(8-2)^2)\0" as *const u8
        as *const libc::c_char;
    let mut r: libc::c_double = te_interp(c, 0 as *mut libc::c_int);
    printf(
        b"The expression:\n\t%s\nevaluates to:\n\t%f\n\0" as *const u8
            as *const libc::c_char,
        c,
        r,
    );
    return 0 as libc::c_int;
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
