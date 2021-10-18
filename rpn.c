#include <stdio.h>

int INSERT = 1, ROLL = 2;
int action = 0;
int l = 0, x = 0, y = 0, z = 0, t = 0;
int decimals = 2;
int reg = 0;

void show() {
    printf("lstx: %i\nx: %i y: %i z: %i t: %i\n", l, x, y, z, t);
}

void insert(int c) {
    if (action == ROLL) {
        t = z;
        z = y;
        y = x;
    }
    if (action != INSERT) {
        action = INSERT;
        x = 0;
    }

    //x = parseFloat(x.toString() + c.toString());
    x = c;
    show();
}

void enter() {
    action = 0;
    t = z;
    z = y;
    y = x;
    show();
}

void add() {
    action = ROLL;
    l = x;
    x = y + x;
    y = z;
    z = t;
    show();
}

void sub() {
    action = ROLL;
    l = x;
    x = y - x;
    y = z;
    z = t;
    show();
}

void mul() {
    action = ROLL;
    l = x;
    x = y * x;
    y = z;
    z = t;
    show();
}

void div() {
    action = ROLL;
    l = x;
    x = y / x;
    y = z;
    z = t;
    show();
}

void swap() {
    action = ROLL;
    int current = x;
    x = y;
    y = current;
    show();
}

void rotate() {
    action = ROLL;
    int current = x;
    x = y;
    y = z;
    z = t;
    t = current;
    show();
}

void clx() {
    action = 0;
    x = 0;
    show();
}

void chs() {
    x = -1 * x;
    show();
}

void lstx() {
    action = ROLL;
    t = z;
    z = y;
    y = x;
    x = l;
    show();
}

void percent() {
    action = ROLL;
    l = x;
    x = x / 100 * y;
    show();
}

void sto() {
    action = ROLL;
    reg = x;
    show();
}

void rcl() {
    if (action == ROLL || action == INSERT) {
        t = z;
        z = y;
        y = x;
    }
    action = ROLL;
    x = reg;
    show();
}

int main() {
    x = 2;
    enter();
    x = 3;
    show();
    swap();
    show();
    mul();
    show();
    rotate();
    rotate();
    x = 8;
    chs();
    show();
    clx();
    lstx();
    chs();
    show();

    return 0;
}
