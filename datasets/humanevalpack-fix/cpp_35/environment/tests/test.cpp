#include <bits/stdc++.h>
#include <cassert>
using namespace std;

// Helper function often used in HumanEvalPack tests
template<typename T> bool issame(T a, T b) { return a == b; }


// Prompt (Function Signature)
/*
Return maximum element in the vector.
>>> max_element({1, 2, 3})
3
>>> max_element({5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10})
123
*/
#include<stdio.h>
#include<math.h>
#include<vector>
using namespace std;
float max_element(vector<float> l){


// Solution Body
  float max=-10000;
  for (int i=0;i<l.size();i++)
  if (max<l[i]) max=l[i];
  return max;

}


// Test Code (includes main)
#undef NDEBUG
#include<assert.h>
int main(){
    assert (abs(max_element({1, 2, 3})- 3)<1e-4);
    assert (abs(max_element({5, 3, -5, 2, -3, 3, 9, 0, 124, 1, -10})- 124)<1e-4);
}

