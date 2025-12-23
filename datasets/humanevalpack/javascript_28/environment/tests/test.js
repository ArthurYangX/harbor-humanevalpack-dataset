const assert = require('assert');

// Solution
/* Concatenate list of strings into a single string
  >>> concatenate([])
  ''
  >>> concatenate(['a', 'b', 'c'])
  'abc'
  */
const concatenate = (strings) => {

  return strings.join('');
}



// Test Code
const testConcatenate = () => {
  console.assert(concatenate([]) === '')
  console.assert(concatenate(['x', 'y', 'z']) === 'xyz')
  console.assert(concatenate(['x', 'y', 'z', 'w', 'k']) === 'xyzwk')
}

testConcatenate()
