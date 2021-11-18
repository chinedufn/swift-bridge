#include <stdint.h>

void run_string_tests(void);
void run_opaque_swift_class_tests(void);

typedef struct OwnedPtrToRust {
    struct __ARustStack *ptr
} OwnedPtrToRust;

typedef struct RefPtrToRust {
    struct __ARustStacl *ptr
} RefPtrToRust;