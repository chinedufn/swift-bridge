#include <stdint.h>

//int16_t foo(void);
void run_string_tests(void);

typedef struct __ARustStack __ARustStack;

typedef struct OwnedPtrToRust {
    struct __ARustStack *ptr
} OwnedPtrToRust;

typedef struct RefPtrToRust {
    struct __ARustStacl *ptr
} RefPtrToRust;

OwnedPtrToRust swift_bridge$unstable$ARustStruct$new(void);
void swift_bridge$unstable$ARustStruct$free(OwnedPtrToRust);

void swift_bridge$unstable$ARustStruct$push(RefPtrToRust, uint8_t);
void swift_bridge$unstable$ARustStruct$pop(RefPtrToRust);
const uint8_t* swift_bridge$unstable$ARustStruct$as_ptr(RefPtrToRust);
uintptr_t swift_bridge$unstable$ARustStruct$len(RefPtrToRust);

