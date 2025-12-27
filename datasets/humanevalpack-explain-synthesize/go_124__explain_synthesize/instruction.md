# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
    "strconv"
    "strings"
)

// You have to write a function which validates a given date string and
// returns true if the date is valid otherwise false.
// The date is valid if all of the following rules are satisfied:
// 1. The date string is not empty.
// 2. The number of days is not less than 1 or higher than 31 days for months 1,3,5,7,8,10,12. And the number of days is not less than 1 or higher than 30 days for months 4,6,9,11. And, the number of days is not less than 1 or higher than 29 for the month 2.
// 3. The months should not be less than 1 or higher than 12.
// 4. The date should be in the format: mm-dd-yyyy
// 
// for example:
// ValidDate('03-11-2000') => true
// 
// ValidDate('15-01-2012') => false
// 
// ValidDate('04-0-2040') => false
// 
// ValidDate('06-04-2020') => true
// 
// ValidDate('06/04/2020') => false
func ValidDate(date string) bool {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
