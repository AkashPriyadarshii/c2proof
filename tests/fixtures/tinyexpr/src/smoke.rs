use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn te_interp(
        expression: *const libc::c_char,
        error: *mut libc::c_int,
    ) -> libc::c_double;
    fn te_compile(
        expression: *const libc::c_char,
        variables: *const te_variable,
        var_count: libc::c_int,
        error: *mut libc::c_int,
    ) -> *mut te_expr;
    fn te_eval(n: *const te_expr) -> libc::c_double;
    fn te_free(n: *mut te_expr);
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn localeconv() -> *mut lconv;
    fn clock() -> clock_t;
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
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
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
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test_case {
    pub expr: *const libc::c_char,
    pub answer: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test_equ {
    pub expr1: *const libc::c_char,
    pub expr2: *const libc::c_char,
}
static mut ltests: libc::c_int = 0 as libc::c_int;
static mut lfails: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn test_results() {
    let mut cases: [test_case; 79] = [
        {
            let mut init = test_case {
                expr: b"1\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 \0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(1)\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"pi\0" as *const u8 as *const libc::c_char,
                answer: 3.14159f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"atan(1)*4 - pi\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"e\0" as *const u8 as *const libc::c_char,
                answer: 2.71828f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2+1\0" as *const u8 as *const libc::c_char,
                answer: (2 as libc::c_int + 1 as libc::c_int) as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(((2+(1))))\0" as *const u8 as *const libc::c_char,
                answer: (2 as libc::c_int + 1 as libc::c_int) as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"3+2\0" as *const u8 as *const libc::c_char,
                answer: (3 as libc::c_int + 2 as libc::c_int) as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"3+2+4\0" as *const u8 as *const libc::c_char,
                answer: (3 as libc::c_int + 2 as libc::c_int + 4 as libc::c_int)
                    as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(3+2)+4\0" as *const u8 as *const libc::c_char,
                answer: (3 as libc::c_int + 2 as libc::c_int + 4 as libc::c_int)
                    as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"3+(2+4)\0" as *const u8 as *const libc::c_char,
                answer: (3 as libc::c_int + 2 as libc::c_int + 4 as libc::c_int)
                    as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(3+2+4)\0" as *const u8 as *const libc::c_char,
                answer: (3 as libc::c_int + 2 as libc::c_int + 4 as libc::c_int)
                    as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"3*2*4\0" as *const u8 as *const libc::c_char,
                answer: (3 as libc::c_int * 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(3*2)*4\0" as *const u8 as *const libc::c_char,
                answer: (3 as libc::c_int * 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"3*(2*4)\0" as *const u8 as *const libc::c_char,
                answer: (3 as libc::c_int * 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(3*2*4)\0" as *const u8 as *const libc::c_char,
                answer: (3 as libc::c_int * 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"3-2-4\0" as *const u8 as *const libc::c_char,
                answer: (3 as libc::c_int - 2 as libc::c_int - 4 as libc::c_int)
                    as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(3-2)-4\0" as *const u8 as *const libc::c_char,
                answer: (3 as libc::c_int - 2 as libc::c_int - 4 as libc::c_int)
                    as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"3-(2-4)\0" as *const u8 as *const libc::c_char,
                answer: (3 as libc::c_int - (2 as libc::c_int - 4 as libc::c_int))
                    as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(3-2-4)\0" as *const u8 as *const libc::c_char,
                answer: (3 as libc::c_int - 2 as libc::c_int - 4 as libc::c_int)
                    as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"3/2/4\0" as *const u8 as *const libc::c_char,
                answer: 3.0f64 / 2.0f64 / 4.0f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(3/2)/4\0" as *const u8 as *const libc::c_char,
                answer: 3.0f64 / 2.0f64 / 4.0f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"3/(2/4)\0" as *const u8 as *const libc::c_char,
                answer: 3.0f64 / (2.0f64 / 4.0f64),
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(3/2/4)\0" as *const u8 as *const libc::c_char,
                answer: 3.0f64 / 2.0f64 / 4.0f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(3*2/4)\0" as *const u8 as *const libc::c_char,
                answer: 3.0f64 * 2.0f64 / 4.0f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(3/2*4)\0" as *const u8 as *const libc::c_char,
                answer: 3.0f64 / 2.0f64 * 4.0f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"3*(2/4)\0" as *const u8 as *const libc::c_char,
                answer: 3.0f64 * (2.0f64 / 4.0f64),
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"asin sin .5\0" as *const u8 as *const libc::c_char,
                answer: 0.5f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sin asin .5\0" as *const u8 as *const libc::c_char,
                answer: 0.5f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"ln exp .5\0" as *const u8 as *const libc::c_char,
                answer: 0.5f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"exp ln .5\0" as *const u8 as *const libc::c_char,
                answer: 0.5f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"asin sin-.5\0" as *const u8 as *const libc::c_char,
                answer: -0.5f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"asin sin-0.5\0" as *const u8 as *const libc::c_char,
                answer: -0.5f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"asin sin -0.5\0" as *const u8 as *const libc::c_char,
                answer: -0.5f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"asin (sin -0.5)\0" as *const u8 as *const libc::c_char,
                answer: -0.5f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"asin (sin (-0.5))\0" as *const u8 as *const libc::c_char,
                answer: -0.5f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"asin sin (-0.5)\0" as *const u8 as *const libc::c_char,
                answer: -0.5f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(asin sin (-0.5))\0" as *const u8 as *const libc::c_char,
                answer: -0.5f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"log10 1000\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"log10 1e3\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"log10 1000\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"log10 1e3\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"log10(1000)\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"log10(1e3)\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"log10 1.0e3\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"10^5*5e-5\0" as *const u8 as *const libc::c_char,
                answer: 5 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"log 1000\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"ln (e^10)\0" as *const u8 as *const libc::c_char,
                answer: 10 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"100^.5+1\0" as *const u8 as *const libc::c_char,
                answer: 11 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"100 ^.5+1\0" as *const u8 as *const libc::c_char,
                answer: 11 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"100^+.5+1\0" as *const u8 as *const libc::c_char,
                answer: 11 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"100^--.5+1\0" as *const u8 as *const libc::c_char,
                answer: 11 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"100^---+-++---++-+-+-.5+1\0" as *const u8 as *const libc::c_char,
                answer: 11 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"100^-.5+1\0" as *const u8 as *const libc::c_char,
                answer: 1.1f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"100^---.5+1\0" as *const u8 as *const libc::c_char,
                answer: 1.1f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"100^+---.5+1\0" as *const u8 as *const libc::c_char,
                answer: 1.1f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1e2^+---.5e0+1e0\0" as *const u8 as *const libc::c_char,
                answer: 1.1f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"--(1e2^(+(-(-(-.5e0))))+1e0)\0" as *const u8
                    as *const libc::c_char,
                answer: 1.1f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sqrt 100 + 7\0" as *const u8 as *const libc::c_char,
                answer: 17 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sqrt 100 * 7\0" as *const u8 as *const libc::c_char,
                answer: 70 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sqrt (100 * 100)\0" as *const u8 as *const libc::c_char,
                answer: 100 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1,2\0" as *const u8 as *const libc::c_char,
                answer: 2 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1,2+1\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1+1,2+2,2+1\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1,2,3\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(1,2),3\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1,(2,3)\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"-(1,(2,3))\0" as *const u8 as *const libc::c_char,
                answer: -(3 as libc::c_int) as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2^2\0" as *const u8 as *const libc::c_char,
                answer: 4 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"pow(2,2)\0" as *const u8 as *const libc::c_char,
                answer: 4 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"atan2(1,1)\0" as *const u8 as *const libc::c_char,
                answer: 0.7854f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"atan2(1,2)\0" as *const u8 as *const libc::c_char,
                answer: 0.4636f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"atan2(2,1)\0" as *const u8 as *const libc::c_char,
                answer: 1.1071f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"atan2(3,4)\0" as *const u8 as *const libc::c_char,
                answer: 0.6435f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"atan2(3+3,4*2)\0" as *const u8 as *const libc::c_char,
                answer: 0.6435f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"atan2(3+3,(4*2))\0" as *const u8 as *const libc::c_char,
                answer: 0.6435f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"atan2((3+3),4*2)\0" as *const u8 as *const libc::c_char,
                answer: 0.6435f64,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"atan2((3+3),(4*2))\0" as *const u8 as *const libc::c_char,
                answer: 0.6435f64,
            };
            init
        },
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[test_case; 79]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<test_case>() as libc::c_ulong)
    {
        let mut expr: *const libc::c_char = cases[i as usize].expr;
        let answer: libc::c_double = cases[i as usize].answer;
        let mut err: libc::c_int = 0;
        let ev: libc::c_double = te_interp(expr, &mut err);
        ltests += 1;
        ltests;
        if err != 0 {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                161 as libc::c_int,
            );
        }
        ltests += 1;
        ltests;
        let __LF_COMPARE: libc::c_double = fabs(ev - answer);
        if __LF_COMPARE > 0.001f64 || __LF_COMPARE != __LF_COMPARE {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                162 as libc::c_int,
                ev,
                answer,
            );
        }
        if err != 0 {
            printf(
                b"FAILED: %s (%d)\n\0" as *const u8 as *const libc::c_char,
                expr,
                err,
            );
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_syntax() {
    let mut errors: [test_case; 13] = [
        {
            let mut init = test_case {
                expr: b"\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1+\0" as *const u8 as *const libc::c_char,
                answer: 2 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1)\0" as *const u8 as *const libc::c_char,
                answer: 2 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(1\0" as *const u8 as *const libc::c_char,
                answer: 2 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1**1\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1*2(+4\0" as *const u8 as *const libc::c_char,
                answer: 4 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1*2(1+4\0" as *const u8 as *const libc::c_char,
                answer: 4 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"a+5\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"_a+5\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"#a+5\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1^^5\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1**5\0" as *const u8 as *const libc::c_char,
                answer: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sin(cos5\0" as *const u8 as *const libc::c_char,
                answer: 8 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[test_case; 13]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<test_case>() as libc::c_ulong)
    {
        let mut expr: *const libc::c_char = errors[i as usize].expr;
        let e: libc::c_int = errors[i as usize].answer as libc::c_int;
        let mut err: libc::c_int = 0;
        let r: libc::c_double = te_interp(expr, &mut err);
        ltests += 1;
        ltests;
        if err != e {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                196 as libc::c_int,
                err,
                e,
            );
        }
        ltests += 1;
        ltests;
        if !(r != r) {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                197 as libc::c_int,
            );
        }
        let mut n: *mut te_expr = te_compile(
            expr,
            0 as *const te_variable,
            0 as libc::c_int,
            &mut err,
        );
        ltests += 1;
        ltests;
        if err != e {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                200 as libc::c_int,
                err,
                e,
            );
        }
        ltests += 1;
        ltests;
        if !n.is_null() {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                201 as libc::c_int,
            );
        }
        if err != e {
            printf(b"FAILED: %s\n\0" as *const u8 as *const libc::c_char, expr);
        }
        let k: libc::c_double = te_interp(expr, 0 as *mut libc::c_int);
        ltests += 1;
        ltests;
        if !(k != k) {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                208 as libc::c_int,
            );
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_nans() {
    let mut nans: [*const libc::c_char; 11] = [
        b"0/0\0" as *const u8 as *const libc::c_char,
        b"1%0\0" as *const u8 as *const libc::c_char,
        b"1%(1%0)\0" as *const u8 as *const libc::c_char,
        b"(1%0)%1\0" as *const u8 as *const libc::c_char,
        b"fac(-1)\0" as *const u8 as *const libc::c_char,
        b"ncr(2, 4)\0" as *const u8 as *const libc::c_char,
        b"ncr(-2, 4)\0" as *const u8 as *const libc::c_char,
        b"ncr(2, -4)\0" as *const u8 as *const libc::c_char,
        b"npr(2, 4)\0" as *const u8 as *const libc::c_char,
        b"npr(-2, 4)\0" as *const u8 as *const libc::c_char,
        b"npr(2, -4)\0" as *const u8 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[*const libc::c_char; 11]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        let mut expr: *const libc::c_char = nans[i as usize];
        let mut err: libc::c_int = 0;
        let r: libc::c_double = te_interp(expr, &mut err);
        ltests += 1;
        ltests;
        if err != 0 as libc::c_int {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                235 as libc::c_int,
                err,
                0 as libc::c_int,
            );
        }
        ltests += 1;
        ltests;
        if !(r != r) {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                236 as libc::c_int,
            );
        }
        let mut n: *mut te_expr = te_compile(
            expr,
            0 as *const te_variable,
            0 as libc::c_int,
            &mut err,
        );
        ltests += 1;
        ltests;
        if n.is_null() {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
            );
        }
        ltests += 1;
        ltests;
        if err != 0 as libc::c_int {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                240 as libc::c_int,
                err,
                0 as libc::c_int,
            );
        }
        let c: libc::c_double = te_eval(n);
        ltests += 1;
        ltests;
        if !(c != c) {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                242 as libc::c_int,
            );
        }
        te_free(n);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_infs() {
    let mut infs: [*const libc::c_char; 10] = [
        b"1/0\0" as *const u8 as *const libc::c_char,
        b"log(0)\0" as *const u8 as *const libc::c_char,
        b"pow(2,10000000)\0" as *const u8 as *const libc::c_char,
        b"fac(300)\0" as *const u8 as *const libc::c_char,
        b"ncr(300,100)\0" as *const u8 as *const libc::c_char,
        b"ncr(300000,100)\0" as *const u8 as *const libc::c_char,
        b"ncr(300000,100)*8\0" as *const u8 as *const libc::c_char,
        b"npr(3,2)*ncr(300000,100)\0" as *const u8 as *const libc::c_char,
        b"npr(100,90)\0" as *const u8 as *const libc::c_char,
        b"npr(30,25)\0" as *const u8 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[*const libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        let mut expr: *const libc::c_char = infs[i as usize];
        let mut err: libc::c_int = 0;
        let r: libc::c_double = te_interp(expr, &mut err);
        ltests += 1;
        ltests;
        if err != 0 as libc::c_int {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                269 as libc::c_int,
                err,
                0 as libc::c_int,
            );
        }
        ltests += 1;
        ltests;
        if !(r == r + 1 as libc::c_int as libc::c_double) {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                270 as libc::c_int,
            );
        }
        let mut n: *mut te_expr = te_compile(
            expr,
            0 as *const te_variable,
            0 as libc::c_int,
            &mut err,
        );
        ltests += 1;
        ltests;
        if n.is_null() {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                273 as libc::c_int,
            );
        }
        ltests += 1;
        ltests;
        if err != 0 as libc::c_int {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                274 as libc::c_int,
                err,
                0 as libc::c_int,
            );
        }
        let c: libc::c_double = te_eval(n);
        ltests += 1;
        ltests;
        if !(c == c + 1 as libc::c_int as libc::c_double) {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                276 as libc::c_int,
            );
        }
        te_free(n);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_variables() {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut test: libc::c_double = 0.;
    let mut lookup: [te_variable; 3] = [
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
        {
            let mut init = te_variable {
                name: b"te_st\0" as *const u8 as *const libc::c_char,
                address: &mut test as *mut libc::c_double as *const libc::c_void,
                type_0: 0,
                context: 0 as *mut libc::c_void,
            };
            init
        },
    ];
    let mut err: libc::c_int = 0;
    let mut expr1: *mut te_expr = te_compile(
        b"cos x + sin y\0" as *const u8 as *const libc::c_char,
        lookup.as_mut_ptr(),
        2 as libc::c_int,
        &mut err,
    );
    ltests += 1;
    ltests;
    if expr1.is_null() {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            290 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if err != 0 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            291 as libc::c_int,
        );
    }
    let mut expr2: *mut te_expr = te_compile(
        b"x+x+x-y\0" as *const u8 as *const libc::c_char,
        lookup.as_mut_ptr(),
        2 as libc::c_int,
        &mut err,
    );
    ltests += 1;
    ltests;
    if expr2.is_null() {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if err != 0 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            295 as libc::c_int,
        );
    }
    let mut expr3: *mut te_expr = te_compile(
        b"x*y^3\0" as *const u8 as *const libc::c_char,
        lookup.as_mut_ptr(),
        2 as libc::c_int,
        &mut err,
    );
    ltests += 1;
    ltests;
    if expr3.is_null() {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            298 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if err != 0 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            299 as libc::c_int,
        );
    }
    let mut expr4: *mut te_expr = te_compile(
        b"te_st+5\0" as *const u8 as *const libc::c_char,
        lookup.as_mut_ptr(),
        3 as libc::c_int,
        &mut err,
    );
    ltests += 1;
    ltests;
    if expr4.is_null() {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            302 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if err != 0 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            303 as libc::c_int,
        );
    }
    y = 2 as libc::c_int as libc::c_double;
    while y < 3 as libc::c_int as libc::c_double {
        x = 0 as libc::c_int as libc::c_double;
        while x < 5 as libc::c_int as libc::c_double {
            let mut ev: libc::c_double = 0.;
            ev = te_eval(expr1);
            ltests += 1;
            ltests;
            let __LF_COMPARE: libc::c_double = fabs(ev - (cos(x) + sin(y)));
            if __LF_COMPARE > 0.001f64 || __LF_COMPARE != __LF_COMPARE {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    310 as libc::c_int,
                    ev,
                    cos(x) + sin(y),
                );
            }
            ev = te_eval(expr2);
            ltests += 1;
            ltests;
            let __LF_COMPARE_0: libc::c_double = fabs(ev - (x + x + x - y));
            if __LF_COMPARE_0 > 0.001f64 || __LF_COMPARE_0 != __LF_COMPARE_0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    313 as libc::c_int,
                    ev,
                    x + x + x - y,
                );
            }
            ev = te_eval(expr3);
            ltests += 1;
            ltests;
            let __LF_COMPARE_1: libc::c_double = fabs(ev - x * y * y * y);
            if __LF_COMPARE_1 > 0.001f64 || __LF_COMPARE_1 != __LF_COMPARE_1 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    316 as libc::c_int,
                    ev,
                    x * y * y * y,
                );
            }
            test = x;
            ev = te_eval(expr4);
            ltests += 1;
            ltests;
            let __LF_COMPARE_2: libc::c_double = fabs(
                ev - (x + 5 as libc::c_int as libc::c_double),
            );
            if __LF_COMPARE_2 > 0.001f64 || __LF_COMPARE_2 != __LF_COMPARE_2 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    320 as libc::c_int,
                    ev,
                    x + 5 as libc::c_int as libc::c_double,
                );
            }
            x += 1.;
            x;
        }
        y += 1.;
        y;
    }
    te_free(expr1);
    te_free(expr2);
    te_free(expr3);
    te_free(expr4);
    let mut expr5: *mut te_expr = te_compile(
        b"xx*y^3\0" as *const u8 as *const libc::c_char,
        lookup.as_mut_ptr(),
        2 as libc::c_int,
        &mut err,
    );
    ltests += 1;
    ltests;
    if !expr5.is_null() {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            332 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if err == 0 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            333 as libc::c_int,
        );
    }
    let mut expr6: *mut te_expr = te_compile(
        b"tes\0" as *const u8 as *const libc::c_char,
        lookup.as_mut_ptr(),
        3 as libc::c_int,
        &mut err,
    );
    ltests += 1;
    ltests;
    if !expr6.is_null() {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            336 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if err == 0 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            337 as libc::c_int,
        );
    }
    let mut expr7: *mut te_expr = te_compile(
        b"sinn x\0" as *const u8 as *const libc::c_char,
        lookup.as_mut_ptr(),
        2 as libc::c_int,
        &mut err,
    );
    ltests += 1;
    ltests;
    if !expr7.is_null() {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            340 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if err == 0 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            341 as libc::c_int,
        );
    }
    let mut expr8: *mut te_expr = te_compile(
        b"si x\0" as *const u8 as *const libc::c_char,
        lookup.as_mut_ptr(),
        2 as libc::c_int,
        &mut err,
    );
    ltests += 1;
    ltests;
    if !expr8.is_null() {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            344 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if err == 0 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            345 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_functions() {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut lookup: [te_variable; 2] = [
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
    let mut expr: *mut te_expr = 0 as *mut te_expr;
    x = -(5 as libc::c_int) as libc::c_double;
    while x < 5 as libc::c_int as libc::c_double {
        if !(fabs(x) != fabs(x)) {
            expr = te_compile(
                b"abs x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE: libc::c_double = fabs(te_eval(expr) - fabs(x));
            if __LF_COMPARE > 0.001f64 || __LF_COMPARE != __LF_COMPARE {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    367 as libc::c_int,
                    te_eval(expr),
                    fabs(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    367 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(acos(x) != acos(x)) {
            expr = te_compile(
                b"acos x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_0: libc::c_double = fabs(te_eval(expr) - acos(x));
            if __LF_COMPARE_0 > 0.001f64 || __LF_COMPARE_0 != __LF_COMPARE_0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    368 as libc::c_int,
                    te_eval(expr),
                    acos(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    368 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(asin(x) != asin(x)) {
            expr = te_compile(
                b"asin x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_1: libc::c_double = fabs(te_eval(expr) - asin(x));
            if __LF_COMPARE_1 > 0.001f64 || __LF_COMPARE_1 != __LF_COMPARE_1 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    369 as libc::c_int,
                    te_eval(expr),
                    asin(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    369 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(atan(x) != atan(x)) {
            expr = te_compile(
                b"atan x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_2: libc::c_double = fabs(te_eval(expr) - atan(x));
            if __LF_COMPARE_2 > 0.001f64 || __LF_COMPARE_2 != __LF_COMPARE_2 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    370 as libc::c_int,
                    te_eval(expr),
                    atan(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    370 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(ceil(x) != ceil(x)) {
            expr = te_compile(
                b"ceil x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_3: libc::c_double = fabs(te_eval(expr) - ceil(x));
            if __LF_COMPARE_3 > 0.001f64 || __LF_COMPARE_3 != __LF_COMPARE_3 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    371 as libc::c_int,
                    te_eval(expr),
                    ceil(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    371 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(cos(x) != cos(x)) {
            expr = te_compile(
                b"cos x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_4: libc::c_double = fabs(te_eval(expr) - cos(x));
            if __LF_COMPARE_4 > 0.001f64 || __LF_COMPARE_4 != __LF_COMPARE_4 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    372 as libc::c_int,
                    te_eval(expr),
                    cos(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    372 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(cosh(x) != cosh(x)) {
            expr = te_compile(
                b"cosh x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_5: libc::c_double = fabs(te_eval(expr) - cosh(x));
            if __LF_COMPARE_5 > 0.001f64 || __LF_COMPARE_5 != __LF_COMPARE_5 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    373 as libc::c_int,
                    te_eval(expr),
                    cosh(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    373 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(exp(x) != exp(x)) {
            expr = te_compile(
                b"exp x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_6: libc::c_double = fabs(te_eval(expr) - exp(x));
            if __LF_COMPARE_6 > 0.001f64 || __LF_COMPARE_6 != __LF_COMPARE_6 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    374 as libc::c_int,
                    te_eval(expr),
                    exp(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    374 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(floor(x) != floor(x)) {
            expr = te_compile(
                b"floor x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_7: libc::c_double = fabs(te_eval(expr) - floor(x));
            if __LF_COMPARE_7 > 0.001f64 || __LF_COMPARE_7 != __LF_COMPARE_7 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    375 as libc::c_int,
                    te_eval(expr),
                    floor(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    375 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(log(x) != log(x)) {
            expr = te_compile(
                b"ln x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_8: libc::c_double = fabs(te_eval(expr) - log(x));
            if __LF_COMPARE_8 > 0.001f64 || __LF_COMPARE_8 != __LF_COMPARE_8 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    376 as libc::c_int,
                    te_eval(expr),
                    log(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    376 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(log10(x) != log10(x)) {
            expr = te_compile(
                b"log10 x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_9: libc::c_double = fabs(te_eval(expr) - log10(x));
            if __LF_COMPARE_9 > 0.001f64 || __LF_COMPARE_9 != __LF_COMPARE_9 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    377 as libc::c_int,
                    te_eval(expr),
                    log10(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    377 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(sin(x) != sin(x)) {
            expr = te_compile(
                b"sin x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_10: libc::c_double = fabs(te_eval(expr) - sin(x));
            if __LF_COMPARE_10 > 0.001f64 || __LF_COMPARE_10 != __LF_COMPARE_10 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    378 as libc::c_int,
                    te_eval(expr),
                    sin(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    378 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(sinh(x) != sinh(x)) {
            expr = te_compile(
                b"sinh x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_11: libc::c_double = fabs(te_eval(expr) - sinh(x));
            if __LF_COMPARE_11 > 0.001f64 || __LF_COMPARE_11 != __LF_COMPARE_11 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    379 as libc::c_int,
                    te_eval(expr),
                    sinh(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    379 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(sqrt(x) != sqrt(x)) {
            expr = te_compile(
                b"sqrt x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_12: libc::c_double = fabs(te_eval(expr) - sqrt(x));
            if __LF_COMPARE_12 > 0.001f64 || __LF_COMPARE_12 != __LF_COMPARE_12 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    380 as libc::c_int,
                    te_eval(expr),
                    sqrt(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    380 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(tan(x) != tan(x)) {
            expr = te_compile(
                b"tan x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_13: libc::c_double = fabs(te_eval(expr) - tan(x));
            if __LF_COMPARE_13 > 0.001f64 || __LF_COMPARE_13 != __LF_COMPARE_13 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    381 as libc::c_int,
                    te_eval(expr),
                    tan(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    381 as libc::c_int,
                );
            }
            te_free(expr);
        }
        if !(tanh(x) != tanh(x)) {
            expr = te_compile(
                b"tanh x\0" as *const u8 as *const libc::c_char,
                lookup.as_mut_ptr(),
                2 as libc::c_int,
                &mut err,
            );
            ltests += 1;
            ltests;
            let __LF_COMPARE_14: libc::c_double = fabs(te_eval(expr) - tanh(x));
            if __LF_COMPARE_14 > 0.001f64 || __LF_COMPARE_14 != __LF_COMPARE_14 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    382 as libc::c_int,
                    te_eval(expr),
                    tanh(x),
                );
            }
            ltests += 1;
            ltests;
            if err != 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                    382 as libc::c_int,
                );
            }
            te_free(expr);
        }
        y = -(2 as libc::c_int) as libc::c_double;
        while y < 2 as libc::c_int as libc::c_double {
            if fabs(x) < 0.01f64 {
                break;
            }
            if !(atan2(x, y) != atan2(x, y)) {
                expr = te_compile(
                    b"atan2(x,y)\0" as *const u8 as *const libc::c_char,
                    lookup.as_mut_ptr(),
                    2 as libc::c_int,
                    &mut err,
                );
                ltests += 1;
                ltests;
                let __LF_COMPARE_15: libc::c_double = fabs(te_eval(expr) - atan2(x, y));
                if __LF_COMPARE_15 > 0.001f64 || __LF_COMPARE_15 != __LF_COMPARE_15 {
                    lfails += 1;
                    lfails;
                    printf(
                        b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                        b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                        386 as libc::c_int,
                        te_eval(expr),
                        atan2(x, y),
                    );
                }
                ltests += 1;
                ltests;
                if err != 0 {
                    lfails += 1;
                    lfails;
                    printf(
                        b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                        b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                        386 as libc::c_int,
                    );
                }
                te_free(expr);
            }
            if !(pow(x, y) != pow(x, y)) {
                expr = te_compile(
                    b"pow(x,y)\0" as *const u8 as *const libc::c_char,
                    lookup.as_mut_ptr(),
                    2 as libc::c_int,
                    &mut err,
                );
                ltests += 1;
                ltests;
                let __LF_COMPARE_16: libc::c_double = fabs(te_eval(expr) - pow(x, y));
                if __LF_COMPARE_16 > 0.001f64 || __LF_COMPARE_16 != __LF_COMPARE_16 {
                    lfails += 1;
                    lfails;
                    printf(
                        b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                        b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                        387 as libc::c_int,
                        te_eval(expr),
                        pow(x, y),
                    );
                }
                ltests += 1;
                ltests;
                if err != 0 {
                    lfails += 1;
                    lfails;
                    printf(
                        b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                        b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                        387 as libc::c_int,
                    );
                }
                te_free(expr);
            }
            y += 0.2f64;
        }
        x += 0.2f64;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sum0() -> libc::c_double {
    return 6 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn sum1(mut a: libc::c_double) -> libc::c_double {
    return a * 2 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn sum2(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return a + b;
}
#[no_mangle]
pub unsafe extern "C" fn sum3(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
) -> libc::c_double {
    return a + b + c;
}
#[no_mangle]
pub unsafe extern "C" fn sum4(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
    mut d: libc::c_double,
) -> libc::c_double {
    return a + b + c + d;
}
#[no_mangle]
pub unsafe extern "C" fn sum5(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
    mut d: libc::c_double,
    mut e: libc::c_double,
) -> libc::c_double {
    return a + b + c + d + e;
}
#[no_mangle]
pub unsafe extern "C" fn sum6(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
    mut d: libc::c_double,
    mut e: libc::c_double,
    mut f: libc::c_double,
) -> libc::c_double {
    return a + b + c + d + e + f;
}
#[no_mangle]
pub unsafe extern "C" fn sum7(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
    mut d: libc::c_double,
    mut e: libc::c_double,
    mut f: libc::c_double,
    mut g: libc::c_double,
) -> libc::c_double {
    return a + b + c + d + e + f + g;
}
#[no_mangle]
pub unsafe extern "C" fn test_dynamic() {
    let mut x: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    let mut lookup: [te_variable; 10] = [
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
                name: b"f\0" as *const u8 as *const libc::c_char,
                address: &mut f as *mut libc::c_double as *const libc::c_void,
                type_0: 0,
                context: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"sum0\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> libc::c_double>,
                    *const libc::c_void,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> libc::c_double,
                            unsafe extern "C" fn() -> libc::c_double,
                        >(sum0),
                    ),
                ),
                type_0: TE_FUNCTION0 as libc::c_int,
                context: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"sum1\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    *const libc::c_void,
                >(Some(sum1 as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
                type_0: TE_FUNCTION1 as libc::c_int,
                context: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"sum2\0" as *const u8 as *const libc::c_char,
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
                        sum2
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
        {
            let mut init = te_variable {
                name: b"sum3\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        sum3
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ),
                type_0: TE_FUNCTION3 as libc::c_int,
                context: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"sum4\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        sum4
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ),
                type_0: TE_FUNCTION4 as libc::c_int,
                context: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"sum5\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                            libc::c_double,
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        sum5
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ),
                type_0: TE_FUNCTION5 as libc::c_int,
                context: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"sum6\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
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
                    *const libc::c_void,
                >(
                    Some(
                        sum6
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ),
                type_0: TE_FUNCTION6 as libc::c_int,
                context: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"sum7\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
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
                    *const libc::c_void,
                >(
                    Some(
                        sum7
                            as unsafe extern "C" fn(
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ),
                type_0: TE_FUNCTION7 as libc::c_int,
                context: 0 as *mut libc::c_void,
            };
            init
        },
    ];
    let mut cases: [test_case; 22] = [
        {
            let mut init = test_case {
                expr: b"x\0" as *const u8 as *const libc::c_char,
                answer: 2 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"f+x\0" as *const u8 as *const libc::c_char,
                answer: 7 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"x+x\0" as *const u8 as *const libc::c_char,
                answer: 4 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"x+f\0" as *const u8 as *const libc::c_char,
                answer: 7 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"f+f\0" as *const u8 as *const libc::c_char,
                answer: 10 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"f+sum0\0" as *const u8 as *const libc::c_char,
                answer: 11 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum0+sum0\0" as *const u8 as *const libc::c_char,
                answer: 12 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum0()+sum0\0" as *const u8 as *const libc::c_char,
                answer: 12 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum0+sum0()\0" as *const u8 as *const libc::c_char,
                answer: 12 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum0()+(0)+sum0()\0" as *const u8 as *const libc::c_char,
                answer: 12 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum1 sum0\0" as *const u8 as *const libc::c_char,
                answer: 12 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum1(sum0)\0" as *const u8 as *const libc::c_char,
                answer: 12 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum1 f\0" as *const u8 as *const libc::c_char,
                answer: 10 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum1 x\0" as *const u8 as *const libc::c_char,
                answer: 4 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum2 (sum0, x)\0" as *const u8 as *const libc::c_char,
                answer: 8 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum3 (sum0, x, 2)\0" as *const u8 as *const libc::c_char,
                answer: 10 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum2(2,3)\0" as *const u8 as *const libc::c_char,
                answer: 5 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum3(2,3,4)\0" as *const u8 as *const libc::c_char,
                answer: 9 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum4(2,3,4,5)\0" as *const u8 as *const libc::c_char,
                answer: 14 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum5(2,3,4,5,6)\0" as *const u8 as *const libc::c_char,
                answer: 20 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum6(2,3,4,5,6,7)\0" as *const u8 as *const libc::c_char,
                answer: 27 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sum7(2,3,4,5,6,7,8)\0" as *const u8 as *const libc::c_char,
                answer: 35 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    x = 2 as libc::c_int as libc::c_double;
    f = 5 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[test_case; 22]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<test_case>() as libc::c_ulong)
    {
        let mut expr: *const libc::c_char = cases[i as usize].expr;
        let answer: libc::c_double = cases[i as usize].answer;
        let mut err: libc::c_int = 0;
        let mut ex: *mut te_expr = te_compile(
            expr,
            lookup.as_mut_ptr(),
            (::core::mem::size_of::<[te_variable; 10]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<te_variable>() as libc::c_ulong)
                as libc::c_int,
            &mut err,
        );
        ltests += 1;
        ltests;
        if ex.is_null() {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                470 as libc::c_int,
            );
        }
        ltests += 1;
        ltests;
        let __LF_COMPARE: libc::c_double = fabs(te_eval(ex) - answer);
        if __LF_COMPARE > 0.001f64 || __LF_COMPARE != __LF_COMPARE {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                471 as libc::c_int,
                te_eval(ex),
                answer,
            );
        }
        te_free(ex);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn clo0(mut context: *mut libc::c_void) -> libc::c_double {
    if !context.is_null() {
        return *(context as *mut libc::c_double) + 6 as libc::c_int as libc::c_double;
    }
    return 6 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn clo1(
    mut context: *mut libc::c_void,
    mut a: libc::c_double,
) -> libc::c_double {
    if !context.is_null() {
        return *(context as *mut libc::c_double)
            + a * 2 as libc::c_int as libc::c_double;
    }
    return a * 2 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn clo2(
    mut context: *mut libc::c_void,
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    if !context.is_null() {
        return *(context as *mut libc::c_double) + a + b;
    }
    return a + b;
}
#[no_mangle]
pub unsafe extern "C" fn cell(
    mut context: *mut libc::c_void,
    mut a: libc::c_double,
) -> libc::c_double {
    let mut c: *mut libc::c_double = context as *mut libc::c_double;
    return *c.offset(a as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn test_closure() {
    let mut extra: libc::c_double = 0.;
    let mut c: [libc::c_double; 5] = [
        5 as libc::c_int as libc::c_double,
        6 as libc::c_int as libc::c_double,
        7 as libc::c_int as libc::c_double,
        8 as libc::c_int as libc::c_double,
        9 as libc::c_int as libc::c_double,
    ];
    let mut lookup: [te_variable; 4] = [
        {
            let mut init = te_variable {
                name: b"c0\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
                    *const libc::c_void,
                >(
                    Some(
                        clo0 as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
                    ),
                ),
                type_0: TE_CLOSURE0 as libc::c_int,
                context: &mut extra as *mut libc::c_double as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"c1\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        clo1
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ),
                type_0: TE_CLOSURE1 as libc::c_int,
                context: &mut extra as *mut libc::c_double as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"c2\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        clo2
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_double,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ),
                type_0: TE_CLOSURE2 as libc::c_int,
                context: &mut extra as *mut libc::c_double as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"cell\0" as *const u8 as *const libc::c_char,
                address: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            libc::c_double,
                        ) -> libc::c_double,
                    >,
                    *const libc::c_void,
                >(
                    Some(
                        cell
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_double,
                            ) -> libc::c_double,
                    ),
                ),
                type_0: TE_CLOSURE1 as libc::c_int,
                context: c.as_mut_ptr() as *mut libc::c_void,
            };
            init
        },
    ];
    let mut cases: [test_case; 3] = [
        {
            let mut init = test_case {
                expr: b"c0\0" as *const u8 as *const libc::c_char,
                answer: 6 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"c1 4\0" as *const u8 as *const libc::c_char,
                answer: 8 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"c2 (10, 20)\0" as *const u8 as *const libc::c_char,
                answer: 30 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[test_case; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<test_case>() as libc::c_ulong)
    {
        let mut expr: *const libc::c_char = cases[i as usize].expr;
        let answer: libc::c_double = cases[i as usize].answer;
        let mut err: libc::c_int = 0;
        let mut ex: *mut te_expr = te_compile(
            expr,
            lookup.as_mut_ptr(),
            (::core::mem::size_of::<[te_variable; 4]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<te_variable>() as libc::c_ulong)
                as libc::c_int,
            &mut err,
        );
        ltests += 1;
        ltests;
        if ex.is_null() {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                520 as libc::c_int,
            );
        }
        extra = 0 as libc::c_int as libc::c_double;
        ltests += 1;
        ltests;
        let __LF_COMPARE: libc::c_double = fabs(te_eval(ex) - (answer + extra));
        if __LF_COMPARE > 0.001f64 || __LF_COMPARE != __LF_COMPARE {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                523 as libc::c_int,
                te_eval(ex),
                answer + extra,
            );
        }
        extra = 10 as libc::c_int as libc::c_double;
        ltests += 1;
        ltests;
        let __LF_COMPARE_0: libc::c_double = fabs(te_eval(ex) - (answer + extra));
        if __LF_COMPARE_0 > 0.001f64 || __LF_COMPARE_0 != __LF_COMPARE_0 {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                526 as libc::c_int,
                te_eval(ex),
                answer + extra,
            );
        }
        te_free(ex);
        i += 1;
        i;
    }
    let mut cases2: [test_case; 4] = [
        {
            let mut init = test_case {
                expr: b"cell 0\0" as *const u8 as *const libc::c_char,
                answer: 5 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"cell 1\0" as *const u8 as *const libc::c_char,
                answer: 6 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"cell 0 + cell 1\0" as *const u8 as *const libc::c_char,
                answer: 11 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"cell 1 * cell 3 + cell 4\0" as *const u8 as *const libc::c_char,
                answer: 57 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[test_case; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<test_case>() as libc::c_ulong)
    {
        let mut expr_0: *const libc::c_char = cases2[i as usize].expr;
        let answer_0: libc::c_double = cases2[i as usize].answer;
        let mut err_0: libc::c_int = 0;
        let mut ex_0: *mut te_expr = te_compile(
            expr_0,
            lookup.as_mut_ptr(),
            (::core::mem::size_of::<[te_variable; 4]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<te_variable>() as libc::c_ulong)
                as libc::c_int,
            &mut err_0,
        );
        ltests += 1;
        ltests;
        if ex_0.is_null() {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                545 as libc::c_int,
            );
        }
        ltests += 1;
        ltests;
        let __LF_COMPARE_1: libc::c_double = fabs(te_eval(ex_0) - answer_0);
        if __LF_COMPARE_1 > 0.001f64 || __LF_COMPARE_1 != __LF_COMPARE_1 {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                546 as libc::c_int,
                te_eval(ex_0),
                answer_0,
            );
        }
        te_free(ex_0);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_optimize() {
    let mut cases: [test_case; 4] = [
        {
            let mut init = test_case {
                expr: b"5+5\0" as *const u8 as *const libc::c_char,
                answer: 10 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"pow(2,2)\0" as *const u8 as *const libc::c_char,
                answer: 4 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"sqrt 100\0" as *const u8 as *const libc::c_char,
                answer: 10 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"pi * 2\0" as *const u8 as *const libc::c_char,
                answer: 6.2832f64,
            };
            init
        },
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[test_case; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<test_case>() as libc::c_ulong)
    {
        let mut expr: *const libc::c_char = cases[i as usize].expr;
        let answer: libc::c_double = cases[i as usize].answer;
        let mut err: libc::c_int = 0;
        let mut ex: *mut te_expr = te_compile(
            expr,
            0 as *const te_variable,
            0 as libc::c_int,
            &mut err,
        );
        ltests += 1;
        ltests;
        if ex.is_null() {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                567 as libc::c_int,
            );
        }
        ltests += 1;
        ltests;
        let __LF_COMPARE: libc::c_double = fabs((*ex).c2rust_unnamed.value - answer);
        if __LF_COMPARE > 0.001f64 || __LF_COMPARE != __LF_COMPARE {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                571 as libc::c_int,
                (*ex).c2rust_unnamed.value,
                answer,
            );
        }
        ltests += 1;
        ltests;
        let __LF_COMPARE_0: libc::c_double = fabs(te_eval(ex) - answer);
        if __LF_COMPARE_0 > 0.001f64 || __LF_COMPARE_0 != __LF_COMPARE_0 {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                572 as libc::c_int,
                te_eval(ex),
                answer,
            );
        }
        te_free(ex);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_pow() {
    let mut cases: [test_equ; 17] = [
        {
            let mut init = test_equ {
                expr1: b"2^3^4\0" as *const u8 as *const libc::c_char,
                expr2: b"(2^3)^4\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"-2^2\0" as *const u8 as *const libc::c_char,
                expr2: b"(-2)^2\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"(-2)^2\0" as *const u8 as *const libc::c_char,
                expr2: b"4\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"--2^2\0" as *const u8 as *const libc::c_char,
                expr2: b"2^2\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"---2^2\0" as *const u8 as *const libc::c_char,
                expr2: b"(-2)^2\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"-2^2\0" as *const u8 as *const libc::c_char,
                expr2: b"4\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"2^1.1^1.2^1.3\0" as *const u8 as *const libc::c_char,
                expr2: b"((2^1.1)^1.2)^1.3\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"-a^b\0" as *const u8 as *const libc::c_char,
                expr2: b"(-a)^b\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"-a^-b\0" as *const u8 as *const libc::c_char,
                expr2: b"(-a)^(-b)\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"1^0\0" as *const u8 as *const libc::c_char,
                expr2: b"1\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"(1)^0\0" as *const u8 as *const libc::c_char,
                expr2: b"1\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"(-1)^0\0" as *const u8 as *const libc::c_char,
                expr2: b"1\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"(-5)^0\0" as *const u8 as *const libc::c_char,
                expr2: b"1\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"(!0)^0\0" as *const u8 as *const libc::c_char,
                expr2: b"1\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"(!!5)^0\0" as *const u8 as *const libc::c_char,
                expr2: b"1\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"2^-3^4\0" as *const u8 as *const libc::c_char,
                expr2: b"(2^(-3))^4\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = test_equ {
                expr1: b"-2^-3^-4\0" as *const u8 as *const libc::c_char,
                expr2: b"((-2)^(-3))^(-4)\0" as *const u8 as *const libc::c_char,
            };
            init
        },
    ];
    let mut a: libc::c_double = 2 as libc::c_int as libc::c_double;
    let mut b: libc::c_double = 3 as libc::c_int as libc::c_double;
    let mut lookup: [te_variable; 2] = [
        {
            let mut init = te_variable {
                name: b"a\0" as *const u8 as *const libc::c_char,
                address: &mut a as *mut libc::c_double as *const libc::c_void,
                type_0: 0,
                context: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = te_variable {
                name: b"b\0" as *const u8 as *const libc::c_char,
                address: &mut b as *mut libc::c_double as *const libc::c_void,
                type_0: 0,
                context: 0 as *mut libc::c_void,
            };
            init
        },
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[test_equ; 17]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<test_equ>() as libc::c_ulong)
    {
        let mut expr1: *const libc::c_char = cases[i as usize].expr1;
        let mut expr2: *const libc::c_char = cases[i as usize].expr2;
        let mut ex1: *mut te_expr = te_compile(
            expr1,
            lookup.as_mut_ptr(),
            (::core::mem::size_of::<[te_variable; 2]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<te_variable>() as libc::c_ulong)
                as libc::c_int,
            0 as *mut libc::c_int,
        );
        let mut ex2: *mut te_expr = te_compile(
            expr2,
            lookup.as_mut_ptr(),
            (::core::mem::size_of::<[te_variable; 2]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<te_variable>() as libc::c_ulong)
                as libc::c_int,
            0 as *mut libc::c_int,
        );
        ltests += 1;
        ltests;
        if ex1.is_null() {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                638 as libc::c_int,
            );
        }
        ltests += 1;
        ltests;
        if ex2.is_null() {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                639 as libc::c_int,
            );
        }
        let mut r1: libc::c_double = te_eval(ex1);
        let mut r2: libc::c_double = te_eval(ex2);
        fflush(stdout);
        let olfail: libc::c_int = lfails;
        ltests += 1;
        ltests;
        let __LF_COMPARE: libc::c_double = fabs(r1 - r2);
        if __LF_COMPARE > 0.001f64 || __LF_COMPARE != __LF_COMPARE {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                646 as libc::c_int,
                r1,
                r2,
            );
        }
        if olfail != lfails {
            printf(
                b"Failed expression: %s <> %s\n\0" as *const u8 as *const libc::c_char,
                expr1,
                expr2,
            );
        }
        te_free(ex1);
        te_free(ex2);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_combinatorics() {
    let mut cases: [test_case; 20] = [
        {
            let mut init = test_case {
                expr: b"fac(0)\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"fac(0.2)\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"fac(1)\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"fac(2)\0" as *const u8 as *const libc::c_char,
                answer: 2 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"fac(3)\0" as *const u8 as *const libc::c_char,
                answer: 6 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"fac(4.8)\0" as *const u8 as *const libc::c_char,
                answer: 24 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"fac(10)\0" as *const u8 as *const libc::c_char,
                answer: 3628800 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"ncr(0,0)\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"ncr(10,1)\0" as *const u8 as *const libc::c_char,
                answer: 10 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"ncr(10,0)\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"ncr(10,10)\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"ncr(16,7)\0" as *const u8 as *const libc::c_char,
                answer: 11440 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"ncr(16,9)\0" as *const u8 as *const libc::c_char,
                answer: 11440 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"ncr(100,95)\0" as *const u8 as *const libc::c_char,
                answer: 75287520 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"npr(0,0)\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"npr(10,1)\0" as *const u8 as *const libc::c_char,
                answer: 10 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"npr(10,0)\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"npr(10,10)\0" as *const u8 as *const libc::c_char,
                answer: 3628800 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"npr(20,5)\0" as *const u8 as *const libc::c_char,
                answer: 1860480 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"npr(100,4)\0" as *const u8 as *const libc::c_char,
                answer: 94109400 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[test_case; 20]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<test_case>() as libc::c_ulong)
    {
        let mut expr: *const libc::c_char = cases[i as usize].expr;
        let answer: libc::c_double = cases[i as usize].answer;
        let mut err: libc::c_int = 0;
        let ev: libc::c_double = te_interp(expr, &mut err);
        ltests += 1;
        ltests;
        if err != 0 {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                691 as libc::c_int,
            );
        }
        ltests += 1;
        ltests;
        let __LF_COMPARE: libc::c_double = fabs(ev - answer);
        if __LF_COMPARE > 0.001f64 || __LF_COMPARE != __LF_COMPARE {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                692 as libc::c_int,
                ev,
                answer,
            );
        }
        if err != 0 {
            printf(
                b"FAILED: %s (%d)\n\0" as *const u8 as *const libc::c_char,
                expr,
                err,
            );
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_logic() {
    let mut cases: [test_case; 65] = [
        {
            let mut init = test_case {
                expr: b"1 && 1\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 && 0\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"0 && 1\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"0 && 0\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 || 1\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 || 0\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"0 || 1\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"0 || 0\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"!0\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"!1\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"!2\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"!-2\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"-!2\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"!!0\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"!!1\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"!!2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"!!-2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"!-!2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"-!!2\0" as *const u8 as *const libc::c_char,
                answer: -(1 as libc::c_int) as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"--!!2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 < 2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 < 2\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 <= 2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 > 1\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 > 2\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 >= 2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 > -2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"-2 < 2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"0 == 0\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"0 != 0\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 == 2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 != 2\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 == 3\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 != 3\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 == 2.0001\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 != 2.0001\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 < 2 && 2 < 3\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 < 2 && 3 < 2\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 < 1 && 2 < 3\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 < 1 && 3 < 2\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 < 2 || 2 < 3\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 < 2 || 3 < 2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 < 1 || 2 < 3\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"2 < 1 || 3 < 2\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"0 == 1 < 2\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 == 1 < 2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 != 2 > 3\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 || 0 && 0\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 || 1 && 0\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"0 && 0 || 1\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"!1 == 0\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 < 1+1\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 < 1*2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 < 2/2\0" as *const u8 as *const libc::c_char,
                answer: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"1 < 2^2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"5+5 < 4+10\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"5+(5 < 4)+10\0" as *const u8 as *const libc::c_char,
                answer: 15 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"5+(5 < 4+10)\0" as *const u8 as *const libc::c_char,
                answer: 6 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"(5+5 < 4)+10\0" as *const u8 as *const libc::c_char,
                answer: 10 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"5+!(5 < 4)+10\0" as *const u8 as *const libc::c_char,
                answer: 16 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"5+!(5 < 4+10)\0" as *const u8 as *const libc::c_char,
                answer: 5 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"!(5+5 < 4)+10\0" as *const u8 as *const libc::c_char,
                answer: 11 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"!0^2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"!0^-1\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = test_case {
                expr: b"-!0^2\0" as *const u8 as *const libc::c_char,
                answer: 1 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[test_case; 65]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<test_case>() as libc::c_ulong)
    {
        let mut expr: *const libc::c_char = cases[i as usize].expr;
        let answer: libc::c_double = cases[i as usize].answer;
        let mut err: libc::c_int = 0;
        let ev: libc::c_double = te_interp(expr, &mut err);
        ltests += 1;
        ltests;
        if err != 0 {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                793 as libc::c_int,
            );
        }
        ltests += 1;
        ltests;
        let __LF_COMPARE: libc::c_double = fabs(ev - answer);
        if __LF_COMPARE > 0.001f64 || __LF_COMPARE != __LF_COMPARE {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                794 as libc::c_int,
                ev,
                answer,
            );
        }
        if err != 0 {
            printf(
                b"FAILED: %s (%d)\n\0" as *const u8 as *const libc::c_char,
                expr,
                err,
            );
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_depth() {
    let mut depths: [libc::c_int; 4] = [
        100 as libc::c_int,
        400 as libc::c_int,
        1000 as libc::c_int,
        5000 as libc::c_int,
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i
        < (::core::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int
    {
        let depth: libc::c_int = depths[i as usize];
        let ok: libc::c_int = (depth < 500 as libc::c_int) as libc::c_int;
        let mut j: libc::c_int = 0;
        let mut err: libc::c_int = 0;
        let mut expr: *mut libc::c_char = malloc(
            (depth * 2 as libc::c_int + 2 as libc::c_int) as libc::c_ulong,
        ) as *mut libc::c_char;
        memset(expr as *mut libc::c_void, '(' as i32, depth as libc::c_ulong);
        *expr.offset(depth as isize) = '1' as i32 as libc::c_char;
        memset(
            expr.offset(depth as isize).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            ')' as i32,
            depth as libc::c_ulong,
        );
        *expr
            .offset(
                (depth * 2 as libc::c_int + 1 as libc::c_int) as isize,
            ) = '\0' as i32 as libc::c_char;
        let mut r: libc::c_double = te_interp(expr, &mut err);
        ltests += 1;
        ltests;
        if if ok != 0 {
            (err == 0 as libc::c_int && r == 1.0f64) as libc::c_int
        } else {
            (err != 0 as libc::c_int && r != r) as libc::c_int
        } == 0
        {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                821 as libc::c_int,
            );
        }
        free(expr as *mut libc::c_void);
        expr = malloc((depth * 4 as libc::c_int + 2 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_char;
        j = 0 as libc::c_int;
        while j < depth {
            memcpy(
                expr.offset((j * 4 as libc::c_int) as isize) as *mut libc::c_void,
                b"sin \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            );
            j += 1;
            j;
        }
        *expr.offset((depth * 4 as libc::c_int) as isize) = '1' as i32 as libc::c_char;
        *expr
            .offset(
                (depth * 4 as libc::c_int + 1 as libc::c_int) as isize,
            ) = '\0' as i32 as libc::c_char;
        r = te_interp(expr, &mut err);
        ltests += 1;
        ltests;
        if if ok != 0 {
            (err == 0 as libc::c_int) as libc::c_int
        } else {
            (err != 0 as libc::c_int && r != r) as libc::c_int
        } == 0
        {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                831 as libc::c_int,
            );
        }
        free(expr as *mut libc::c_void);
        i += 1;
        i;
    }
}
static mut number_rand_state: libc::c_ulonglong = 1 as libc::c_int as libc::c_ulonglong;
unsafe extern "C" fn number_rand(mut modulus: libc::c_ulong) -> libc::c_ulong {
    number_rand_state = number_rand_state
        .wrapping_mul(6364136223846793005 as libc::c_ulonglong)
        .wrapping_add(1442695040888963407 as libc::c_ulonglong);
    return (number_rand_state >> 33 as libc::c_int)
        .wrapping_rem(modulus as libc::c_ulonglong) as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn test_number() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 5000 as libc::c_int {
        let mut expr: [libc::c_char; 64] = [0; 64];
        let mut p: *mut libc::c_char = expr.as_mut_ptr();
        let mut j: libc::c_int = 0;
        if number_rand(8 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {
            let digits: libc::c_int = (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(number_rand(8 as libc::c_int as libc::c_ulong))
                as libc::c_int;
            p = p
                .offset(
                    sprintf(p, b"0x\0" as *const u8 as *const libc::c_char) as isize,
                );
            j = 0 as libc::c_int;
            while j < digits {
                p = p
                    .offset(
                        sprintf(
                            p,
                            b"%c\0" as *const u8 as *const libc::c_char,
                            (*::core::mem::transmute::<
                                &[u8; 23],
                                &[libc::c_char; 23],
                            >(
                                b"0123456789abcdefABCDEF\0",
                            ))[number_rand(22 as libc::c_int as libc::c_ulong) as usize]
                                as libc::c_int,
                        ) as isize,
                    );
                j += 1;
                j;
            }
        } else {
            let int_digits: libc::c_int = number_rand(11 as libc::c_int as libc::c_ulong)
                as libc::c_int;
            let frac_digits: libc::c_int = (if int_digits != 0 {
                number_rand(11 as libc::c_int as libc::c_ulong)
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(number_rand(10 as libc::c_int as libc::c_ulong))
            }) as libc::c_int;
            j = 0 as libc::c_int;
            while j < int_digits {
                p = p
                    .offset(
                        sprintf(
                            p,
                            b"%c\0" as *const u8 as *const libc::c_char,
                            (*::core::mem::transmute::<
                                &[u8; 11],
                                &[libc::c_char; 11],
                            >(
                                b"0123456789\0",
                            ))[number_rand(10 as libc::c_int as libc::c_ulong) as usize]
                                as libc::c_int,
                        ) as isize,
                    );
                j += 1;
                j;
            }
            if frac_digits != 0 || number_rand(2 as libc::c_int as libc::c_ulong) != 0 {
                p = p
                    .offset(
                        sprintf(p, b".\0" as *const u8 as *const libc::c_char) as isize,
                    );
                j = 0 as libc::c_int;
                while j < frac_digits {
                    p = p
                        .offset(
                            sprintf(
                                p,
                                b"%c\0" as *const u8 as *const libc::c_char,
                                (*::core::mem::transmute::<
                                    &[u8; 11],
                                    &[libc::c_char; 11],
                                >(
                                    b"0123456789\0",
                                ))[number_rand(10 as libc::c_int as libc::c_ulong) as usize]
                                    as libc::c_int,
                            ) as isize,
                        );
                    j += 1;
                    j;
                }
            }
            if number_rand(2 as libc::c_int as libc::c_ulong) != 0 {
                p = p
                    .offset(
                        sprintf(
                            p,
                            b"%c\0" as *const u8 as *const libc::c_char,
                            (*::core::mem::transmute::<
                                &[u8; 3],
                                &[libc::c_char; 3],
                            >(
                                b"eE\0",
                            ))[number_rand(2 as libc::c_int as libc::c_ulong) as usize]
                                as libc::c_int,
                        ) as isize,
                    );
                if number_rand(2 as libc::c_int as libc::c_ulong) != 0 {
                    p = p
                        .offset(
                            sprintf(
                                p,
                                b"%c\0" as *const u8 as *const libc::c_char,
                                (*::core::mem::transmute::<
                                    &[u8; 3],
                                    &[libc::c_char; 3],
                                >(
                                    b"+-\0",
                                ))[number_rand(2 as libc::c_int as libc::c_ulong) as usize]
                                    as libc::c_int,
                            ) as isize,
                        );
                }
                p = p
                    .offset(
                        sprintf(
                            p,
                            b"%lu\0" as *const u8 as *const libc::c_char,
                            number_rand(320 as libc::c_int as libc::c_ulong),
                        ) as isize,
                    );
            }
        }
        let expected: libc::c_double = strtod(
            expr.as_mut_ptr(),
            0 as *mut *mut libc::c_char,
        );
        let mut err: libc::c_int = 0;
        let got: libc::c_double = te_interp(expr.as_mut_ptr(), &mut err);
        let ok: libc::c_int = (err == 0 as libc::c_int
            && (got == expected
                || fabs(got - expected) <= 1e-14f64 * fabs(expected) + 1e-304f64))
            as libc::c_int;
        ltests += 1;
        ltests;
        if ok == 0 {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                886 as libc::c_int,
            );
        }
        if ok == 0 {
            printf(
                b"FAILED: %s => %.17g, expected %.17g (err=%d)\n\0" as *const u8
                    as *const libc::c_char,
                expr.as_mut_ptr(),
                got,
                expected,
                err,
            );
        }
        i += 1;
        i;
    }
    let mut partial: [*const libc::c_char; 4] = [
        b"1e\0" as *const u8 as *const libc::c_char,
        b"1e+\0" as *const u8 as *const libc::c_char,
        b"0x\0" as *const u8 as *const libc::c_char,
        b".\0" as *const u8 as *const libc::c_char,
    ];
    let mut j_0: libc::c_int = 0;
    j_0 = 0 as libc::c_int;
    while j_0
        < (::core::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int
    {
        let mut err_0: libc::c_int = 0;
        let r: libc::c_double = te_interp(partial[j_0 as usize], &mut err_0);
        ltests += 1;
        ltests;
        if !(r != r) {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                901 as libc::c_int,
            );
        }
        ltests += 1;
        ltests;
        if !(err_0 != 0 as libc::c_int) {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
                902 as libc::c_int,
            );
        }
        j_0 += 1;
        j_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_locale() {
    let mut locales: [*const libc::c_char; 7] = [
        b"de_DE.UTF-8\0" as *const u8 as *const libc::c_char,
        b"de_DE.utf8\0" as *const u8 as *const libc::c_char,
        b"fr_FR.UTF-8\0" as *const u8 as *const libc::c_char,
        b"fr_FR.utf8\0" as *const u8 as *const libc::c_char,
        b"es_ES.UTF-8\0" as *const u8 as *const libc::c_char,
        b"es_ES.utf8\0" as *const u8 as *const libc::c_char,
        b"German_Germany.1252\0" as *const u8 as *const libc::c_char,
    ];
    let mut found: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i
        < (::core::mem::size_of::<[*const libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int
    {
        if !(setlocale(6 as libc::c_int, locales[i as usize])).is_null()
            && *((*localeconv()).decimal_point).offset(0 as libc::c_int as isize)
                as libc::c_int == ',' as i32
        {
            found = locales[i as usize];
            break;
        } else {
            i += 1;
            i;
        }
    }
    if found.is_null() {
        setlocale(6 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
        printf(
            b"no comma-decimal locale installed, skipping\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    ltests += 1;
    ltests;
    let __LF_COMPARE: libc::c_double = fabs(
        te_interp(b"1.5\0" as *const u8 as *const libc::c_char, 0 as *mut libc::c_int)
            - 1.5f64,
    );
    if __LF_COMPARE > 0.001f64 || __LF_COMPARE != __LF_COMPARE {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            929 as libc::c_int,
            te_interp(
                b"1.5\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_int,
            ),
            1.5f64,
        );
    }
    ltests += 1;
    ltests;
    let __LF_COMPARE_0: libc::c_double = fabs(
        te_interp(b".5\0" as *const u8 as *const libc::c_char, 0 as *mut libc::c_int)
            - 0.5f64,
    );
    if __LF_COMPARE_0 > 0.001f64 || __LF_COMPARE_0 != __LF_COMPARE_0 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            930 as libc::c_int,
            te_interp(
                b".5\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_int,
            ),
            0.5f64,
        );
    }
    ltests += 1;
    ltests;
    let __LF_COMPARE_1: libc::c_double = fabs(
        te_interp(
            b"2.25+3.5\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_int,
        ) - 5.75f64,
    );
    if __LF_COMPARE_1 > 0.001f64 || __LF_COMPARE_1 != __LF_COMPARE_1 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            931 as libc::c_int,
            te_interp(
                b"2.25+3.5\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_int,
            ),
            5.75f64,
        );
    }
    ltests += 1;
    ltests;
    let __LF_COMPARE_2: libc::c_double = fabs(
        te_interp(b"1e3\0" as *const u8 as *const libc::c_char, 0 as *mut libc::c_int)
            - 1000 as libc::c_int as libc::c_double,
    );
    if __LF_COMPARE_2 > 0.001f64 || __LF_COMPARE_2 != __LF_COMPARE_2 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            932 as libc::c_int,
            te_interp(
                b"1e3\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_int,
            ),
            1000 as libc::c_int as libc::c_double,
        );
    }
    ltests += 1;
    ltests;
    let __LF_COMPARE_3: libc::c_double = fabs(
        te_interp(b"1.5e-1\0" as *const u8 as *const libc::c_char, 0 as *mut libc::c_int)
            - 0.15f64,
    );
    if __LF_COMPARE_3 > 0.001f64 || __LF_COMPARE_3 != __LF_COMPARE_3 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            933 as libc::c_int,
            te_interp(
                b"1.5e-1\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_int,
            ),
            0.15f64,
        );
    }
    ltests += 1;
    ltests;
    let __LF_COMPARE_4: libc::c_double = fabs(
        te_interp(b"0x1F\0" as *const u8 as *const libc::c_char, 0 as *mut libc::c_int)
            - 31 as libc::c_int as libc::c_double,
    );
    if __LF_COMPARE_4 > 0.001f64 || __LF_COMPARE_4 != __LF_COMPARE_4 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            934 as libc::c_int,
            te_interp(
                b"0x1F\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_int,
            ),
            31 as libc::c_int as libc::c_double,
        );
    }
    ltests += 1;
    ltests;
    let __LF_COMPARE_5: libc::c_double = fabs(
        te_interp(b"1,5\0" as *const u8 as *const libc::c_char, 0 as *mut libc::c_int)
            - 5 as libc::c_int as libc::c_double,
    );
    if __LF_COMPARE_5 > 0.001f64 || __LF_COMPARE_5 != __LF_COMPARE_5 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"/work/src/smoke.c\0" as *const u8 as *const libc::c_char,
            935 as libc::c_int,
            te_interp(
                b"1,5\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_int,
            ),
            5 as libc::c_int as libc::c_double,
        );
    }
    setlocale(6 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let ts: libc::c_int = ltests;
    let fs: libc::c_int = lfails;
    let start: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"Results\0" as *const u8 as *const libc::c_char,
    );
    test_results();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts - (lfails - fs),
        lfails - fs,
        ((clock() - start) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_0: libc::c_int = ltests;
    let fs_0: libc::c_int = lfails;
    let start_0: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"Syntax\0" as *const u8 as *const libc::c_char,
    );
    test_syntax();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_0 - (lfails - fs_0),
        lfails - fs_0,
        ((clock() - start_0) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_1: libc::c_int = ltests;
    let fs_1: libc::c_int = lfails;
    let start_1: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"NaNs\0" as *const u8 as *const libc::c_char,
    );
    test_nans();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_1 - (lfails - fs_1),
        lfails - fs_1,
        ((clock() - start_1) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_2: libc::c_int = ltests;
    let fs_2: libc::c_int = lfails;
    let start_2: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"INFs\0" as *const u8 as *const libc::c_char,
    );
    test_infs();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_2 - (lfails - fs_2),
        lfails - fs_2,
        ((clock() - start_2) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_3: libc::c_int = ltests;
    let fs_3: libc::c_int = lfails;
    let start_3: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"Variables\0" as *const u8 as *const libc::c_char,
    );
    test_variables();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_3 - (lfails - fs_3),
        lfails - fs_3,
        ((clock() - start_3) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_4: libc::c_int = ltests;
    let fs_4: libc::c_int = lfails;
    let start_4: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"Functions\0" as *const u8 as *const libc::c_char,
    );
    test_functions();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_4 - (lfails - fs_4),
        lfails - fs_4,
        ((clock() - start_4) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_5: libc::c_int = ltests;
    let fs_5: libc::c_int = lfails;
    let start_5: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"Dynamic\0" as *const u8 as *const libc::c_char,
    );
    test_dynamic();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_5 - (lfails - fs_5),
        lfails - fs_5,
        ((clock() - start_5) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_6: libc::c_int = ltests;
    let fs_6: libc::c_int = lfails;
    let start_6: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"Closure\0" as *const u8 as *const libc::c_char,
    );
    test_closure();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_6 - (lfails - fs_6),
        lfails - fs_6,
        ((clock() - start_6) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_7: libc::c_int = ltests;
    let fs_7: libc::c_int = lfails;
    let start_7: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"Optimize\0" as *const u8 as *const libc::c_char,
    );
    test_optimize();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_7 - (lfails - fs_7),
        lfails - fs_7,
        ((clock() - start_7) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_8: libc::c_int = ltests;
    let fs_8: libc::c_int = lfails;
    let start_8: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"Pow\0" as *const u8 as *const libc::c_char,
    );
    test_pow();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_8 - (lfails - fs_8),
        lfails - fs_8,
        ((clock() - start_8) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_9: libc::c_int = ltests;
    let fs_9: libc::c_int = lfails;
    let start_9: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"Combinatorics\0" as *const u8 as *const libc::c_char,
    );
    test_combinatorics();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_9 - (lfails - fs_9),
        lfails - fs_9,
        ((clock() - start_9) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_10: libc::c_int = ltests;
    let fs_10: libc::c_int = lfails;
    let start_10: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"Logic\0" as *const u8 as *const libc::c_char,
    );
    test_logic();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_10 - (lfails - fs_10),
        lfails - fs_10,
        ((clock() - start_10) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_11: libc::c_int = ltests;
    let fs_11: libc::c_int = lfails;
    let start_11: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"Depth\0" as *const u8 as *const libc::c_char,
    );
    test_depth();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_11 - (lfails - fs_11),
        lfails - fs_11,
        ((clock() - start_11) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_12: libc::c_int = ltests;
    let fs_12: libc::c_int = lfails;
    let start_12: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"Number\0" as *const u8 as *const libc::c_char,
    );
    test_number();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_12 - (lfails - fs_12),
        lfails - fs_12,
        ((clock() - start_12) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_13: libc::c_int = ltests;
    let fs_13: libc::c_int = lfails;
    let start_13: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"Locale\0" as *const u8 as *const libc::c_char,
    );
    test_locale();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_13 - (lfails - fs_13),
        lfails - fs_13,
        ((clock() - start_13) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    if lfails == 0 as libc::c_int {
        printf(
            b"ALL TESTS PASSED (%d/%d)\n\0" as *const u8 as *const libc::c_char,
            ltests,
            ltests,
        );
    } else {
        printf(
            b"SOME TESTS FAILED (%d/%d)\n\0" as *const u8 as *const libc::c_char,
            ltests - lfails,
            ltests,
        );
    }
    return (lfails != 0 as libc::c_int) as libc::c_int;
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
