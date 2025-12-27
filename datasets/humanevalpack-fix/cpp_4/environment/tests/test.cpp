#include <bits/stdc++.h>
#include <cassert>
using namespace std;

// Helper function often used in HumanEvalPack tests
template<typename T> bool issame(T a, T b) { return a == b; }


// Prompt (Function Signature)
/*
For a given vector of input numbers, calculate Mean Absolute Deviation
around the mean of this dataset.
Mean Absolute Deviation is the average absolute difference between each
element and a centerpoint (mean in this case):
MAD = average | x - x_mean |
>>> mean_absolute_deviation({1.0, 2.0, 3.0, 4.0})
1.0
*/
#include<stdio.h>
#include<math.h>
#include<vector>
using namespace std;
float mean_absolute_deviation(vector<float> numbers){


// Solution Body
    float sum=0;
    float avg,msum,mavg;
    int i=0;
    for (i=0;i<numbers.size();i++)
        sum+=numbers[i];
    avg=sum/numbers.size();
    msum=0;
    for (i=0;i<numbers.size();i++)
        msum+=abs(numbers[i]-avg);
    return msum/numbers.size();
}


// Test Code (includes main)
#undef NDEBUG
#include<assert.h>
int main(){
    assert (abs(mean_absolute_deviation({1.0, 2.0, 3.0}) - 2.0/3.0) < 1e-4);
    assert (abs(mean_absolute_deviation({1.0, 2.0, 3.0, 4.0}) - 1.0) < 1e-4);
    assert (abs(mean_absolute_deviation({1.0, 2.0, 3.0, 4.0, 5.0}) - 6.0/5.0) < 1e-4);
}
