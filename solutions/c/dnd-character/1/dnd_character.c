#include "dnd_character.h"

#include <stdlib.h>
#include <math.h>

int ability()
{
    int dice[4] = {0, 0, 0, 0};
    int smallest_die_idx = 0;
    for (int i = 0; i < 4; i++) {
        int die = (rand() % 6) + 1;
        dice[i] = die;
        if (i == 0 || die < dice[smallest_die_idx]) {
          smallest_die_idx = i;
        }
    }

  int sum = 0;
  for (int i = 0; i < 4; i++) {
    if (i != smallest_die_idx) {
      sum += dice[i];
    }
  }

  return sum;
}

int modifier(int score)
{
    float modifier_f = (score - 10) / 2.0;
    return (int)floorf(modifier_f);
}

dnd_character_t make_dnd_character(void)
{
    dnd_character_t character = {
        .strength = ability(),
        .dexterity = ability(),
        .constitution = ability(),
        .intelligence = ability(),
        .wisdom = ability(),
        .charisma = ability(),
    };
    character.hitpoints = modifier(character.constitution) + 10;
    return character;
}