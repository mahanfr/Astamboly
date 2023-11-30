#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ESC_IMPLEMENTATION
#include "../libs/esc.h"

const char stack[1024];
const char ax[4];
const char bx[4];
const char cx[4];
const char dx[4];
const char si[4];
const char di[4];
const char bp[4];
const char sp[4];
const char r8[4];
const char r9[4];
const char r10[4];

enum ValueType {
    VT_NONE,
    VT_REGISTER,
    VT_MEM,
    VT_LITERAL,
};

typedef enum {
    RT_AX,
    RT_BX,
    RT_CX,
    RT_DX,
    RT_SI,
    RT_DI,
    RT_BP,
    RT_SP,
    RT_R8,
    RT_R9,
    RT_R10,
} RegType;

typedef struct {
    enum ValueType type;
    long literal;
    uint64_t mem_offset;
    RegType reg_type;
    uint8_t reg_size;
} Value;

Value literal(long number) {
    Value value;
    value.type = VT_LITERAL;
    value.literal = number;
    return value; 
}

Value reg(RegType rtp, uint8_t size) {
    Value value;
    value.type = VT_REGISTER;
    value.reg_type = rtp;
    value.reg_size = size;
    return value; 
}

Value mem(uint64_t offset) {
    Value value;
    value.type = VT_MEM;
    value.mem_offset = offset;
    return value;
}

void mov(Value val1, Value val2) {
    printf("mov %d , %d\n", val1.type, val2.type);
}

int main(void) {
    mov(mem(0), literal(69));
    return 0;
}
