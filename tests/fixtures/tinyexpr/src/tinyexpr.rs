use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn cosh(_: libc::c_double) -> libc::c_double;
    fn sinh(_: libc::c_double) -> libc::c_double;
    fn tanh(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub start: *const libc::c_char,
    pub next: *const libc::c_char,
    pub type_0: libc::c_int,
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub context: *mut libc::c_void,
    pub lookup: *const te_variable,
    pub lookup_len: libc::c_int,
    pub applied_unary: libc::c_int,
    pub depth: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub value: libc::c_double,
    pub bound: *const libc::c_double,
    pub function: *const libc::c_void,
}
pub const TOK_ERROR: C2RustUnnamed_3 = 25;
pub const TOK_NULL: C2RustUnnamed_3 = 24;
pub const TOK_SEP: C2RustUnnamed_3 = 27;
pub const TOK_CLOSE: C2RustUnnamed_3 = 29;
pub const TOK_OPEN: C2RustUnnamed_3 = 28;
pub const TOK_INFIX: C2RustUnnamed_3 = 32;
pub const TOK_VARIABLE: C2RustUnnamed_3 = 31;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const TOK_NUMBER: C2RustUnnamed_3 = 30;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const TOK_END: C2RustUnnamed_3 = 26;
pub const TE_CONSTANT: C2RustUnnamed_4 = 1;
pub type te_fun2 = Option::<
    unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double,
