from typing import List, Dict, Tuple, Optional, Any, Union
import math



def strlen(string: str) -> int:
    """ Return length of given string
    >>> strlen('')
    0
    >>> strlen('abc')
    3
    """

    return len(string)







def check(strlen):
    assert strlen('') == 0
    assert strlen('x') == 1
    assert strlen('asdasnakj') == 9

check(strlen)

# Execution Trigger
if __name__ == '__main__':
    check(strlen)
