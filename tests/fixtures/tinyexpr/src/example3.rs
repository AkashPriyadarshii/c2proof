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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const TE_FLAG_PURE: C2RustUnnamed_0 = 32;
pub const TE_CLOSURE7: C2RustUnnamed_0 = 23;
pub const TE_CLOSURE6: C2RustUnnamed_0 = 22;
pub const TE_CLOSURE5: C2RustUnnamed_0 = 21;
pub const TE_CLOSURE4: C2RustUnnamed_0 = 20;
pub const TE_CLOSURE3: C2RustUnnamed_0 = 19;
pub const TE_CLOSURE2: C2RustUnnamed_0 = 18;
pub const TE_CLOSURE1: C2RustUnnamed_0 = 17;
pub const TE_CLOSURE0: C2RustUnnamed_0 = 16;
pub const TE_FUNCTION7: C2RustUnnamed_0 = 15;
pub const TE_FUNCTION6: C2RustUnnamed_0 = 14;
pub const TE_FUNCTION5: C2RustUnnamed_0 = 13;
pub const TE_FUNCTION4: C2RustUnnamed_0 = 12;
pub const TE_FUNCTION3: C2RustUnnamed_0 = 11;
pub const TE_FUNCTION2: C2RustUnnamed_0 = 10;
pub const TE_FUNCTION1: C2RustUnnamed_0 = 9;
pub const TE_FUNCTION0: C2RustUnnamed_0 = 8;
pub const TE_VARIABLE: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct te_variable {
    pub name: *const libc::c_char,
    pub address: *const libc::c_void,
    pub type_0: libc::c_int,
    pub context: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn my_sum(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    printf(
        b"Called C function with %f and %f.\n\0" as *const u8 as *const libc::c_char,
        a,
        b,
    );
    return a + b;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut vars: [te_variable; 1] = [
        {
            let mut init = te_variable {
                name: b"mysum\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        my_sum
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ),
                type_0: TE_FUNCTION2 as libc::c_int,
                context: 0 as *mut libc::c_void,
            };
            init
        },
    ];
    let mut expression: *const libc::c_char = b"mysum(5, 6)\0" as *const u8
        as *const libc::c_char;
    printf(b"Evaluating:\n\t%s\n\0" as *const u8 as *const libc::c_char, expression);
    let mut err: libc::c_int = 0;
    let mut n: *mut te_expr = te_compile(
        expression,
        vars.as_mut_ptr(),
        1 as libc::c_int,
        &mut err,
    );
    if !n.is_null() {
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
