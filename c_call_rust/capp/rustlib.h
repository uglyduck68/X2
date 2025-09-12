#pragma once
#ifndef RUSTLIB_H
#define RUSTLIB_H

typedef struct {
    int a;
    int b;
    int result;
} MyStruct;

extern "C" {
	void process(MyStruct* s);
}

#endif