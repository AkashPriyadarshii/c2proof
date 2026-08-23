use ::libc;
extern "C" {
    fn te_compile(
        expression: *const libc::c_char,
        variables: *const te_variable,
        var_count: libc::c_int,
        error: *mut libc::c_int,
    ) -> *mut te_expr;
    fn te_eval(n: *const te_expr) -> libc::c_double;
    fn te_free(n: *mut te_expr);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct te_expr {
    pub type_0: libc::c_int,
    pub c2rust_unnamed: C2RustUnnamed,
    pub parameters: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub value: libc::c_double,
    pub bound: *const libc::c_double,
    pub function: *const libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct te_variable {
    pub name: *const libc::c_char,
    pub address: *const libc::c_void,
    pub type_0: libc::c_int,
    pub context: *mut libc::c_void,
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc < 2 as libc::c_int {
        printf(
            b"Usage: example2 \"expression\"\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    let mut expression: *const libc::c_char = *argv.offset(1 as libc::c_int as isize);
    printf(b"Evaluating:\n\t%s\n\0" as *const u8 as *const libc::c_char, expression);
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut vars: [te_variable; 2] = [
        {
            let mut init = te_variable {
                name: b"x\0" as *const u8 as *const libc::c_char,
                address: &mut x as *mut libc::c_double as *const libc::c_void,
                type_0: 0,
                context: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"y\0" as *const u8 as *const libc::c_char,
                address: &mut y as *mut libc::c_double as *const libc::c_void,
                type_0: 0,
                context: 0 as *mut libc::c_void,
            };
            init
        },
    ];
    let mut err: libc::c_int = 0;
    let mut n: *mut te_expr = te_compile(
        expression,
        vars.as_mut_ptr(),
        2 as libc::c_int,
        &mut err,
    );
    if !n.is_null() {
        x = 3 as libc::c_int as libc::c_double;
        y = 4 as libc::c_int as libc::c_double;
        let r: libc::c_double = te_eval(n);
        printf(b"Result:\n\t%f\n\0" as *const u8 as *const libc::c_char, r);
        te_free(n);
    } else {
        printf(
            b"\t%*s^\nError near here\0" as *const u8 as *const libc::c_char,
            err - 1 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
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
