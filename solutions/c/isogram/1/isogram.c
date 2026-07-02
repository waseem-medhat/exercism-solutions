#include "isogram.h"

#include <ctype.h>
#include <stdio.h>

bool is_isogram(const char phrase[])
{
    if (!phrase) return false;
    
    bool letters[26] = {false};
    for(; *phrase != '\0'; phrase++) {
        int idx = tolower(*phrase) - 'a';
        if (idx < 0 || idx > 25) continue;
        if (letters[idx]) return false;
        letters[idx] = true;
    }
    return true;
}