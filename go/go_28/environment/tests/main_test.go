package main

import (
    "testing"
    "github.com/stretchr/testify/assert"
)


// Prompt
// Concatenate list of strings into a single string
// >>> Concatenate([])
// ''
// >>> Concatenate(['a', 'b', 'c'])
// 'abc'
func Concatenate(strings []string) string {
// Solution
if len(strings) == 0 {
		return ""
	}
	return strings[0] + Concatenate(strings[1:])
}

// Test Code
func TestConcatenate(t *testing.T) {
    assert := assert.New(t)
    assert.Equal("", Concatenate([]string{}))
    assert.Equal("xyz", Concatenate([]string{"x", "y", "z"}))
    assert.Equal("xyzwk", Concatenate([]string{"x", "y","z", "w", "k"}))
}
