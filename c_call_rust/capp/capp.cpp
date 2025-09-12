#include <stdio.h>
#include "rustlib.h"

int main() {
    MyStruct s = { 10, 20, 0 };
    process(&s);
    printf("Result from Rust: %d = %d + %d\n", s.result, s.a, s.b);
    return 0;
}