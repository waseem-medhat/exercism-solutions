#include "queen_attack.h"
#include <stdbool.h>

attack_status_t can_attack(position_t queen1, position_t queen2)
{
    uint8_t r1 = queen1.row;
    uint8_t c1 = queen1.column;
    uint8_t r2 = queen2.row;
    uint8_t c2 = queen2.column;

    if (r1 > 7 || c1 > 7 || r2 > 7 || c2 > 7)
        return INVALID_POSITION;
    if (r1 == r2 && c1 == c2)
        return INVALID_POSITION;

    bool is_vertical = c1 == c2;
    bool is_horizontal = r1 == r2;
    uint8_t r_hi = r1 > r2 ? r1 : r2;
    uint8_t r_lo = r1 > r2 ? r2 : r1;
    uint8_t c_hi = c1 > c2 ? c1 : c2;
    uint8_t c_lo = c1 > c2 ? c2 : c1;
    bool is_diagonal = (c_hi - c_lo) == (r_hi - r_lo);

    return (is_vertical || is_horizontal || is_diagonal)
        ? CAN_ATTACK
        : CAN_NOT_ATTACK;
}