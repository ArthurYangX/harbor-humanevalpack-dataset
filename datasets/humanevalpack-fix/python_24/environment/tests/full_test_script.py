from typing import List, Dict, Tuple, Optional, Any, Union
import math



def largest_divisor(n: int) -> int:
    """ For a given number n, find the largest number that divides n evenly, smaller than n
    >>> largest_divisor(15)
    5
    """

    for i in reversed(range(n)):
        if n % i == 0:
            return i







def check(largest_divisor):
    assert largest_divisor(3) == 1
    assert largest_divisor(7) == 1
    assert largest_divisor(10) == 5
    assert largest_divisor(100) == 50
    assert largest_divisor(49) == 7

check(largest_divisor)

# Execution Trigger
if __name__ == '__main__':
    check(largest_divisor)
