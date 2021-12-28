use macro_::vec_externs;

vec_externs!(u8, 123);
vec_externs!(u16, 123);
vec_externs!(u32, 123);
vec_externs!(u64, 123);
vec_externs!(usize, 123);

vec_externs!(i8, 123);
vec_externs!(i16, 123);
vec_externs!(i32, 123);
vec_externs!(i64, 123);
vec_externs!(isize, 123);

vec_externs!(f32, 0.123);
vec_externs!(f64, 0.123);

vec_externs!(bool, false);

mod macro_ {
    macro_rules! vec_externs {
        ($ty:ty, $unused_none:expr) => {
            const _: () = {
                #[export_name = concat!("__swift_bridge__$Vec_", stringify!($ty), "$new")]
                #[doc(hidden)]
                pub extern "C" fn _new() -> *mut Vec<$ty> {
                    Box::into_raw(Box::new(Vec::new()))
                }

                #[export_name = concat!("__swift_bridge__$Vec_", stringify!($ty), "$_free")]
                #[doc(hidden)]
                pub extern "C" fn _drop(vec: *mut Vec<$ty>) {
                    let vec = unsafe { Box::from_raw(vec) };
                    drop(vec)
                }

                #[export_name = concat!("__swift_bridge__$Vec_", stringify!($ty), "$len")]
                #[doc(hidden)]
                pub extern "C" fn _len(vec: *mut Vec<$ty>) -> usize {
                    let vec = unsafe { &*vec };
                    vec.len()
                }

                #[export_name = concat!("__swift_bridge__$Vec_", stringify!($ty), "$push")]
                #[doc(hidden)]
                pub extern "C" fn _push(vec: *mut Vec<$ty>, val: $ty) {
                    let vec = unsafe { &mut *vec };
                    vec.push(val);
                }

                #[export_name = concat!("__swift_bridge__$Vec_", stringify!($ty), "$pop")]
                #[doc(hidden)]
                pub extern "C" fn _pop(vec: *mut Vec<$ty>) -> $ty {
                    let vec = unsafe { &mut *vec };
                    if let Some(val) = vec.pop() {
                        crate::option::_set_option_return(true);
                        val
                    } else {
                        crate::option::_set_option_return(false);
                        $unused_none
                    }
                }

                // TODO: Return *const $ty and have that be an `UnsafePointer<$ty>` on the Swift
                //  side.
                #[export_name = concat!("__swift_bridge__$Vec_", stringify!($ty), "$get")]
                #[doc(hidden)]
                pub extern "C" fn _get(vec: *mut Vec<$ty>, index: usize) -> $ty {
                    let vec = unsafe { &*vec };
                    if let Some(val) = vec.get(index) {
                        crate::option::_set_option_return(true);
                        *val
                    } else {
                        crate::option::_set_option_return(false);
                        $unused_none
                    }
                }

                // TODO: Return *mut $ty and have that be an `UnsafeMutablePointer<$ty>` on the Swift
                //  side.
                #[export_name = concat!("__swift_bridge__$Vec_", stringify!($ty), "$get_mut")]
                #[doc(hidden)]
                pub extern "C" fn _get_mut(vec: *mut Vec<$ty>, index: usize) -> $ty {
                    let vec = unsafe { &mut *vec };
                    if let Some(val) = vec.get(index) {
                        crate::option::_set_option_return(true);
                        *val
                    } else {
                        crate::option::_set_option_return(false);
                        $unused_none
                    }
                }

                #[export_name = concat!("__swift_bridge__$Vec_", stringify!($ty), "$as_ptr")]
                #[doc(hidden)]
                pub extern "C" fn _as_ptr(vec: *mut Vec<$ty>) -> *const $ty {
                    let vec = unsafe { &*vec };
                    vec.as_ptr()
                }
            };
        };
    }

    pub(super) use vec_externs;
}
