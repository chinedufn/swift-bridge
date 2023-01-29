#include <stdint.h>
typedef struct RustString RustString;
void __swift_bridge__$RustString$_free(void* self);

void* __swift_bridge__$Vec_RustString$new(void);
void __swift_bridge__$Vec_RustString$drop(void* vec_ptr);
void __swift_bridge__$Vec_RustString$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_RustString$pop(void* vec_ptr);
void* __swift_bridge__$Vec_RustString$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_RustString$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_RustString$len(void* vec_ptr);
void* __swift_bridge__$Vec_RustString$as_ptr(void* vec_ptr);

void* __swift_bridge__$RustString$new(void);
void* __swift_bridge__$RustString$new_with_str(struct RustStr str);
uintptr_t __swift_bridge__$RustString$len(void* self);
struct RustStr __swift_bridge__$RustString$as_str(void* self);
struct RustStr __swift_bridge__$RustString$trim(void* self);
bool __swift_bridge__equality_operator_for_RustStr(struct RustStr lhs, struct RustStr rhs);