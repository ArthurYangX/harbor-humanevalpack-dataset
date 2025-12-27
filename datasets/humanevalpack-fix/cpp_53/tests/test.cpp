#include <bits/stdc++.h>
#include <cassert>
using namespace std;

// Helper function often used in HumanEvalPack tests
template<typename T> bool issame(T a, T b) { return a == b; }


// Prompt (Function Signature)
/*
Add two numbers x and y
>>> add(2, 3)
5
>>> add(5, 7)
12
*/
#include<stdio.h>
#include<stdlib.h>
using namespace std;
int add(int x,int y){


// Solution Body
    return x+y;
}


// Test Code (includes main)
#undef NDEBUG
#include<assert.h>
int main(){
    assert (add(0, 1) == 1);
    assert (add(1, 0) == 1);
    assert (add(2, 3) == 5);
    assert (add(5, 7) == 12);
    assert (add(7, 5) == 12);
    for (int i=0;i<100;i+=1)
    {
        int x=rand()%1000;
        int y=rand()%1000;
        assert (add(x, y) == x + y);
    }
}

