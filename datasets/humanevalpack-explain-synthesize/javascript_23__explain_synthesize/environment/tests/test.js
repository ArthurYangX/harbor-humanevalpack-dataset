const assert = require('assert');

// Solution
/* Return length of given string
  >>> strlen('')
  0
  >>> strlen('abc')
  3
  */
const strlen = (string) => {

  return string.length;
}



// Test Code
const testStrlen = () => {
  console.assert(strlen('') === 0)
  console.assert(strlen('x') === 1)
  console.assert(strlen('asdasnakj') === 9)
}

testStrlen()
