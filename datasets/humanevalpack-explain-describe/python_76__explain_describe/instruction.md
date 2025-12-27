# Context

You are given a reference implementation (canonical solution) to explain.

```python
def is_simple_power(x, n):
    if (n == 1): 
        return (x == 1) 
    power = 1
    while (power < x): 
        power = power * n 
    return (power == x)
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


def is_simple_power(x, n):
    """Your task is to write a function that returns true if a number x is a simple
    power of n and false in other cases.
    x is a simple power of n if n**int=x
    For example:
    is_simple_power(1, 4) => true
    is_simple_power(2, 2) => true
    is_simple_power(8, 2) => true
    is_simple_power(3, 2) => false
    is_simple_power(3, 1) => false
    is_simple_power(5, 3) => false
    """


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
