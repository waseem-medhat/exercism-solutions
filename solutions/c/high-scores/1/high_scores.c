#include "high_scores.h"

#include<string.h>
#include<stdio.h>

int32_t latest(const int32_t *scores, size_t scores_len)
{
    return *(scores + scores_len - 1);
}

int32_t personal_best(const int32_t *scores, size_t scores_len)
{
    int32_t best = 0;
    for(size_t i = 0; i < scores_len; i++) {
        if (*(scores + i) > best) {
            best = *(scores + i);
        }
    }
    return best;
}

size_t personal_top_three(
    const int32_t *scores, size_t scores_len, int32_t *output
)
{
    memset(output, 0, 4);
    for(size_t i = 0; i < scores_len; i++) {
        int32_t score = *(scores + i);
        if (score > output[0]) {
            output[2] = output[1];
            output[1] = output[0];
            output[0] = score;
        } else if (score > output[1]) {
            output[2] = output[1];
            output[1] = score;
        } else if (score > output[2]) {
            output[2] = score;
        }
    }
    
    int n_scores = 0;
    for (int i = 0; output[i] != 0 && i < 3; i++) {
        n_scores++;
    }
    return n_scores;
}