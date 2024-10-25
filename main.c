#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    printf("Testing malloc...\n");
    int* arr = (int*) malloc(5 * sizeof(int));
    if (arr == NULL) {
        printf("malloc failed\n");
        return 1;
    }

    for (int i = 0; i < 5; i++) {
        arr[i] = i * 10;
        printf("arr[%d] = %d\n", i, arr[i]);
    }

    printf("Testing calloc...\n");
    int* arr2 = (int*) calloc(5, sizeof(int));
    if (arr2 == NULL) {
        printf("calloc failed\n");
        free(arr);
        return 1;
    }

    for (int i = 0; i < 5; i++) {
        printf("arr2[%d] = %d\n", i, arr2[i]);
    }

    printf("Testing realloc...\n");
    arr = (int*) realloc(arr, 10 * sizeof(int));
    if (arr == NULL) {
        printf("realloc failed\n");
        free(arr2);
        return 1;
    }

    for (int i = 5; i < 10; i++) {
        arr[i] = i * 10;
        printf("arr[%d] = %d\n", i, arr[i]);
    }

    printf("Testing free...\n");
    free(arr);
    free(arr2);

    printf("Memory operations completed successfully.\n");
    return 0;
}