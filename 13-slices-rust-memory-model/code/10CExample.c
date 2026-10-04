#include <stdio.h>

int main() {
    int numbers[] = {10, 20, 30, 40, 50};

    int *part = &numbers[1];
    int length = 3;

    for (int i = 0; i < length; i++) {
        printf("%d ", part[i]);
    }

    return 0;
}
