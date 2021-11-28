#include <stdint.h>
#include <stdbool.h> 
typedef struct RustStr { uint8_t* const start; uintptr_t len; } RustStr;
typedef struct __private__FfiSlice { void* const start; uintptr_t len; } __private__FfiSlice;
typedef struct __private__PointerToSwiftType { void* ptr; } __private__RustHandleToSwiftType;
bool _get_option_arg(uint8_t is_some);
void _set_option_arg(uint8_t idx, bool is_some);
bool _get_option_return();
void _set_option_return(bool is_some);

void* __swift_bridge__$Vec_u8$new();
void __swift_bridge__$Vec_u8$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_u8$len(void* const vec);
void __swift_bridge__$Vec_u8$push(void* const vec, uint8_t val);
uint8_t __swift_bridge__$Vec_u8$pop(void* const vec);
uint8_t __swift_bridge__$Vec_u8$get(void* const vec, uintptr_t index);
uint8_t const * __swift_bridge__$Vec_u8$as_ptr(void* const vec);

void* __swift_bridge__$Vec_u16$new();
void __swift_bridge__$Vec_u16$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_u16$len(void* const vec);
void __swift_bridge__$Vec_u16$push(void* const vec, uint16_t val);
uint16_t __swift_bridge__$Vec_u16$pop(void* const vec);
uint16_t __swift_bridge__$Vec_u16$get(void* const vec, uintptr_t index);
uint16_t const * __swift_bridge__$Vec_u16$as_ptr(void* const vec);

void* __swift_bridge__$Vec_u32$new();
void __swift_bridge__$Vec_u32$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_u32$len(void* const vec);
void __swift_bridge__$Vec_u32$push(void* const vec, uint32_t val);
uint32_t __swift_bridge__$Vec_u32$pop(void* const vec);
uint32_t __swift_bridge__$Vec_u32$get(void* const vec, uintptr_t index);
uint32_t const * __swift_bridge__$Vec_u32$as_ptr(void* const vec);

void* __swift_bridge__$Vec_u64$new();
void __swift_bridge__$Vec_u64$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_u64$len(void* const vec);
void __swift_bridge__$Vec_u64$push(void* const vec, uint64_t val);
uint64_t __swift_bridge__$Vec_u64$pop(void* const vec);
uint64_t __swift_bridge__$Vec_u64$get(void* const vec, uintptr_t index);
uint64_t const * __swift_bridge__$Vec_u64$as_ptr(void* const vec);

void* __swift_bridge__$Vec_usize$new();
void __swift_bridge__$Vec_usize$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_usize$len(void* const vec);
void __swift_bridge__$Vec_usize$push(void* const vec, uintptr_t val);
uintptr_t __swift_bridge__$Vec_usize$pop(void* const vec);
uintptr_t __swift_bridge__$Vec_usize$get(void* const vec, uintptr_t index);
uintptr_t const * __swift_bridge__$Vec_usize$as_ptr(void* const vec);

void* __swift_bridge__$Vec_i8$new();
void __swift_bridge__$Vec_i8$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_i8$len(void* const vec);
void __swift_bridge__$Vec_i8$push(void* const vec, int8_t val);
int8_t __swift_bridge__$Vec_i8$pop(void* const vec);
int8_t __swift_bridge__$Vec_i8$get(void* const vec, uintptr_t index);
int8_t const * __swift_bridge__$Vec_i8$as_ptr(void* const vec);

void* __swift_bridge__$Vec_i16$new();
void __swift_bridge__$Vec_i16$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_i16$len(void* const vec);
void __swift_bridge__$Vec_i16$push(void* const vec, int16_t val);
int16_t __swift_bridge__$Vec_i16$pop(void* const vec);
int16_t __swift_bridge__$Vec_i16$get(void* const vec, uintptr_t index);
int16_t const * __swift_bridge__$Vec_i16$as_ptr(void* const vec);

void* __swift_bridge__$Vec_i32$new();
void __swift_bridge__$Vec_i32$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_i32$len(void* const vec);
void __swift_bridge__$Vec_i32$push(void* const vec, int32_t val);
int32_t __swift_bridge__$Vec_i32$pop(void* const vec);
int32_t __swift_bridge__$Vec_i32$get(void* const vec, uintptr_t index);
int32_t const * __swift_bridge__$Vec_i32$as_ptr(void* const vec);

void* __swift_bridge__$Vec_i64$new();
void __swift_bridge__$Vec_i64$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_i64$len(void* const vec);
void __swift_bridge__$Vec_i64$push(void* const vec, int64_t val);
int64_t __swift_bridge__$Vec_i64$pop(void* const vec);
int64_t __swift_bridge__$Vec_i64$get(void* const vec, uintptr_t index);
int64_t const * __swift_bridge__$Vec_i64$as_ptr(void* const vec);

void* __swift_bridge__$Vec_isize$new();
void __swift_bridge__$Vec_isize$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_isize$len(void* const vec);
void __swift_bridge__$Vec_isize$push(void* const vec, intptr_t val);
intptr_t __swift_bridge__$Vec_isize$pop(void* const vec);
intptr_t __swift_bridge__$Vec_isize$get(void* const vec, uintptr_t index);
intptr_t const * __swift_bridge__$Vec_isize$as_ptr(void* const vec);

void* __swift_bridge__$Vec_bool$new();
void __swift_bridge__$Vec_bool$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_bool$len(void* const vec);
void __swift_bridge__$Vec_bool$push(void* const vec, bool val);
bool __swift_bridge__$Vec_bool$pop(void* const vec);
bool __swift_bridge__$Vec_bool$get(void* const vec, uintptr_t index);
bool const * __swift_bridge__$Vec_bool$as_ptr(void* const vec);

// File automatically generated by swift-bridge.
#include <stdint.h>
typedef struct RustString RustString;
void __swift_bridge__$RustString$_free(void* self);
void* __swift_bridge__$RustString$new(void);
void* __swift_bridge__$RustString$new_with_str(struct RustStr str);
uintptr_t __swift_bridge__$RustString$len(void* self);
struct RustStr __swift_bridge__$RustString$trim(void* self);


