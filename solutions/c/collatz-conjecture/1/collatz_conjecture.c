#include "collatz_conjecture.h"
#include <math.h>

#define SIZE 1000000

int MEM_STEPS[SIZE] = {0};

int steps(int start)
{
    if (start < 1) return -1;
    
    int current_mem[SIZE] = {0};
    int n_steps = 0;
    int num = start;

    while (num > 1) {
        if (MEM_STEPS[num] > 0) {
            n_steps += MEM_STEPS[start];
            break;
        }
        num = (num % 2 == 0)
            ? num / 2
            : 3 * num + 1;
        n_steps++;
    }

    for (int i = 0; i < SIZE; i++) {
        MEM_STEPS[i] = current_mem[i];
    }

    return n_steps;
}