>;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
pub type C2RustUnnamed_3 = libc::c_uint;
pub type C2RustUnnamed_4 = libc::c_uint;
unsafe extern "C" fn new_expr(
    type_0: libc::c_int,
    mut parameters: *mut *const te_expr,
) -> *mut te_expr {
    let arity: libc::c_int = if type_0
        & (TE_FUNCTION0 as libc::c_int | TE_CLOSURE0 as libc::c_int) != 0
    {
        type_0 & 0x7 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let psize: libc::c_int = (::core::mem::size_of::<*mut libc::c_void>()
        as libc::c_ulong)
        .wrapping_mul(arity as libc::c_ulong) as libc::c_int;
    let size: libc::c_int = (::core::mem::size_of::<te_expr>() as libc::c_ulong)
        .wrapping_add(psize as libc::c_ulong)
        .wrapping_add(
            (if type_0 & TE_CLOSURE0 as libc::c_int != 0 as libc::c_int {
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        ) as libc::c_int;
    let mut ret: *mut te_expr = malloc(size as libc::c_ulong) as *mut te_expr;
    if ret.is_null() {
        return 0 as *mut te_expr;
    }
    memset(ret as *mut libc::c_void, 0 as libc::c_int, size as libc::c_ulong);
    if arity != 0 && !parameters.is_null() {
        memcpy(
            ((*ret).parameters).as_mut_ptr() as *mut libc::c_void,
            parameters as *const libc::c_void,
            psize as libc::c_ulong,
        );
    }
    (*ret).type_0 = type_0;
    (*ret).c2rust_unnamed.bound = 0 as *const libc::c_double;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn te_free_parameters(mut n: *mut te_expr) {
    if n.is_null() {
        return;
    }
    let mut current_block_8: u64;
    match (*n).type_0 & 0x1f as libc::c_int {
        15 | 23 => {
            te_free(
                *((*n).parameters).as_mut_ptr().offset(6 as libc::c_int as isize)
                    as *mut te_expr,
            );
            current_block_8 = 14107434098666203977;
        }
        14 | 22 => {
            current_block_8 = 14107434098666203977;
        }
        13 | 21 => {
            current_block_8 = 9979581075683099426;
        }
        12 | 20 => {
            current_block_8 = 2147467507124728288;
        }
        11 | 19 => {
            current_block_8 = 14895785621082176942;
        }
        10 | 18 => {
            current_block_8 = 17788683342750976530;
        }
        9 | 17 => {
            current_block_8 = 10401831275666622796;
        }
        _ => {
            current_block_8 = 5399440093318478209;
        }
    }
    match current_block_8 {
        14107434098666203977 => {
            te_free(
                *((*n).parameters).as_mut_ptr().offset(5 as libc::c_int as isize)
                    as *mut te_expr,
            );
            current_block_8 = 9979581075683099426;
        }
        _ => {}
    }
    match current_block_8 {
        9979581075683099426 => {
            te_free(
                *((*n).parameters).as_mut_ptr().offset(4 as libc::c_int as isize)
                    as *mut te_expr,
            );
            current_block_8 = 2147467507124728288;
        }
        _ => {}
    }
    match current_block_8 {
        2147467507124728288 => {
            te_free(
                *((*n).parameters).as_mut_ptr().offset(3 as libc::c_int as isize)
                    as *mut te_expr,
            );
            current_block_8 = 14895785621082176942;
        }
        _ => {}
    }
    match current_block_8 {
        14895785621082176942 => {
            te_free(
                *((*n).parameters).as_mut_ptr().offset(2 as libc::c_int as isize)
                    as *mut te_expr,
            );
            current_block_8 = 17788683342750976530;
        }
        _ => {}
    }
    match current_block_8 {
        17788683342750976530 => {
            te_free(
                *((*n).parameters).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut te_expr,
            );
            current_block_8 = 10401831275666622796;
        }
        _ => {}
    }
    match current_block_8 {
        10401831275666622796 => {
            te_free(
                *((*n).parameters).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut te_expr,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn te_free(mut n: *mut te_expr) {
    if n.is_null() {
        return;
    }
    te_free_parameters(n);
    free(n as *mut libc::c_void);
}
unsafe extern "C" fn pi() -> libc::c_double {
    return 3.14159265358979323846f64;
}
unsafe extern "C" fn e() -> libc::c_double {
    return 2.71828182845904523536f64;
}
unsafe extern "C" fn fac(mut a: libc::c_double) -> libc::c_double {
    if !(a >= 0.0f64) {
        return ::core::f32::NAN as libc::c_double;
    }
    if a
        > (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_double
    {
        return ::core::f32::INFINITY as libc::c_double;
    }
    let mut ua: libc::c_uint = a as libc::c_uint;
    let mut result: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_ulong = 0;
    i = 1 as libc::c_int as libc::c_ulong;
    while i <= ua as libc::c_ulong {
        if i
            > (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong)
                .wrapping_div(result)
        {
            return ::core::f32::INFINITY as libc::c_double;
        }
        result = result.wrapping_mul(i);
        i = i.wrapping_add(1);
        i;
    }
    return result as libc::c_double;
}
unsafe extern "C" fn ncr(
    mut n: libc::c_double,
    mut r: libc::c_double,
) -> libc::c_double {
    if !(n >= 0.0f64) || !(r >= 0.0f64) || n < r {
        return ::core::f32::NAN as libc::c_double;
    }
    if n
        > (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_double
        || r
            > (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as libc::c_double
    {
        return ::core::f32::INFINITY as libc::c_double;
    }
    let mut un: libc::c_ulong = n as libc::c_uint as libc::c_ulong;
    let mut ur: libc::c_ulong = r as libc::c_uint as libc::c_ulong;
    let mut i: libc::c_ulong = 0;
    let mut result: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
    if ur > un.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        ur = un.wrapping_sub(ur);
    }
    i = 1 as libc::c_int as libc::c_ulong;
    while i <= ur {
        if result
            > (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong)
                .wrapping_div(un.wrapping_sub(ur).wrapping_add(i))
        {
            return ::core::f32::INFINITY as libc::c_double;
        }
        result = result.wrapping_mul(un.wrapping_sub(ur).wrapping_add(i));
        result = result.wrapping_div(i);
        i = i.wrapping_add(1);
        i;
    }
    return result as libc::c_double;
}
unsafe extern "C" fn npr(
    mut n: libc::c_double,
    mut r: libc::c_double,
) -> libc::c_double {
    return ncr(n, r) * fac(r);
}
static mut functions: [te_variable; 25] = unsafe {
    [
        {
            let mut init = te_variable {
                name: b"abs\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(fabs as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"acos\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(acos as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"asin\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(asin as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"atan\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(atan as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"atan2\0" as *const u8 as *const libc::c_char,
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
                        atan2
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ),
                type_0: TE_FUNCTION2 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"ceil\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(ceil as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"cos\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(cos as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"cosh\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(cosh as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"e\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> libc::c_double>,
                    *const libc::c_void,
                >(Some(e as unsafe extern "C" fn() -> libc::c_double)),
                type_0: TE_FUNCTION0 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"exp\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(exp as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"fac\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(fac as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"floor\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(floor as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"ln\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(log as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"log\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(log10 as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"log10\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(log10 as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"ncr\0" as *const u8 as *const libc::c_char,
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
                        ncr
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ),
                type_0: TE_FUNCTION2 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"npr\0" as *const u8 as *const libc::c_char,
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
                        npr
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ),
                type_0: TE_FUNCTION2 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"pi\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> libc::c_double>,
                    *const libc::c_void,
                >(Some(pi as unsafe extern "C" fn() -> libc::c_double)),
                type_0: TE_FUNCTION0 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"pow\0" as *const u8 as *const libc::c_char,
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
                        pow
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ),
                type_0: TE_FUNCTION2 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"sin\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(sin as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"sinh\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(sinh as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"sqrt\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(sqrt as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"tan\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(tan as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"tanh\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(tanh as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: 0 as *const libc::c_char,
                address: 0 as *const libc::c_void,
                type_0: 0 as libc::c_int,
                context: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
unsafe extern "C" fn find_builtin(
    mut name: *const libc::c_char,
    mut len: libc::c_int,
) -> *const te_variable {
    let mut imin: libc::c_int = 0 as libc::c_int;
    let mut imax: libc::c_int = (::core::mem::size_of::<[te_variable; 25]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<te_variable>() as libc::c_ulong)
        .wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    while imax >= imin {
        let i: libc::c_int = imin + (imax - imin) / 2 as libc::c_int;
        let mut c: libc::c_int = strncmp(
            name,
            functions[i as usize].name,
            len as libc::c_ulong,
        );
        if c == 0 {
            c = '\0' as i32
                - *(functions[i as usize].name).offset(len as isize) as libc::c_int;
        }
        if c == 0 as libc::c_int {
            return functions.as_ptr().offset(i as isize)
        } else if c > 0 as libc::c_int {
            imin = i + 1 as libc::c_int;
        } else {
            imax = i - 1 as libc::c_int;
        }
    }
    return 0 as *const te_variable;
}
unsafe extern "C" fn find_lookup(
    mut s: *const state,
    mut name: *const libc::c_char,
    mut len: libc::c_int,
) -> *const te_variable {
    let mut iters: libc::c_int = 0;
    let mut var: *const te_variable = 0 as *const te_variable;
    if ((*s).lookup).is_null() {
        return 0 as *const te_variable;
    }
    var = (*s).lookup;
    iters = (*s).lookup_len;
    while iters != 0 {
        if strncmp(name, (*var).name, len as libc::c_ulong) == 0 as libc::c_int
            && *((*var).name).offset(len as isize) as libc::c_int == '\0' as i32
        {
            return var;
        }
        var = var.offset(1);
        var;
        iters -= 1;
        iters;
    }
    return 0 as *const te_variable;
}
unsafe extern "C" fn add(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return a + b;
}
unsafe extern "C" fn sub(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return a - b;
}
unsafe extern "C" fn mul(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return a * b;
}
unsafe extern "C" fn divide(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return a / b;
}
unsafe extern "C" fn negate(mut a: libc::c_double) -> libc::c_double {
    return -a;
}
unsafe extern "C" fn comma(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return b;
}
unsafe extern "C" fn greater(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return (a > b) as libc::c_int as libc::c_double;
}
unsafe extern "C" fn greater_eq(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return (a >= b) as libc::c_int as libc::c_double;
}
unsafe extern "C" fn lower(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return (a < b) as libc::c_int as libc::c_double;
}
unsafe extern "C" fn lower_eq(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return (a <= b) as libc::c_int as libc::c_double;
}
unsafe extern "C" fn equal(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return (a == b) as libc::c_int as libc::c_double;
}
unsafe extern "C" fn not_equal(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return (a != b) as libc::c_int as libc::c_double;
}
unsafe extern "C" fn logical_and(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return (a != 0.0f64 && b != 0.0f64) as libc::c_int as libc::c_double;
}
unsafe extern "C" fn logical_or(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return (a != 0.0f64 || b != 0.0f64) as libc::c_int as libc::c_double;
}
unsafe extern "C" fn logical_not(mut a: libc::c_double) -> libc::c_double {
    return (a == 0.0f64) as libc::c_int as libc::c_double;
}
unsafe extern "C" fn logical_notnot(mut a: libc::c_double) -> libc::c_double {
    return (a != 0.0f64) as libc::c_int as libc::c_double;
}
unsafe extern "C" fn negate_logical_not(mut a: libc::c_double) -> libc::c_double {
    return -((a == 0.0f64) as libc::c_int) as libc::c_double;
}
unsafe extern "C" fn negate_logical_notnot(mut a: libc::c_double) -> libc::c_double {
    return -((a != 0.0f64) as libc::c_int) as libc::c_double;
}
unsafe extern "C" fn parse_number(mut s: *mut state) -> libc::c_double {
    let mut p: *const libc::c_char = (*s).next;
    let mut value: libc::c_double = 0.0f64;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        && (*p.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
            || *p.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
        && *(*__ctype_b_loc())
            .offset(
                *p.offset(2 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                    as isize,
            ) as libc::c_int & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(2 as libc::c_int as isize);
        while *(*__ctype_b_loc())
            .offset(
                *p.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                    as isize,
            ) as libc::c_int & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            let c: libc::c_char = *p.offset(0 as libc::c_int as isize);
            let d: libc::c_int = if *(*__ctype_b_loc())
                .offset(c as libc::c_uchar as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                c as libc::c_int - '0' as i32
            } else if c as libc::c_int >= 'a' as i32 {
                c as libc::c_int - 'a' as i32 + 10 as libc::c_int
            } else {
                c as libc::c_int - 'A' as i32 + 10 as libc::c_int
            };
            value = value * 16.0f64 + d as libc::c_double;
            p = p.offset(1);
            p;
        }
    } else {
        let mut exponent: libc::c_int = 0 as libc::c_int;
        let mut digits: libc::c_int = 0 as libc::c_int;
        while *(*__ctype_b_loc())
            .offset(
                *p.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                    as isize,
            ) as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            value = value * 10.0f64
                + (*p.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
                    as libc::c_double;
            p = p.offset(1);
            p;
            digits += 1;
            digits;
        }
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
            p = p.offset(1);
            p;
            while *(*__ctype_b_loc())
                .offset(
                    *p.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                value = value * 10.0f64
                    + (*p.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
                        as libc::c_double;
                p = p.offset(1);
                p;
                digits += 1;
                digits;
                exponent -= 1;
                exponent;
            }
        }
        if digits == 0 as libc::c_int {
            (*s).type_0 = TOK_ERROR as libc::c_int;
            return 0.0f64;
        }
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'e' as i32
            || *p.offset(0 as libc::c_int as isize) as libc::c_int == 'E' as i32
        {
            let mut ep: *const libc::c_char = p.offset(1 as libc::c_int as isize);
            let mut esign: libc::c_int = 1 as libc::c_int;
            let mut e_0: libc::c_int = 0 as libc::c_int;
            if *ep.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
                ep = ep.offset(1);
                ep;
            } else if *ep.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            {
                esign = -(1 as libc::c_int);
                ep = ep.offset(1);
                ep;
            }
            if *(*__ctype_b_loc())
                .offset(
                    *ep.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                while *(*__ctype_b_loc())
                    .offset(
                        *ep.offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    if e_0 < 100000000 as libc::c_int {
                        e_0 = e_0 * 10 as libc::c_int
                            + (*ep.offset(0 as libc::c_int as isize) as libc::c_int
                                - '0' as i32);
                    }
                    ep = ep.offset(1);
                    ep;
                }
                exponent += esign * e_0;
                p = ep;
            }
        }
        if exponent != 0 && value != 0.0f64 {
            value *= pow(10.0f64, exponent as libc::c_double);
        }
    }
    (*s).type_0 = TOK_NUMBER as libc::c_int;
    (*s).next = p;
    return value;
}
unsafe extern "C" fn next_token(mut s: *mut state) {
    (*s).type_0 = TOK_NULL as libc::c_int;
    loop {
        if *(*s).next == 0 {
            (*s).type_0 = TOK_END as libc::c_int;
            return;
        }
        if *((*s).next).offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
            && *((*s).next).offset(0 as libc::c_int as isize) as libc::c_int
                <= '9' as i32
            || *((*s).next).offset(0 as libc::c_int as isize) as libc::c_int
                == '.' as i32
        {
            (*s).c2rust_unnamed.value = parse_number(s);
        } else if *(*__ctype_b_loc())
            .offset(
                *((*s).next).offset(0 as libc::c_int as isize) as libc::c_uchar
                    as libc::c_int as isize,
            ) as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            let mut start: *const libc::c_char = 0 as *const libc::c_char;
            start = (*s).next;
            while *(*__ctype_b_loc())
                .offset(
                    *((*s).next).offset(0 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *(*__ctype_b_loc())
                    .offset(
                        *((*s).next).offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *((*s).next).offset(0 as libc::c_int as isize) as libc::c_int
                    == '_' as i32
            {
                (*s).next = ((*s).next).offset(1);
                (*s).next;
            }
            let mut var: *const te_variable = find_lookup(
                s,
                start,
                ((*s).next).offset_from(start) as libc::c_long as libc::c_int,
            );
            if var.is_null() {
                var = find_builtin(
                    start,
                    ((*s).next).offset_from(start) as libc::c_long as libc::c_int,
                );
            }
            if var.is_null() {
                (*s).type_0 = TOK_ERROR as libc::c_int;
            } else {
                let mut current_block_18: u64;
                match (*var).type_0 & 0x1f as libc::c_int {
                    0 => {
                        (*s).type_0 = TOK_VARIABLE as libc::c_int;
                        (*s)
                            .c2rust_unnamed
                            .bound = (*var).address as *const libc::c_double;
                        current_block_18 = 9828876828309294594;
                    }
                    16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 => {
                        (*s).context = (*var).context;
                        current_block_18 = 15788862888931182500;
                    }
                    8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
                        current_block_18 = 15788862888931182500;
                    }
                    _ => {
                        current_block_18 = 9828876828309294594;
                    }
                }
                match current_block_18 {
                    15788862888931182500 => {
                        (*s).type_0 = (*var).type_0;
                        (*s).c2rust_unnamed.function = (*var).address;
                    }
                    _ => {}
                }
            }
        } else {
            let fresh0 = (*s).next;
            (*s).next = ((*s).next).offset(1);
            match *fresh0.offset(0 as libc::c_int as isize) as libc::c_int {
                43 => {
                    (*s).type_0 = TOK_INFIX as libc::c_int;
                    (*s)
                        .c2rust_unnamed
                        .function = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                        *const libc::c_void,
                    >(
                        Some(
                            add
                                as unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                        ),
                    );
                }
                45 => {
                    (*s).type_0 = TOK_INFIX as libc::c_int;
                    (*s)
                        .c2rust_unnamed
                        .function = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                        *const libc::c_void,
                    >(
                        Some(
                            sub
                                as unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                        ),
                    );
                }
                42 => {
                    (*s).type_0 = TOK_INFIX as libc::c_int;
                    (*s)
                        .c2rust_unnamed
                        .function = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                        *const libc::c_void,
                    >(
                        Some(
                            mul
                                as unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                        ),
                    );
                }
                47 => {
                    (*s).type_0 = TOK_INFIX as libc::c_int;
                    (*s)
                        .c2rust_unnamed
                        .function = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                        *const libc::c_void,
                    >(
                        Some(
                            divide
                                as unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                        ),
                    );
                }
                94 => {
                    (*s).type_0 = TOK_INFIX as libc::c_int;
                    (*s)
                        .c2rust_unnamed
                        .function = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                        *const libc::c_void,
                    >(
                        Some(
                            pow
                                as unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                        ),
                    );
                }
                37 => {
                    (*s).type_0 = TOK_INFIX as libc::c_int;
                    (*s)
                        .c2rust_unnamed
                        .function = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                        *const libc::c_void,
                    >(
                        Some(
                            fmod
                                as unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                        ),
                    );
                }
                33 => {
                    let fresh1 = (*s).next;
                    (*s).next = ((*s).next).offset(1);
                    if *fresh1.offset(0 as libc::c_int as isize) as libc::c_int
                        == '=' as i32
                    {
                        (*s).type_0 = TOK_INFIX as libc::c_int;
                        (*s)
                            .c2rust_unnamed
                            .function = ::core::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                            >,
                            *const libc::c_void,
                        >(
                            Some(
                                not_equal
                                    as unsafe extern "C" fn(
                                        libc::c_double,
                                        libc::c_double,
                                    ) -> libc::c_double,
                            ),
                        );
                    } else {
                        (*s).next = ((*s).next).offset(-1);
                        (*s).next;
                        (*s).type_0 = TOK_INFIX as libc::c_int;
                        (*s)
                            .c2rust_unnamed
                            .function = ::core::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                            >,
                            *const libc::c_void,
                        >(
                            Some(
                                logical_not
                                    as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                            ),
                        );
                    }
                }
                61 => {
                    let fresh2 = (*s).next;
                    (*s).next = ((*s).next).offset(1);
                    if *fresh2.offset(0 as libc::c_int as isize) as libc::c_int
                        == '=' as i32
                    {
                        (*s).type_0 = TOK_INFIX as libc::c_int;
                        (*s)
                            .c2rust_unnamed
                            .function = ::core::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                            >,
                            *const libc::c_void,
                        >(
                            Some(
                                equal
                                    as unsafe extern "C" fn(
                                        libc::c_double,
                                        libc::c_double,
                                    ) -> libc::c_double,
                            ),
                        );
                    } else {
                        (*s).next = ((*s).next).offset(-1);
                        (*s).next;
                        (*s).type_0 = TOK_ERROR as libc::c_int;
                    }
                }
                60 => {
                    let fresh3 = (*s).next;
                    (*s).next = ((*s).next).offset(1);
                    if *fresh3.offset(0 as libc::c_int as isize) as libc::c_int
                        == '=' as i32
                    {
                        (*s).type_0 = TOK_INFIX as libc::c_int;
                        (*s)
                            .c2rust_unnamed
                            .function = ::core::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                            >,
                            *const libc::c_void,
                        >(
                            Some(
                                lower_eq
                                    as unsafe extern "C" fn(
                                        libc::c_double,
                                        libc::c_double,
                                    ) -> libc::c_double,
                            ),
                        );
                    } else {
                        (*s).next = ((*s).next).offset(-1);
                        (*s).next;
                        (*s).type_0 = TOK_INFIX as libc::c_int;
                        (*s)
                            .c2rust_unnamed
                            .function = ::core::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                            >,
                            *const libc::c_void,
                        >(
                            Some(
                                lower
                                    as unsafe extern "C" fn(
                                        libc::c_double,
                                        libc::c_double,
                                    ) -> libc::c_double,
                            ),
                        );
                    }
                }
                62 => {
                    let fresh4 = (*s).next;
                    (*s).next = ((*s).next).offset(1);
                    if *fresh4.offset(0 as libc::c_int as isize) as libc::c_int
                        == '=' as i32
                    {
                        (*s).type_0 = TOK_INFIX as libc::c_int;
                        (*s)
                            .c2rust_unnamed
                            .function = ::core::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                            >,
                            *const libc::c_void,
                        >(
                            Some(
                                greater_eq
                                    as unsafe extern "C" fn(
                                        libc::c_double,
                                        libc::c_double,
                                    ) -> libc::c_double,
                            ),
                        );
                    } else {
                        (*s).next = ((*s).next).offset(-1);
                        (*s).next;
                        (*s).type_0 = TOK_INFIX as libc::c_int;
                        (*s)
                            .c2rust_unnamed
                            .function = ::core::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                            >,
                            *const libc::c_void,
                        >(
                            Some(
                                greater
                                    as unsafe extern "C" fn(
                                        libc::c_double,
                                        libc::c_double,
                                    ) -> libc::c_double,
                            ),
                        );
                    }
                }
                38 => {
                    let fresh5 = (*s).next;
                    (*s).next = ((*s).next).offset(1);
                    if *fresh5.offset(0 as libc::c_int as isize) as libc::c_int
                        == '&' as i32
                    {
                        (*s).type_0 = TOK_INFIX as libc::c_int;
                        (*s)
                            .c2rust_unnamed
                            .function = ::core::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                            >,
                            *const libc::c_void,
                        >(
                            Some(
                                logical_and
                                    as unsafe extern "C" fn(
                                        libc::c_double,
                                        libc::c_double,
                                    ) -> libc::c_double,
                            ),
                        );
                    } else {
                        (*s).next = ((*s).next).offset(-1);
                        (*s).next;
                        (*s).type_0 = TOK_ERROR as libc::c_int;
                    }
                }
                124 => {
                    let fresh6 = (*s).next;
                    (*s).next = ((*s).next).offset(1);
                    if *fresh6.offset(0 as libc::c_int as isize) as libc::c_int
                        == '|' as i32
                    {
                        (*s).type_0 = TOK_INFIX as libc::c_int;
                        (*s)
                            .c2rust_unnamed
                            .function = ::core::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    libc::c_double,
                                    libc::c_double,
                                ) -> libc::c_double,
                            >,
                            *const libc::c_void,
                        >(
                            Some(
                                logical_or
                                    as unsafe extern "C" fn(
                                        libc::c_double,
                                        libc::c_double,
                                    ) -> libc::c_double,
                            ),
                        );
                    } else {
                        (*s).next = ((*s).next).offset(-1);
                        (*s).next;
                        (*s).type_0 = TOK_ERROR as libc::c_int;
                    }
                }
                40 => {
                    (*s).type_0 = TOK_OPEN as libc::c_int;
                }
                41 => {
                    (*s).type_0 = TOK_CLOSE as libc::c_int;
                }
                44 => {
                    (*s).type_0 = TOK_SEP as libc::c_int;
                }
                32 | 9 | 10 | 13 => {}
                _ => {
                    (*s).type_0 = TOK_ERROR as libc::c_int;
                }
            }
        }
        if !((*s).type_0 == TOK_NULL as libc::c_int) {
            break;
        }
    };
}
unsafe extern "C" fn base(mut s: *mut state) -> *mut te_expr {
    if (*s).depth >= 512 as libc::c_int {
        (*s).type_0 = TOK_ERROR as libc::c_int;
        return 0 as *mut te_expr;
    }
    (*s).depth += 1;
    (*s).depth;
    let mut ret: *mut te_expr = base_impl(s);
    (*s).depth -= 1;
    (*s).depth;
    return ret;
}
unsafe extern "C" fn base_impl(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = 0 as *mut te_expr;
    let mut arity: libc::c_int = 0;
    match (*s).type_0 & 0x1f as libc::c_int {
        30 => {
            ret = new_expr(TE_CONSTANT as libc::c_int, 0 as *mut *const te_expr);
            if ret.is_null() {
                return 0 as *mut te_expr;
            }
            (*ret).c2rust_unnamed.value = (*s).c2rust_unnamed.value;
            next_token(s);
        }
        31 => {
            ret = new_expr(TE_VARIABLE as libc::c_int, 0 as *mut *const te_expr);
            if ret.is_null() {
                return 0 as *mut te_expr;
            }
            (*ret).c2rust_unnamed.bound = (*s).c2rust_unnamed.bound;
            next_token(s);
        }
        8 | 16 => {
            ret = new_expr((*s).type_0, 0 as *mut *const te_expr);
            if ret.is_null() {
                return 0 as *mut te_expr;
            }
            (*ret).c2rust_unnamed.function = (*s).c2rust_unnamed.function;
            if (*s).type_0 & TE_CLOSURE0 as libc::c_int != 0 as libc::c_int {
                let ref mut fresh7 = *((*ret).parameters)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize);
                *fresh7 = (*s).context;
            }
            next_token(s);
            if (*s).type_0 == TOK_OPEN as libc::c_int {
                next_token(s);
                if (*s).type_0 != TOK_CLOSE as libc::c_int {
                    (*s).type_0 = TOK_ERROR as libc::c_int;
                } else {
                    next_token(s);
                }
            }
        }
        9 | 17 => {
            ret = new_expr((*s).type_0, 0 as *mut *const te_expr);
            if ret.is_null() {
                return 0 as *mut te_expr;
            }
            (*ret).c2rust_unnamed.function = (*s).c2rust_unnamed.function;
            if (*s).type_0 & TE_CLOSURE0 as libc::c_int != 0 as libc::c_int {
                let ref mut fresh8 = *((*ret).parameters)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize);
                *fresh8 = (*s).context;
            }
            next_token(s);
            let ref mut fresh9 = *((*ret).parameters)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize);
            *fresh9 = power(s) as *mut libc::c_void;
            if (*((*ret).parameters).as_mut_ptr().offset(0 as libc::c_int as isize))
                .is_null()
            {
                te_free(ret);
                return 0 as *mut te_expr;
            }
        }
        10 | 11 | 12 | 13 | 14 | 15 | 18 | 19 | 20 | 21 | 22 | 23 => {
            arity = if (*s).type_0
                & (TE_FUNCTION0 as libc::c_int | TE_CLOSURE0 as libc::c_int) != 0
            {
                (*s).type_0 & 0x7 as libc::c_int
            } else {
                0 as libc::c_int
            };
            ret = new_expr((*s).type_0, 0 as *mut *const te_expr);
            if ret.is_null() {
                return 0 as *mut te_expr;
            }
            (*ret).c2rust_unnamed.function = (*s).c2rust_unnamed.function;
            if (*s).type_0 & TE_CLOSURE0 as libc::c_int != 0 as libc::c_int {
                let ref mut fresh10 = *((*ret).parameters)
                    .as_mut_ptr()
                    .offset(arity as isize);
                *fresh10 = (*s).context;
            }
            next_token(s);
            if (*s).type_0 != TOK_OPEN as libc::c_int {
                (*s).type_0 = TOK_ERROR as libc::c_int;
            } else {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < arity {
                    next_token(s);
                    let ref mut fresh11 = *((*ret).parameters)
                        .as_mut_ptr()
                        .offset(i as isize);
                    *fresh11 = expr(s) as *mut libc::c_void;
                    if (*((*ret).parameters).as_mut_ptr().offset(i as isize)).is_null() {
                        te_free(ret);
                        return 0 as *mut te_expr;
                    }
                    if (*s).type_0 != TOK_SEP as libc::c_int {
                        break;
                    }
                    i += 1;
                    i;
                }
                if (*s).type_0 != TOK_CLOSE as libc::c_int
                    || i != arity - 1 as libc::c_int
                {
                    (*s).type_0 = TOK_ERROR as libc::c_int;
                } else {
                    next_token(s);
                }
            }
        }
        28 => {
            next_token(s);
            ret = list(s);
            if ret.is_null() {
                return 0 as *mut te_expr;
            }
            if (*s).type_0 != TOK_CLOSE as libc::c_int {
                (*s).type_0 = TOK_ERROR as libc::c_int;
            } else {
                next_token(s);
            }
        }
        _ => {
            ret = new_expr(0 as libc::c_int, 0 as *mut *const te_expr);
            if ret.is_null() {
                return 0 as *mut te_expr;
            }
            (*s).type_0 = TOK_ERROR as libc::c_int;
            (*ret).c2rust_unnamed.value = ::core::f32::NAN as libc::c_double;
        }
    }
    return ret;
}
unsafe extern "C" fn power(mut s: *mut state) -> *mut te_expr {
    let mut sign: libc::c_int = 1 as libc::c_int;
    while (*s).type_0 == TOK_INFIX as libc::c_int
        && ((*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
                >,
                *const libc::c_void,
            >(
                Some(
                    add
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
            )
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        sub
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ))
    {
        if (*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
                >,
                *const libc::c_void,
            >(
                Some(
                    sub
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
            )
        {
            sign = -sign;
        }
        next_token(s);
    }
    let mut logical: libc::c_int = 0 as libc::c_int;
    while (*s).type_0 == TOK_INFIX as libc::c_int
        && ((*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
                >,
                *const libc::c_void,
            >(
                Some(
                    add
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
            )
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        sub
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                )
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(
                    Some(
                        logical_not
                            as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                    ),
                ))
    {
        if (*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                *const libc::c_void,
            >(
                Some(
                    logical_not as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
            )
        {
            if logical == 0 as libc::c_int {
                logical = -(1 as libc::c_int);
            } else {
                logical = -logical;
            }
        }
        next_token(s);
    }
    let mut ret: *mut te_expr = 0 as *mut te_expr;
    if sign == 1 as libc::c_int {
        if logical == 0 as libc::c_int {
            ret = base(s);
        } else {
            let mut b: *mut te_expr = base(s);
            if b.is_null() {
                return 0 as *mut te_expr;
            }
            ret = new_expr(
                TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
                [b as *const te_expr].as_mut_ptr(),
            );
            if ret.is_null() {
                te_free(b);
                return 0 as *mut te_expr;
            }
            if logical == -(1 as libc::c_int) {
                (*ret)
                    .c2rust_unnamed
                    .function = ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(
                    Some(
                        logical_not
                            as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                    ),
                );
            } else {
                (*ret)
                    .c2rust_unnamed
                    .function = ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(
                    Some(
                        logical_notnot
                            as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                    ),
                );
            }
        }
    } else {
        let mut b_0: *mut te_expr = base(s);
        if b_0.is_null() {
            return 0 as *mut te_expr;
        }
        ret = new_expr(
            TE_FUNCTION1 as libc::c_int | TE_FLAG_PURE as libc::c_int,
            [b_0 as *const te_expr].as_mut_ptr(),
        );
        if ret.is_null() {
            te_free(b_0);
            return 0 as *mut te_expr;
        }
        if logical == 0 as libc::c_int {
            (*ret)
                .c2rust_unnamed
                .function = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                *const libc::c_void,
            >(Some(negate as unsafe extern "C" fn(libc::c_double) -> libc::c_double));
        } else if logical == -(1 as libc::c_int) {
            (*ret)
                .c2rust_unnamed
                .function = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                *const libc::c_void,
            >(
                Some(
                    negate_logical_not
                        as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
            );
        } else {
            (*ret)
                .c2rust_unnamed
                .function = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                *const libc::c_void,
            >(
                Some(
                    negate_logical_notnot
                        as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                ),
            );
        }
    }
    (*s)
        .applied_unary = (sign != 1 as libc::c_int || logical != 0 as libc::c_int)
        as libc::c_int;
    return ret;
}
unsafe extern "C" fn factor(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = power(s);
    if ret.is_null() {
        return 0 as *mut te_expr;
    }
    while (*s).type_0 == TOK_INFIX as libc::c_int
        && (*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
                >,
                *const libc::c_void,
            >(
                Some(
                    pow
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
            )
    {
        let mut t: te_fun2 = ::core::mem::transmute::<
            *const libc::c_void,
            te_fun2,
        >((*s).c2rust_unnamed.function);
        next_token(s);
        let mut p: *mut te_expr = power(s);
        if p.is_null() {
            te_free(ret);
            return 0 as *mut te_expr;
        }
        let mut prev: *mut te_expr = ret;
        ret = new_expr(
            TE_FUNCTION2 as libc::c_int | TE_FLAG_PURE as libc::c_int,
            [ret as *const te_expr, p as *const te_expr].as_mut_ptr(),
        );
        if ret.is_null() {
            te_free(p);
            te_free(prev);
            return 0 as *mut te_expr;
        }
        (*ret)
            .c2rust_unnamed
            .function = ::core::mem::transmute::<te_fun2, *const libc::c_void>(t);
    }
    return ret;
}
unsafe extern "C" fn term(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = factor(s);
    if ret.is_null() {
        return 0 as *mut te_expr;
    }
    while (*s).type_0 == TOK_INFIX as libc::c_int
        && ((*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
                >,
                *const libc::c_void,
            >(
                Some(
                    mul
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
            )
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        divide
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                )
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        fmod
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ))
    {
        let mut t: te_fun2 = ::core::mem::transmute::<
            *const libc::c_void,
            te_fun2,
        >((*s).c2rust_unnamed.function);
        next_token(s);
        let mut f: *mut te_expr = factor(s);
        if f.is_null() {
            te_free(ret);
            return 0 as *mut te_expr;
        }
        let mut prev: *mut te_expr = ret;
        ret = new_expr(
            TE_FUNCTION2 as libc::c_int | TE_FLAG_PURE as libc::c_int,
            [ret as *const te_expr, f as *const te_expr].as_mut_ptr(),
        );
        if ret.is_null() {
            te_free(f);
            te_free(prev);
            return 0 as *mut te_expr;
        }
        (*ret)
            .c2rust_unnamed
            .function = ::core::mem::transmute::<te_fun2, *const libc::c_void>(t);
    }
    return ret;
}
unsafe extern "C" fn sum_expr(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = term(s);
    if ret.is_null() {
        return 0 as *mut te_expr;
    }
    while (*s).type_0 == TOK_INFIX as libc::c_int
        && ((*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
                >,
                *const libc::c_void,
            >(
                Some(
                    add
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
            )
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        sub
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ))
    {
        let mut t: te_fun2 = ::core::mem::transmute::<
            *const libc::c_void,
            te_fun2,
        >((*s).c2rust_unnamed.function);
        next_token(s);
        let mut te: *mut te_expr = term(s);
        if te.is_null() {
            te_free(ret);
            return 0 as *mut te_expr;
        }
        let mut prev: *mut te_expr = ret;
        ret = new_expr(
            TE_FUNCTION2 as libc::c_int | TE_FLAG_PURE as libc::c_int,
            [ret as *const te_expr, te as *const te_expr].as_mut_ptr(),
        );
        if ret.is_null() {
            te_free(te);
            te_free(prev);
            return 0 as *mut te_expr;
        }
        (*ret)
            .c2rust_unnamed
            .function = ::core::mem::transmute::<te_fun2, *const libc::c_void>(t);
    }
    return ret;
}
unsafe extern "C" fn rel_expr(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = sum_expr(s);
    if ret.is_null() {
        return 0 as *mut te_expr;
    }
    while (*s).type_0 == TOK_INFIX as libc::c_int
        && ((*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
                >,
                *const libc::c_void,
            >(
                Some(
                    greater
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
            )
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        greater_eq
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                )
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        lower
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                )
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        lower_eq
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ))
    {
        let mut t: te_fun2 = ::core::mem::transmute::<
            *const libc::c_void,
            te_fun2,
        >((*s).c2rust_unnamed.function);
        next_token(s);
        let mut e_0: *mut te_expr = sum_expr(s);
        if e_0.is_null() {
            te_free(ret);
            return 0 as *mut te_expr;
        }
        let mut prev: *mut te_expr = ret;
        ret = new_expr(
            TE_FUNCTION2 as libc::c_int | TE_FLAG_PURE as libc::c_int,
            [ret as *const te_expr, e_0 as *const te_expr].as_mut_ptr(),
        );
        if ret.is_null() {
            te_free(e_0);
            te_free(prev);
            return 0 as *mut te_expr;
        }
        (*ret)
            .c2rust_unnamed
            .function = ::core::mem::transmute::<te_fun2, *const libc::c_void>(t);
    }
    return ret;
}
unsafe extern "C" fn eq_expr(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = rel_expr(s);
    if ret.is_null() {
        return 0 as *mut te_expr;
    }
    while (*s).type_0 == TOK_INFIX as libc::c_int
        && ((*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
                >,
                *const libc::c_void,
            >(
                Some(
                    equal
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
            )
            || (*s).c2rust_unnamed.function
                == ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        not_equal
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ))
    {
        let mut t: te_fun2 = ::core::mem::transmute::<
            *const libc::c_void,
            te_fun2,
        >((*s).c2rust_unnamed.function);
        next_token(s);
        let mut e_0: *mut te_expr = rel_expr(s);
        if e_0.is_null() {
            te_free(ret);
            return 0 as *mut te_expr;
        }
        let mut prev: *mut te_expr = ret;
        ret = new_expr(
            TE_FUNCTION2 as libc::c_int | TE_FLAG_PURE as libc::c_int,
            [ret as *const te_expr, e_0 as *const te_expr].as_mut_ptr(),
        );
        if ret.is_null() {
            te_free(e_0);
            te_free(prev);
            return 0 as *mut te_expr;
        }
        (*ret)
            .c2rust_unnamed
            .function = ::core::mem::transmute::<te_fun2, *const libc::c_void>(t);
    }
    return ret;
}
unsafe extern "C" fn and_expr(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = eq_expr(s);
    if ret.is_null() {
        return 0 as *mut te_expr;
    }
    while (*s).type_0 == TOK_INFIX as libc::c_int
        && (*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
                >,
                *const libc::c_void,
            >(
                Some(
                    logical_and
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
            )
    {
        next_token(s);
        let mut e_0: *mut te_expr = eq_expr(s);
        if e_0.is_null() {
            te_free(ret);
            return 0 as *mut te_expr;
        }
        let mut prev: *mut te_expr = ret;
        ret = new_expr(
            TE_FUNCTION2 as libc::c_int | TE_FLAG_PURE as libc::c_int,
            [ret as *const te_expr, e_0 as *const te_expr].as_mut_ptr(),
        );
        if ret.is_null() {
            te_free(e_0);
            te_free(prev);
            return 0 as *mut te_expr;
        }
        (*ret)
            .c2rust_unnamed
            .function = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double,
            >,
            *const libc::c_void,
        >(
            Some(
                logical_and
                    as unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
            ),
        );
    }
    return ret;
}
unsafe extern "C" fn expr(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = and_expr(s);
    if ret.is_null() {
        return 0 as *mut te_expr;
    }
    while (*s).type_0 == TOK_INFIX as libc::c_int
        && (*s).c2rust_unnamed.function
            == ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
                >,
                *const libc::c_void,
            >(
                Some(
                    logical_or
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
            )
    {
        next_token(s);
        let mut e_0: *mut te_expr = and_expr(s);
        if e_0.is_null() {
            te_free(ret);
            return 0 as *mut te_expr;
        }
        let mut prev: *mut te_expr = ret;
        ret = new_expr(
            TE_FUNCTION2 as libc::c_int | TE_FLAG_PURE as libc::c_int,
            [ret as *const te_expr, e_0 as *const te_expr].as_mut_ptr(),
        );
        if ret.is_null() {
            te_free(e_0);
            te_free(prev);
            return 0 as *mut te_expr;
        }
        (*ret)
            .c2rust_unnamed
            .function = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double,
            >,
            *const libc::c_void,
        >(
            Some(
                logical_or
                    as unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
            ),
        );
    }
    return ret;
}
unsafe extern "C" fn list(mut s: *mut state) -> *mut te_expr {
    let mut ret: *mut te_expr = expr(s);
    if ret.is_null() {
        return 0 as *mut te_expr;
    }
    while (*s).type_0 == TOK_SEP as libc::c_int {
        next_token(s);
        let mut e_0: *mut te_expr = expr(s);
        if e_0.is_null() {
            te_free(ret);
            return 0 as *mut te_expr;
        }
        let mut prev: *mut te_expr = ret;
        ret = new_expr(
            TE_FUNCTION2 as libc::c_int | TE_FLAG_PURE as libc::c_int,
            [ret as *const te_expr, e_0 as *const te_expr].as_mut_ptr(),
        );
        if ret.is_null() {
            te_free(e_0);
            te_free(prev);
            return 0 as *mut te_expr;
        }
        (*ret)
            .c2rust_unnamed
            .function = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double,
            >,
            *const libc::c_void,
        >(
            Some(
                comma
                    as unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
            ),
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn te_eval(mut n: *const te_expr) -> libc::c_double {
    if n.is_null() {
        return ::core::f32::NAN as libc::c_double;
    }
    match (*n).type_0 & 0x1f as libc::c_int {
        1 => return (*n).c2rust_unnamed.value,
        0 => return *(*n).c2rust_unnamed.bound,
        8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            match if (*n).type_0
                & (TE_FUNCTION0 as libc::c_int | TE_CLOSURE0 as libc::c_int) != 0
            {
                (*n).type_0 & 0x7 as libc::c_int
            } else {
                0 as libc::c_int
            } {
                0 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<unsafe extern "C" fn() -> libc::c_double>,
                    >((*n).c2rust_unnamed.function))
                        .expect("non-null function pointer")();
                }
                1 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                2 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(1 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                3 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(1 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(2 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                4 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(1 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(2 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(3 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                5 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(1 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(2 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(3 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(4 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                6 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(1 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(2 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(3 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(4 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(5 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                7 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(1 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(2 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(3 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(4 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(5 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(6 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                _ => return ::core::f32::NAN as libc::c_double,
            }
        }
        16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 => {
            match if (*n).type_0
                & (TE_FUNCTION0 as libc::c_int | TE_CLOSURE0 as libc::c_int) != 0
            {
                (*n).type_0 & 0x7 as libc::c_int
            } else {
                0 as libc::c_int
            } {
                0 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(*((*n).parameters).as_ptr().offset(0 as libc::c_int as isize));
                }
                1 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        *((*n).parameters).as_ptr().offset(1 as libc::c_int as isize),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                2 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        *((*n).parameters).as_ptr().offset(2 as libc::c_int as isize),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(1 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                3 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        *((*n).parameters).as_ptr().offset(3 as libc::c_int as isize),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(1 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(2 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                4 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        *((*n).parameters).as_ptr().offset(4 as libc::c_int as isize),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(1 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(2 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(3 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                5 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        *((*n).parameters).as_ptr().offset(5 as libc::c_int as isize),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(1 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(2 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(3 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(4 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                6 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        *((*n).parameters).as_ptr().offset(6 as libc::c_int as isize),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(1 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(2 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(3 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(4 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(5 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                7 => {
                    return (::core::mem::transmute::<
                        *const libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                        >,
                    >((*n).c2rust_unnamed.function))
                        .expect(
                            "non-null function pointer",
                        )(
                        *((*n).parameters).as_ptr().offset(7 as libc::c_int as isize),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(0 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(1 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(2 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(3 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(4 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(5 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                        te_eval(
                            *((*n).parameters).as_ptr().offset(6 as libc::c_int as isize)
                                as *const te_expr,
                        ),
                    );
                }
                _ => return ::core::f32::NAN as libc::c_double,
            }
        }
        _ => return ::core::f32::NAN as libc::c_double,
    };
}
unsafe extern "C" fn optimize(mut n: *mut te_expr) {
    if (*n).type_0 == TE_CONSTANT as libc::c_int {
        return;
    }
    if (*n).type_0 == TE_VARIABLE as libc::c_int {
        return;
    }
    if (*n).type_0 & TE_FLAG_PURE as libc::c_int != 0 as libc::c_int {
        let arity: libc::c_int = if (*n).type_0
            & (TE_FUNCTION0 as libc::c_int | TE_CLOSURE0 as libc::c_int) != 0
        {
            (*n).type_0 & 0x7 as libc::c_int
        } else {
            0 as libc::c_int
        };
        let mut known: libc::c_int = 1 as libc::c_int;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < arity {
            optimize(*((*n).parameters).as_mut_ptr().offset(i as isize) as *mut te_expr);
            if (*(*((*n).parameters).as_mut_ptr().offset(i as isize) as *mut te_expr))
                .type_0 != TE_CONSTANT as libc::c_int
            {
                known = 0 as libc::c_int;
            }
            i += 1;
            i;
        }
        if known != 0 {
            let value: libc::c_double = te_eval(n);
            te_free_parameters(n);
            (*n).type_0 = TE_CONSTANT as libc::c_int;
            (*n).c2rust_unnamed.value = value;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn te_compile(
    mut expression: *const libc::c_char,
    mut variables: *const te_variable,
    mut var_count: libc::c_int,
    mut error: *mut libc::c_int,
) -> *mut te_expr {
    let mut s: state = state {
        start: 0 as *const libc::c_char,
        next: 0 as *const libc::c_char,
        type_0: 0,
        c2rust_unnamed: C2RustUnnamed_1 { value: 0. },
        context: 0 as *mut libc::c_void,
        lookup: 0 as *const te_variable,
        lookup_len: 0,
        applied_unary: 0,
        depth: 0,
    };
    s.next = expression;
    s.start = s.next;
    s.lookup = variables;
    s.lookup_len = var_count;
    s.depth = 0 as libc::c_int;
    next_token(&mut s);
    let mut root: *mut te_expr = list(&mut s);
    if root.is_null() {
        if !error.is_null() {
            *error = -(1 as libc::c_int);
        }
        return 0 as *mut te_expr;
    }
    if s.type_0 != TOK_END as libc::c_int {
        te_free(root);
        if !error.is_null() {
            *error = (s.next).offset_from(s.start) as libc::c_long as libc::c_int;
            if *error == 0 as libc::c_int {
                *error = 1 as libc::c_int;
            }
        }
        return 0 as *mut te_expr;
    } else {
        optimize(root);
        if !error.is_null() {
            *error = 0 as libc::c_int;
        }
        return root;
    };
}
#[no_mangle]
pub unsafe extern "C" fn te_interp(
    mut expression: *const libc::c_char,
    mut error: *mut libc::c_int,
) -> libc::c_double {
    let mut n: *mut te_expr = te_compile(
        expression,
        0 as *const te_variable,
        0 as libc::c_int,
        error,
    );
    let mut ret: libc::c_double = 0.;
    if !n.is_null() {
        ret = te_eval(n);
        te_free(n);
    } else {
        ret = ::core::f32::NAN as libc::c_double;
    }
    return ret;
}
unsafe extern "C" fn pn(mut n: *const te_expr, mut depth: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    printf(
        b"%*s\0" as *const u8 as *const libc::c_char,
        depth,
        b"\0" as *const u8 as *const libc::c_char,
    );
    match (*n).type_0 & 0x1f as libc::c_int {
        1 => {
            printf(
                b"%f\n\0" as *const u8 as *const libc::c_char,
                (*n).c2rust_unnamed.value,
            );
        }
        0 => {
            printf(
                b"bound %p\n\0" as *const u8 as *const libc::c_char,
                (*n).c2rust_unnamed.bound,
            );
        }
        8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 => {
            arity = if (*n).type_0
                & (TE_FUNCTION0 as libc::c_int | TE_CLOSURE0 as libc::c_int) != 0
            {
                (*n).type_0 & 0x7 as libc::c_int
            } else {
                0 as libc::c_int
            };
            printf(b"f%d\0" as *const u8 as *const libc::c_char, arity);
            i = 0 as libc::c_int;
            while i < arity {
                printf(
                    b" %p\0" as *const u8 as *const libc::c_char,
                    *((*n).parameters).as_ptr().offset(i as isize),
                );
                i += 1;
                i;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < arity {
                pn(
                    *((*n).parameters).as_ptr().offset(i as isize) as *const te_expr,
                    depth + 1 as libc::c_int,
                );
                i += 1;
                i;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn te_print(mut n: *const te_expr) {
    pn(n, 0 as libc::c_int);
}
