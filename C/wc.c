#include <ctype.h>
#include <stdio.h>


void print_error(char *error) {
    fprintf(stderr, "%s\n", error);
    fprintf(stderr, "Usage: wc [FILE]\n");
}


int parse_args(int argc, char** argv, char** filename) {
    if (1 == argc) {
        *filename = NULL; 
        return 0;
    }
    if (2 == argc) {
        *filename = argv[1];
        return 0;
    }
    return 1;
}


FILE *open_file(char *filename) {
    if (filename) {
        return fopen(filename, "r");
    }
    return stdin;
}


void wc(FILE *file, size_t *lines, size_t *words, size_t *bytes) {
    int c;
    char is_word = 0;

    *lines = 0;
    *words = 0;
    *bytes = 0;

    while (EOF != (c = fgetc(file))) {
        if ('\n' == c) {
            (*lines)++;
        }
        if (isspace(c)) {
            is_word = 0;
        } else if (0 == is_word) {
            (*words)++;
            is_word = 1;
        }

        (*bytes)++;
    }
}


int main(int argc, char** argv) {
    char *filename = NULL;
    if (parse_args(argc, argv, &filename)) {
        print_error("Error parsing filename");
        return 1;
    }

    FILE *file = open_file(filename);
    if (file == NULL) {
        print_error("Error opening file");
        return 1;
    }

    size_t words, lines, bytes;
    wc(file, &words, &lines, &bytes);

    printf("%ld\t%ld\t%ld\t%s\n", words, lines, bytes, filename);

    fclose(file);
}
