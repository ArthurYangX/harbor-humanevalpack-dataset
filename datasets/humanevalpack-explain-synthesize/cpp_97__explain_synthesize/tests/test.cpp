#include <bits/stdc++.h>
#include <cassert>
using namespace std;

// Helper function often used in HumanEvalPack tests
template<typename T> bool issame(T a, T b) { return a == b; }


// Prompt (Function Signature)
/*
Complete the function that takes two integers and returns 
the product of their unit digits.
Assume the input is always valid.
Examples:
multiply(148, 412) should return 16.
multiply(19, 28) should return 72.
multiply(2020, 1851) should return 0.
multiply(14,-15) should return 20.
*/
#include<stdio.h>
#include<math.h>
using namespace std;
int multiply(int a,int b){


// Solution Body
    return (abs(a)%10)*(abs(b)%10);
}


// Test Code (includes main)
#undef NDEBUG
#include<assert.h>
int main(){
    assert (multiply(148, 412) == 16                   );
    assert (multiply(19, 28) == 72       );
    assert (multiply(2020, 1851) == 0);
    assert (multiply(14,-15) == 20   );
    assert (multiply(76, 67) == 42    );
    assert (multiply(17, 27) == 49    );
    assert (multiply(0, 1) == 0);
    assert (multiply(0, 0) == 0);
}

