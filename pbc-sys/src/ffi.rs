pub type __int64_t = ::std::os::raw::c_longlong;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub _read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: fpos_t,
            arg3: ::std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
}

pub type FILE = __sFILE;
pub type mp_limb_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __mpz_struct {
    pub _mp_alloc: ::std::os::raw::c_int,
    pub _mp_size: ::std::os::raw::c_int,
    pub _mp_d: *mut mp_limb_t,
}

pub type mpz_t = __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
pub type const_mpz_ptr = *const __mpz_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct element_s {
    pub field: *const field_s,
    pub data: *mut ::std::os::raw::c_void,
}

pub type element_ptr = *mut element_s;
pub type const_element_ptr = *const element_s;
pub type element_t = element_s;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct element_pp_s {
    pub field: *mut field_s,
    pub data: *mut ::std::os::raw::c_void,
}

pub type element_pp_t = [element_pp_s; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct multiz_s {
    _unused: [u8; 0],
}
pub type multiz = *mut multiz_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct field_s {
    pub field_clear: ::std::option::Option<unsafe extern "C" fn(f: *mut field_s)>,
    pub init: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr)>,
    pub clear: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr)>,
    pub set_mpz: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr, arg2: mpz_ptr)>,
    pub set_multiz: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr, arg2: multiz)>,
    pub set:
        ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr, arg2: const_element_ptr)>,
    pub set0: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr)>,
    pub set1: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr)>,
    pub set_str: ::std::option::Option<
        unsafe extern "C" fn(
            e: element_ptr,
            s: *const ::std::os::raw::c_char,
            base: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub out_str: ::std::option::Option<
        unsafe extern "C" fn(
            stream: *mut FILE,
            base: ::std::os::raw::c_int,
            arg1: element_ptr,
        ) -> usize,
    >,
    pub add: ::std::option::Option<
        unsafe extern "C" fn(arg1: element_ptr, arg2: const_element_ptr, arg3: const_element_ptr),
    >,
    pub sub: ::std::option::Option<
        unsafe extern "C" fn(arg1: element_ptr, arg2: const_element_ptr, arg3: const_element_ptr),
    >,
    pub mul: ::std::option::Option<
        unsafe extern "C" fn(arg1: element_ptr, arg2: const_element_ptr, arg3: const_element_ptr),
    >,
    pub is_sqr:
        ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr) -> ::std::os::raw::c_int>,
    pub sqrt: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr, arg2: element_ptr)>,
    pub item_count:
        ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr) -> ::std::os::raw::c_int>,
    pub item: ::std::option::Option<
        unsafe extern "C" fn(arg1: element_ptr, arg2: ::std::os::raw::c_int) -> element_ptr,
    >,
    pub get_x: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr) -> element_ptr>,
    pub get_y: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr) -> element_ptr>,
    pub set_si: ::std::option::Option<
        unsafe extern "C" fn(arg1: element_ptr, arg2: ::std::os::raw::c_long),
    >,
    pub add_ui: ::std::option::Option<
        unsafe extern "C" fn(arg1: element_ptr, arg2: element_ptr, arg3: ::std::os::raw::c_ulong),
    >,
    pub mul_mpz: ::std::option::Option<
        unsafe extern "C" fn(arg1: element_ptr, arg2: element_ptr, arg3: mpz_ptr),
    >,
    pub mul_si: ::std::option::Option<
        unsafe extern "C" fn(arg1: element_ptr, arg2: element_ptr, arg3: ::std::os::raw::c_long),
    >,
    pub div: ::std::option::Option<
        unsafe extern "C" fn(arg1: element_ptr, arg2: const_element_ptr, arg3: const_element_ptr),
    >,
    pub doub: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr, arg2: element_ptr)>,
    pub multi_doub: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut element_ptr,
            arg2: *mut element_ptr,
            n: ::std::os::raw::c_int,
        ),
    >,
    pub multi_add: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut element_ptr,
            arg2: *mut element_ptr,
            arg3: *mut element_ptr,
            n: ::std::os::raw::c_int,
        ),
    >,
    pub halve: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr, arg2: element_ptr)>,
    pub square: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr, arg2: element_ptr)>,
    pub cubic: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr, arg2: element_ptr)>,
    pub pow_mpz: ::std::option::Option<
        unsafe extern "C" fn(arg1: element_ptr, arg2: const_element_ptr, arg3: const_mpz_ptr),
    >,
    pub invert: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr, arg2: element_ptr)>,
    pub neg: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr, arg2: element_ptr)>,
    pub random: ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr)>,
    pub from_hash: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: element_ptr,
            data: *const ::std::os::raw::c_void,
            len: ::std::os::raw::c_int,
        ),
    >,
    pub is1:
        ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr) -> ::std::os::raw::c_int>,
    pub is0: ::std::option::Option<
        unsafe extern "C" fn(arg1: const_element_ptr) -> ::std::os::raw::c_int,
    >,
    pub sign:
        ::std::option::Option<unsafe extern "C" fn(arg1: element_ptr) -> ::std::os::raw::c_int>,
    pub cmp: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: const_element_ptr,
            arg2: const_element_ptr,
        ) -> ::std::os::raw::c_int,
    >,
    pub to_bytes: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_uchar,
            arg1: const_element_ptr,
        ) -> ::std::os::raw::c_int,
    >,
    pub from_bytes: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: element_ptr,
            data: *mut ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
    >,
    pub length_in_bytes: ::std::option::Option<
        unsafe extern "C" fn(arg1: const_element_ptr) -> ::std::os::raw::c_int,
    >,
    pub fixed_length_in_bytes: ::std::os::raw::c_int,
    pub snprint: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut ::std::os::raw::c_char,
            n: usize,
            e: element_ptr,
        ) -> ::std::os::raw::c_int,
    >,
    pub to_mpz: ::std::option::Option<unsafe extern "C" fn(arg1: mpz_ptr, arg2: const_element_ptr)>,
    pub out_info: ::std::option::Option<unsafe extern "C" fn(arg1: *mut FILE, arg2: *mut field_s)>,
    pub pp_init:
        ::std::option::Option<unsafe extern "C" fn(p: *mut element_pp_s, in_: *mut element_s)>,
    pub pp_clear: ::std::option::Option<unsafe extern "C" fn(p: *mut element_pp_s)>,
    pub pp_pow: ::std::option::Option<
        unsafe extern "C" fn(out: *mut element_s, power: mpz_ptr, p: *mut element_pp_s),
    >,
    pub pairing: *mut pairing_s,
    pub order: mpz_t,
    pub nqr: element_ptr,
    pub name: *mut ::std::os::raw::c_char,
    pub data: *mut ::std::os::raw::c_void,
}

