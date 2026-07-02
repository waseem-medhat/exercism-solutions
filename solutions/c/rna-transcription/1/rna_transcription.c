#include "rna_transcription.h"

#include<stdlib.h>
#include<string.h>

char *to_rna(const char *dna)
{
    char *rna_result = malloc(sizeof(char) * (1 + strlen(dna)));
    char *rna = rna_result;
    for (; *dna != '\0'; dna++, rna++) {
        switch (*dna) {
            case 'G':
                *rna = 'C';
                break;
            case 'C':
                *rna = 'G';
                break;
            case 'T':
                *rna = 'A';
                break;
            case 'A':
                *rna = 'U';
                break;
         }
    }
    *rna = '\0';
    return rna_result;
}