use libc;

use crate::dc_tools::*;
use crate::types::*;
use crate::x::*;

/* *
 * @class dc_param_t
 *
 * An object for handling key=value parameter lists; for the key, curently only
 * a single character is allowed.
 *
 * The object is used eg. by dc_chat_t or dc_msg_t, for readable paramter names,
 * these classes define some DC_PARAM_* constantats.
 *
 * Only for library-internal use.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dc_param_t {
    pub packed: *mut libc::c_char,
}

// values for DC_PARAM_FORCE_PLAINTEXT
/* user functions */
pub unsafe fn dc_param_exists(mut param: *mut dc_param_t, mut key: libc::c_int) -> libc::c_int {
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    if param.is_null() || key == 0i32 {
        return 0i32;
    }
    return if !find_param((*param).packed, key, &mut p2).is_null() {
        1i32
    } else {
        0i32
    };
}
unsafe extern "C" fn find_param(
    mut haystack: *mut libc::c_char,
    mut key: libc::c_int,
    mut ret_p2: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    p1 = haystack;
    loop {
        if p1.is_null() || *p1 as libc::c_int == 0i32 {
            return 0 as *mut libc::c_char;
        } else {
            if *p1 as libc::c_int == key && *p1.offset(1isize) as libc::c_int == '=' as i32 {
                break;
            }
            p1 = strchr(p1, '\n' as i32);
            if !p1.is_null() {
                p1 = p1.offset(1isize)
            }
        }
    }
    p2 = strchr(p1, '\n' as i32);
    if p2.is_null() {
        p2 = &mut *p1.offset(strlen(p1) as isize) as *mut libc::c_char
    }
    *ret_p2 = p2;
    return p1;
}
/* the value may be an empty string, "def" is returned only if the value unset.  The result must be free()'d in any case. */
pub unsafe fn dc_param_get(
    mut param: *const dc_param_t,
    mut key: libc::c_int,
    mut def: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bak: libc::c_char = 0i32 as libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if param.is_null() || key == 0i32 {
        return if !def.is_null() {
            dc_strdup(def)
        } else {
            0 as *mut libc::c_char
        };
    }
    p1 = find_param((*param).packed, key, &mut p2);
    if p1.is_null() {
        return if !def.is_null() {
            dc_strdup(def)
        } else {
            0 as *mut libc::c_char
        };
    }
    p1 = p1.offset(2isize);
    bak = *p2;
    *p2 = 0i32 as libc::c_char;
    ret = dc_strdup(p1);
    dc_rtrim(ret);
    *p2 = bak;
    return ret;
}
pub unsafe fn dc_param_get_int(
    mut param: *const dc_param_t,
    mut key: libc::c_int,
    mut def: int32_t,
) -> int32_t {
    if param.is_null() || key == 0i32 {
        return def;
    }
    let mut str: *mut libc::c_char = dc_param_get(param, key, 0 as *const libc::c_char);
    if str.is_null() {
        return def;
    }
    let mut ret: int32_t = atol(str) as int32_t;
    free(str as *mut libc::c_void);
    return ret;
}
pub unsafe fn dc_param_set(
    mut param: *mut dc_param_t,
    mut key: libc::c_int,
    mut value: *const libc::c_char,
) {
    let mut old1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new1: *mut libc::c_char = 0 as *mut libc::c_char;
    if param.is_null() || key == 0i32 {
        return;
    }
    old1 = (*param).packed;
    old2 = 0 as *mut libc::c_char;
    if !old1.is_null() {
        let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
        p1 = find_param(old1, key, &mut p2);
        if !p1.is_null() {
            *p1 = 0i32 as libc::c_char;
            old2 = p2
        } else if value.is_null() {
            return;
        }
    }
    dc_rtrim(old1);
    dc_ltrim(old2);
    if !old1.is_null() && *old1.offset(0isize) as libc::c_int == 0i32 {
        old1 = 0 as *mut libc::c_char
    }
    if !old2.is_null() && *old2.offset(0isize) as libc::c_int == 0i32 {
        old2 = 0 as *mut libc::c_char
    }
    if !value.is_null() {
        new1 = dc_mprintf(
            b"%s%s%c=%s%s%s\x00" as *const u8 as *const libc::c_char,
            if !old1.is_null() {
                old1
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if !old1.is_null() {
                b"\n\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            key,
            value,
            if !old2.is_null() {
                b"\n\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if !old2.is_null() {
                old2
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
        )
    } else {
        new1 = dc_mprintf(
            b"%s%s%s\x00" as *const u8 as *const libc::c_char,
            if !old1.is_null() {
                old1
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if !old1.is_null() && !old2.is_null() {
                b"\n\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if !old2.is_null() {
                old2
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
        )
    }
    free((*param).packed as *mut libc::c_void);
    (*param).packed = new1;
}
pub unsafe fn dc_param_set_int(
    mut param: *mut dc_param_t,
    mut key: libc::c_int,
    mut value: int32_t,
) {
    if param.is_null() || key == 0i32 {
        return;
    }
    let mut value_str: *mut libc::c_char = dc_mprintf(
        b"%i\x00" as *const u8 as *const libc::c_char,
        value as libc::c_int,
    );
    if value_str.is_null() {
        return;
    }
    dc_param_set(param, key, value_str);
    free(value_str as *mut libc::c_void);
}
/* library-private */
pub unsafe fn dc_param_new() -> *mut dc_param_t {
    let mut param: *mut dc_param_t = 0 as *mut dc_param_t;
    param = calloc(
        1i32 as libc::c_ulong,
        ::std::mem::size_of::<dc_param_t>() as libc::c_ulong,
    ) as *mut dc_param_t;
    if param.is_null() {
        exit(28i32);
    }
    (*param).packed = calloc(1i32 as libc::c_ulong, 1i32 as libc::c_ulong) as *mut libc::c_char;
    return param;
}
pub unsafe fn dc_param_empty(mut param: *mut dc_param_t) {
    if param.is_null() {
        return;
    }
    *(*param).packed.offset(0isize) = 0i32 as libc::c_char;
}
pub unsafe fn dc_param_unref(mut param: *mut dc_param_t) {
    if param.is_null() {
        return;
    }
    dc_param_empty(param);
    free((*param).packed as *mut libc::c_void);
    free(param as *mut libc::c_void);
}
pub unsafe fn dc_param_set_packed(mut param: *mut dc_param_t, mut packed: *const libc::c_char) {
    if param.is_null() {
        return;
    }
    dc_param_empty(param);
    if !packed.is_null() {
        free((*param).packed as *mut libc::c_void);
        (*param).packed = dc_strdup(packed)
    };
}
pub unsafe fn dc_param_set_urlencoded(
    mut param: *mut dc_param_t,
    mut urlencoded: *const libc::c_char,
) {
    if param.is_null() {
        return;
    }
    dc_param_empty(param);
    if !urlencoded.is_null() {
        free((*param).packed as *mut libc::c_void);
        (*param).packed = dc_strdup(urlencoded);
        dc_str_replace(
            &mut (*param).packed,
            b"&\x00" as *const u8 as *const libc::c_char,
            b"\n\x00" as *const u8 as *const libc::c_char,
        );
    };
}
