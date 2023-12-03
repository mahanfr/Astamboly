#include <endian.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ESC_IMPLEMENTATION
#include "../libs/esc.h"

static char stack[1024];
static char ax[8];
static char bx[8];
static char cx[8];
static char dx[8];
static char si[8];
static char di[8];
static char bp[8];
static char sp[8];

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
} RegType;

char* RegType2Pointer(RegType type) {
    switch (type) {
        case RT_AX:
            return (char*) ax;
        case RT_BX:
            return (char*) bx;
        case RT_CX:
            return (char*) cx;
        case RT_DX:
            return (char*) dx;
        case RT_SI:
            return (char*) si;
        case RT_DI:
            return (char*) di;
        case RT_BP:
            return (char*) bp;
        case RT_SP:
            return (char*) sp;
    }
}

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

void literal_to_reg(char* bytes, long literal, uint8_t size) {
    for (int i = 0; i < 8; i++) {
        bytes[(8 - i) - 1] = (literal >> (8 * i)) & 0xFF;
    }
}

void print_bytes(char* bytes, uint8_t size) {
    for (int i = 0; i < size; i++) {
        printf("%02X ",(unsigned int) (bytes[i] & 0xFF));
    }
    printf("\n");
}

void mov(Value val1, Value val2) {
    if (val1.type == VT_REGISTER) {
        switch (val2.type) {
            case VT_LITERAL: {
                int move_size = val1.reg_size;
                char* reg = RegType2Pointer(val1.reg_type);
                literal_to_reg(reg, val2.literal, move_size);
            }
            break;
            case VT_REGISTER: {
                // TODO: Handel Sizes
                char* reg1 = RegType2Pointer(val1.reg_type);
                char* reg2 = RegType2Pointer(val2.reg_type);
                if (val2.reg_size > val1.reg_size) {
                    fprintf(stderr, "Error: cannot move from bigger to smaller register\n");
                    exit(1);
                }
                for (int i=7; i >= (8 - val2.reg_size); i--) {
                    reg1[i] = reg2[i];
                }
            }
            break;
            case VT_MEM:
                break;
            default:
                fprintf(stderr, "Error: can not move to a literal\n");
                exit(1);
        }
    } else if (val1.type == VT_MEM) {
        fprintf(stderr, "TODO: Not Implemented yet!\n");
        exit(0);
    } else {
        fprintf(stderr, "Error: can not move to a literal\n");
        exit(1);
    }
}

void print_register_state() {
    printf("ax: ");
    print_bytes((char*) ax, 8);
    printf("bx: ");
    print_bytes((char*) bx, 8);
    printf("cx: ");
    print_bytes((char*) cx, 8);
    printf("dx: ");
    print_bytes((char*) dx, 8);
    printf("si: ");
    print_bytes((char*) si, 8);
    printf("di: ");
    print_bytes((char*) di, 8);
    printf("bp: ");
    print_bytes((char*) bp, 8);
    printf("sp: ");
    print_bytes((char*) sp, 8);
}

int main(void) {
    // char bytes[4];
    // literal_to_le_bytes(bytes, 69420, 4);
    // print_bytes(bytes, 4);
    // print_register_state();
    mov(reg(RT_AX, 4), literal(INT32_MAX));
    mov(reg(RT_BX, 4), reg(RT_AX, 2));
    print_register_state();
    return 0;
}
