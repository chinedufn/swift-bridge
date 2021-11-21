use macro_::vec_externs;

vec_externs!(u8);
vec_externs!(u16);
vec_externs!(u32);
vec_externs!(u64);
vec_externs!(usize);

vec_externs!(i8);
vec_externs!(i16);
vec_externs!(i32);
vec_externs!(i64);
vec_externs!(isize);

vec_externs!(bool);

mod macro_ {
    macro_rules! vec_externs {
        ($ty:ty) => {
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
                pub extern "C" fn _pop(vec: *mut Vec<$ty>) {
                    let vec = unsafe { &mut *vec };
                    vec.pop();
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
