# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a positive integer, obtain its roman numeral equivalent as a string,
    and return it in lowercase.
    Restrictions: 1 <= num <= 1000

    Examples:
    >>> intToMiniRoman(19) == "xix"
    >>> intToMiniRoman(152) == "clii"
    >>> intToMiniRoman(426) == "cdxxvi"
     */
    public String intToMiniRoman(int number) {
        List<Integer> num = Arrays.asList(1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000);
        List<String> sym = Arrays.asList("I", "IV", "V", "IX", "X", "XL", "L", "XC", "C", "CD", "D", "CM", "M");
        int i = 12;
        String res = "";
        while (number > 0) {
            int div = number / num.get(i);
            number %= num.get(i);
            while (div != 0) {
                res += sym.get(i);
                div -= 1;
            }
            i -= 1;
        }
        return res.toLowerCase();
    }
}
```

# Instruction

Explain the algorithm in natural language:
- Describe the core idea and step-by-step approach
- Specify inputs/outputs and edge cases
- Provide time/space complexity

Constraints:
- Output must be plain text only
- Do NOT output any code
- Write your explanation to `solution/explanation.txt`

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a positive integer, obtain its roman numeral equivalent as a string,
    and return it in lowercase.
    Restrictions: 1 <= num <= 1000

    Examples:
    >>> intToMiniRoman(19) == "xix"
    >>> intToMiniRoman(152) == "clii"
    >>> intToMiniRoman(426) == "cdxxvi"
     */
    public String intToMiniRoman(int number) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
