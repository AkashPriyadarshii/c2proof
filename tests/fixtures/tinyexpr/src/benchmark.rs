use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn clock() -> clock_t;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn te_compile(
        expression: *const libc::c_char,
        variables: *const te_variable,
        var_count: libc::c_int,
        error: *mut libc::c_int,
    ) -> *mut te_expr;
    fn te_eval(n: *const te_expr) -> libc::c_double;
    fn te_free(n: *mut te_expr);
}
pub type __clock_t = libc::c_long;
pub type clock_t = __clock_t;
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
pub type function1 = Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>;
#[no_mangle]
pub unsafe extern "C" fn bench(mut expr: *const libc::c_char, mut func: function1) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut d: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut start: clock_t = 0;
    let mut lk: te_variable = {
        let mut init = te_variable {
            name: b"a\0" as *const u8 as *const libc::c_char,
            address: &mut tmp as *mut libc::c_double as *const libc::c_void,
            type_0: 0,
            context: 0 as *mut libc::c_void,
        };
        init
    };
    printf(b"Expression: %s\n\0" as *const u8 as *const libc::c_char, expr);
    printf(b"native \0" as *const u8 as *const libc::c_char);
    start = clock();
    ::core::ptr::write_volatile(
        &mut d as *mut libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    j = 0 as libc::c_int;
    while j < 10000 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 10000 as libc::c_int {
            tmp = i as libc::c_double;
            ::core::ptr::write_volatile(
                &mut d as *mut libc::c_double,
                ::core::ptr::read_volatile::<libc::c_double>(&d as *const libc::c_double)
                    + func.expect("non-null function pointer")(tmp),
            );
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    let nelapsed: libc::c_int = ((clock() - start) * 1000 as libc::c_int as libc::c_long
        / 1000000 as libc::c_int as __clock_t) as libc::c_int;
    printf(b" %.5g\0" as *const u8 as *const libc::c_char, d);
    if nelapsed != 0 {
        printf(
            b"\t%5dms\t%5dmfps\n\0" as *const u8 as *const libc::c_char,
            nelapsed,
            10000 as libc::c_int * 10000 as libc::c_int / nelapsed / 1000 as libc::c_int,
        );
    } else {
        printf(b"\tinf\n\0" as *const u8 as *const libc::c_char);
    }
    printf(b"interp \0" as *const u8 as *const libc::c_char);
    let mut n: *mut te_expr = te_compile(
        expr,
        &mut lk,
        1 as libc::c_int,
        0 as *mut libc::c_int,
    );
    start = clock();
    ::core::ptr::write_volatile(
        &mut d as *mut libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    j = 0 as libc::c_int;
    while j < 10000 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 10000 as libc::c_int {
            tmp = i as libc::c_double;
            ::core::ptr::write_volatile(
                &mut d as *mut libc::c_double,
                ::core::ptr::read_volatile::<libc::c_double>(&d as *const libc::c_double)
                    + te_eval(n),
            );
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    let eelapsed: libc::c_int = ((clock() - start) * 1000 as libc::c_int as libc::c_long
        / 1000000 as libc::c_int as __clock_t) as libc::c_int;
    te_free(n);
    printf(b" %.5g\0" as *const u8 as *const libc::c_char, d);
    if eelapsed != 0 {
        printf(
            b"\t%5dms\t%5dmfps\n\0" as *const u8 as *const libc::c_char,
            eelapsed,
            10000 as libc::c_int * 10000 as libc::c_int / eelapsed / 1000 as libc::c_int,
        );
    } else {
        printf(b"\tinf\n\0" as *const u8 as *const libc::c_char);
    }
    printf(
        b"%.2f%% longer\n\0" as *const u8 as *const libc::c_char,
        (eelapsed as libc::c_double / nelapsed as libc::c_double - 1.0f64) * 100.0f64,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn a5(mut a: libc::c_double) -> libc::c_double {
    return a + 5 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn a55(mut a: libc::c_double) -> libc::c_double {
    return 5 as libc::c_int as libc::c_double + a + 5 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn a5abs(mut a: libc::c_double) -> libc::c_double {
    return fabs(a + 5 as libc::c_int as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn a52(mut a: libc::c_double) -> libc::c_double {
    return (a + 5 as libc::c_int as libc::c_double) * 2 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn a10(mut a: libc::c_double) -> libc::c_double {
    return a + (5 as libc::c_int * 2 as libc::c_int) as libc::c_double;
}
#[export_name = "as"]
pub unsafe extern "C" fn as_0(mut a: libc::c_double) -> libc::c_double {
    return sqrt(pow(a, 1.5f64) + pow(a, 2.5f64));
}
#[no_mangle]
pub unsafe extern "C" fn al(mut a: libc::c_double) -> libc::c_double {
    return 1 as libc::c_int as libc::c_double / (a + 1 as libc::c_int as libc::c_double)
        + 2 as libc::c_int as libc::c_double / (a + 2 as libc::c_int as libc::c_double)
        + 3 as libc::c_int as libc::c_double / (a + 3 as libc::c_int as libc::c_double);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    bench(
        b"a+5\0" as *const u8 as *const libc::c_char,
        Some(a5 as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    );
    bench(
        b"5+a+5\0" as *const u8 as *const libc::c_char,
        Some(a55 as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    );
    bench(
        b"abs(a+5)\0" as *const u8 as *const libc::c_char,
        Some(a5abs as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    );
    bench(
        b"sqrt(a^1.5+a^2.5)\0" as *const u8 as *const libc::c_char,
        Some(as_0 as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    );
    bench(
        b"a+(5*2)\0" as *const u8 as *const libc::c_char,
        Some(a10 as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    );
    bench(
        b"(a+5)*2\0" as *const u8 as *const libc::c_char,
        Some(a52 as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    );
    bench(
        b"(1/(a+1)+2/(a+2)+3/(a+3))\0" as *const u8 as *const libc::c_char,
        Some(al as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
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
