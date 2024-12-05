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
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main() {
char name[2048];
scanf("%2047[^\n]", name);
    printf("%s\n", name);
    fflush(stdout);
if (strcmp(name, "joy") == 0){
    printf("Hello boss\n");
    fflush(stdout);

}

    return 0;
}
