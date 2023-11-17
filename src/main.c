#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ESC_IMPLEMENTATION
#include "../libs/esc.h"

#define STRING_DEFAULT_SIZE 4

typedef struct {
    char* data;
    size_t len;
    size_t size;
} String;

String* str_new() {
    String* str = (String*) malloc(sizeof(String));
    str->data = malloc(STRING_DEFAULT_SIZE * sizeof(char));
    str->len = 0;
    str->size = STRING_DEFAULT_SIZE;
    return str;
}

void str_push(String *str, char ch) {
    str->data[str->len] = ch;
    str->len++;
    if (str->len > str->size - 2) {
        if (!realloc(str->data, str->size * 2)) {
            fprintf(stderr, "Error: Rellocating string buffer!\n");
            return;
        }
        str->size = str->size * 2;
    }
}

typedef struct {
    char* opcode;
    char* operators[];
} Instruct;

int main(void) {
    FILE *source_fd = fopen("./examples/hello_world.astm", "r");
    char* buffer = NULL;
    int len = esc_readfile(source_fd, &buffer);
    if (len < 0) {
        fprintf(stderr, "Error: Unable to read to file!");
    }
    //printf("%s\n", buffer);

    // String* str = str_new();
    // printf("%s", str->data);

    fclose(source_fd);
    free(buffer);
    return 0;
}