pub type field_ptr = *mut field_s;
pub type field_t = [field_s; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pairing_pp_s {
    pub pairing: *mut pairing_s,
    pub data: *mut ::std::os::raw::c_void,
}
pub type pairing_pp_t = [pairing_pp_s; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pairing_s {
    pub r: mpz_t,
    pub Zr: field_t,
    pub G1: field_ptr,
    pub G2: field_ptr,
    pub GT: field_t,
    pub phikonr: mpz_t,
    pub phi: ::std::option::Option<
        unsafe extern "C" fn(out: element_ptr, in_: element_ptr, pairing: *mut pairing_s),
    >,
    pub map: ::std::option::Option<
        unsafe extern "C" fn(
            out: element_ptr,
            in1: const_element_ptr,
            in2: const_element_ptr,
            p: *const pairing_s,
        ),
    >,
    pub prod_pairings: ::std::option::Option<
        unsafe extern "C" fn(
            out: element_ptr,
            in1: *mut element_t,
            in2: *mut element_t,
            n_prod: ::std::os::raw::c_int,
            p: *mut pairing_s,
        ),
    >,
    pub is_almost_coddh: ::std::option::Option<
        unsafe extern "C" fn(
            a: element_ptr,
            b: element_ptr,
            c: element_ptr,
            d: element_ptr,
            p: *mut pairing_s,
        ) -> ::std::os::raw::c_int,
    >,
    pub clear_func: ::std::option::Option<unsafe extern "C" fn(arg1: *mut pairing_s)>,
    pub pp_init: ::std::option::Option<
        unsafe extern "C" fn(p: *mut pairing_pp_s, in1: *mut element_s, arg1: *mut pairing_s),
    >,
    pub pp_clear: ::std::option::Option<unsafe extern "C" fn(p: *mut pairing_pp_s)>,
    pub pp_apply: ::std::option::Option<
        unsafe extern "C" fn(out: *mut element_s, in2: *mut element_s, p: *mut pairing_pp_s),
    >,
    pub finalpow: ::std::option::Option<unsafe extern "C" fn(e: *mut element_s)>,
    pub option_set: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pairing_s,
            key: *mut ::std::os::raw::c_char,
            value: *mut ::std::os::raw::c_char,
        ),
    >,
    pub data: *mut ::std::os::raw::c_void,
}
pub type pairing_t = pairing_s;
