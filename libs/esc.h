/*
 *  Extended Standard C
 *  Implements frequently used functions
 *  License MIT under Mahan.farzaneh2@gmail.com
 * */

#ifndef ESC_H_
#define ESC_H_

#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <string.h>

#define STRING_DEFAULT_SIZE 128

typedef struct {
    char* data;
    size_t len;
    size_t size;
} String;

int esc_readfile(FILE *fd, char** buffer);

void panic(char* err);
String* str_new();
void str_push(String *str, char ch);
char* mystd_strcat_null(char* str, ...);


#define esc_strcat(str,...) esc_strcat_null(str, __VA_ARGS__, NULL)
#endif

#ifdef ESC_IMPLEMENTATION
int esc_readfile(FILE *fd, char** buffer) {
    if (fd == NULL) {
        fprintf(stderr, "Error: Not able to open file!");
        return -1;
    }
    fseek(fd, 0L, SEEK_END);
    size_t len = ftell(fd) + 1;
    fseek(fd, 0L, SEEK_SET);
    *buffer = malloc(sizeof(char) * (len) + 1);
    fread(*buffer, len, 1, fd);
    return len;
}
char* esc_strcat_null(char* str, ...) {
    size_t str_size = 1;
    va_list ptr;

    str_size += strlen(str);
    va_start(ptr,str);
    char* arg;
    while ((arg = va_arg(ptr, char*)) != NULL) {
        str_size += strlen(arg);
    }
    va_end(ptr);
    char *res = (char*) malloc(str_size);

    strcpy(res, str);
    va_start(ptr,str);
    while ((arg = va_arg(ptr, char*)) != NULL) {
        strcat(res,arg);
    }
    va_end(ptr);
    return res;
}
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

void panic(char* err) {
    fprintf(stderr,"%s\n",err);
    exit(-1);
}
#endif
