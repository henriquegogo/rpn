#include <stdio.h>

int lastx = 0;
int stack[4] = {0, 0, 0, 0};

void enter() {
    stack[3] = stack[2];
    stack[2] = stack[1];
    stack[1] = stack[0];
}

void add() {
    lastx = stack[0];
    stack[0] = stack[1] + stack[0];
    stack[2] = stack[3];
    stack[1] = stack[2];
}

void sub() {
    lastx = stack[0];
    stack[0] = stack[1] - stack[0];
    stack[2] = stack[3];
    stack[1] = stack[2];
}

void mul() {
    lastx = stack[0];
    stack[0] = stack[1] * stack[0];
    stack[2] = stack[3];
    stack[1] = stack[2];
}

void div() {
    lastx = stack[0];
    stack[0] = stack[1] / stack[0];
    stack[2] = stack[3];
    stack[1] = stack[2];
}

void swap() {
    int x = stack[0];
    stack[0] = stack[1];
    stack[1] = x;
}

void rotate() {
    int x = stack[0];
    stack[0] = stack[1];
    stack[1] = stack[2];
    stack[2] = stack[3];
    stack[3] = x;
}

void lstx() {
    stack[3] = stack[2];
    stack[2] = stack[1];
    stack[1] = stack[0];
    stack[0] = lastx;
}

void clx() {
    stack[0] = 0;
}

void chs() {
    stack[0] = -1 * stack[0];
}

void showvars() {
    printf("lstx: %i\nx: %i y: %i z: %i t: %i\n", lastx, stack[0], stack[1], stack[2], stack[3]);
}

int main() {
    stack[0] = 2;
    enter();
    stack[0] = 3;
    showvars();
    swap();
    showvars();
    mul();
    showvars();
    rotate();
    rotate();
    stack[0] = 8;
    chs();
    showvars();
    clx();
    lstx();
    chs();
    showvars();

    return 0;
}
