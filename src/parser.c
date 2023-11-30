// int main(void) {
//     FILE *source_fd = fopen("./examples/hello_world.astm", "r");
//     if (source_fd == NULL) {
//         printf("Error: Can not open file!");
//     }
//     char* line = NULL;
//     ssize_t read;
//     size_t len;
//     while ((read = getline(&line, &len, source_fd)) > 0) {
//         parse_line(line, read);
//     }
// 
//     fclose(source_fd);
//     free(line);
//     return 0;
// }
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <string.h>

#include "../libs/esc.h"

void trim_left(size_t* cur, char* line, size_t len) {
    while (*cur < len) {
        if (line[*cur] > 32) {
            break;
        } else {
            (*cur)++;
        }
    }
}

String* get_token(size_t* cur, char* line, size_t len) {
    trim_left(cur, line, len);
    String* nstr = str_new();
    while (*cur < len) {
        if (line[*cur] > 32) {
            str_push(nstr, line[*cur]);
            (*cur)++;
        } else {
            return nstr;
        }
    }
    return NULL;
}

void parse_line(char* line, size_t len) {
    size_t cur = 0;
    trim_left(&cur, line, len);
    if (cur >= len) return;
    String* token = get_token(&cur, line, len);
    if (token == NULL) {
        panic("Expected a token found nothing!");
    }
    printf("%s\n", token->data);
    if (strcmp(token->data, "mov") == 0) {
        token = get_token(&cur, line, len);
        printf("%s\n", token->data);
    }
    free(token);
}

