#include "robot_simulator.h"

#include <stdio.h>

robot_status_t robot_create(robot_direction_t direction, int x, int y)
{
    robot_position_t position = {
        .x = x,
        .y = y
    };
    robot_status_t status = {
        .position = position,
        .direction = direction
    };
    return status;
}

void robot_move(robot_status_t *robot, const char *commands)
{
    for (; *commands != '\0'; commands++) {
        switch (*commands) {
            case 'R':
            case 'L':
                robot->direction = rotate(robot->direction, *commands);
                break;
            case 'A':
                robot->position = advance(robot->position, robot->direction);
                break;
            default:
                break;
        }
    }

    printf("%p", (void *)robot);
    printf("%s", commands);
}

robot_direction_t rotate(robot_direction_t direction, char command)
{
    if (command == 'R') {
        switch (direction) {
            case DIRECTION_NORTH:
                return DIRECTION_EAST;
            case DIRECTION_EAST:
                return DIRECTION_SOUTH;
            case DIRECTION_SOUTH:
                return DIRECTION_WEST;
            case DIRECTION_WEST:
                return DIRECTION_NORTH;
            case DIRECTION_MAX:
                return direction;
        }
    }
    if (command == 'L') {
        switch (direction) {
            case DIRECTION_NORTH:
                return DIRECTION_WEST;
            case DIRECTION_WEST:
                return DIRECTION_SOUTH;
            case DIRECTION_SOUTH:
                return DIRECTION_EAST;
            case DIRECTION_EAST:
                return DIRECTION_NORTH;
            case DIRECTION_MAX:
                return direction;
        }
    }
    return direction;
}

robot_position_t advance(robot_position_t position, robot_direction_t direction)
{
    int x = position.x;
    int y = position.y;
    switch (direction) {
        case DIRECTION_NORTH:
            y++;
            break;
        case DIRECTION_SOUTH:
            y--;
            break;
        case DIRECTION_EAST:
            x++;
            break;
        case DIRECTION_WEST:
            x--;
        case DIRECTION_MAX:
            break;
    }
    robot_position_t new_position = {
        .x = x,
        .y = y
    };
    return new_position;
}