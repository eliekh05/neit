
#include <stdio.h>
#include <stdlib.h>
int strcmp(const char *str1, const char *str2) {
    while (*str1 != '\0' && *str2 != '\0') {
        if (*str1 != *str2) {
            return (unsigned char)(*str1) - (unsigned char)(*str2);
        }
        str1++;
        str2++;
    }
    return (unsigned char)(*str1) - (unsigned char)(*str2);
}
void __NCLRSCRN__() {
    #if defined(_WIN32) || defined(_WIN64)
        if (system("cls") == -1) {
            perror("Error clearing screen on Windows");
        }
    #elif defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
        if (system("clear") == -1) {
            perror("Error clearing screen on Unix-like platform");
        }
    #else
        printf("\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n");
    #endif
    fflush(stdout);
}
