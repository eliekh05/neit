#include <stdio.h>
#include <string.h>
int fdi(int a, int b);
double fdf(double a, double b);
int main() {
    char name[337] = "joy";
    fgets(name, sizeof(name) - 1, stdin);
    size_t len_name = strcspn(name, "\n");
    name[len_name] = '\0';
    if (name == "joy") {
        printf("hi\n");
    }
    return 0;
}
