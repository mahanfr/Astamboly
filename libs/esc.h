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

char* mystd_strcat_null(char* str, ...);

int esc_readfile(FILE *fd, char** buffer);

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
#endif
