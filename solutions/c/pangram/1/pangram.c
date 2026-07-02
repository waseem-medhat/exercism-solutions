#include "pangram.h"

#include <stdbool.h>
#include <ctype.h>

bool is_pangram(const char *sentence)
{
    if (!sentence) return false;
    int letter_counts[26] = {0};
    for (; *sentence != '\0'; sentence++) {
        int letter_i = tolower(*sentence) - 'a';
        if (letter_i >= 26) continue;
        letter_counts[letter_i]++;
    }

    for (int i = 0; i < 26; i++) {
        if (letter_counts[i] == 0) return false;
    }
    return true;
}