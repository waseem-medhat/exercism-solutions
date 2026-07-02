#include "resistor_color_trio.h"
#include <math.h>
#include <stdio.h>

resistor_value_t color_code(resistor_band_t bands[])
{    
    int val, n_zeroes;
    if (bands[1] == BLACK) {
        val = bands[0];
        n_zeroes = 1 + bands[2];
    } else {
        val = (10 * bands[0]) + bands[1];
        n_zeroes = bands[2];
    }
    
    resistor_unit_t unit = OHMS;
    switch (n_zeroes / 3) {
        case 1:
            unit = KILOOHMS;
            break;
        case 2:
            unit = MEGAOHMS;
            break;
        case 3:
            unit = GIGAOHMS;
            break;
    }


    int multiplier = (int)pow(10, n_zeroes % 3);
    resistor_value_t resistor_val = {
        .value = val * multiplier,
        .unit = unit
    };
    return resistor_val;
}