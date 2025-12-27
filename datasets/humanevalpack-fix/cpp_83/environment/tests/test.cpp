#include <bits/stdc++.h>
#include <cassert>
using namespace std;

// Helper function often used in HumanEvalPack tests
template<typename T> bool issame(T a, T b) { return a == b; }


// Prompt (Function Signature)
/*
Given a positive integer n, return the count of the numbers of n-digit
positive integers that start or end with 1.
*/
#include<stdio.h>
using namespace std;
int starts_one_ends(int n){


// Solution Body
    if (n<1) return 0;
    if (n==1) return 1;
    int out=18;
    for (int i=2;i<n;i++)
        out=out*10;
    return out;
}


// Test Code (includes main)
#undef NDEBUG
#include<assert.h>
int main(){
    assert (starts_one_ends(1) == 1);
    assert (starts_one_ends(2) == 18);
    assert (starts_one_ends(3) == 180);
    assert (starts_one_ends(4) == 1800);
    assert (starts_one_ends(5) == 18000);
}

