package main

import (
    "testing"
    "github.com/stretchr/testify/assert"
    "sort"
)


// Prompt
// Return sorted Unique elements in a list
// >>> Unique([5, 3, 5, 2, 3, 3, 9, 0, 123])
// [0, 2, 3, 5, 9, 123]
func Unique(l []int) []int {
// Solution
set := make(map[int]interface{})
	for _, i := range l {
		set[i]=nil
	}
	l = make([]int,0)
	for i, _ := range set {
		l = append(l, i)
	}
	sort.Ints(l)
	return l
}

// Test Code
func TestUnique(t *testing.T) {
    assert := assert.New(t)
    assert.Equal([]int{0, 2, 3, 5, 9, 123}, Unique([]int{5, 3,5, 2, 3, 3, 9, 0, 123}))
}
