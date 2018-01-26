"""
Check if a string of digits can be converted into a true mathematical
expression.
"""

from fractions import Fraction


def memoize(func):
    """ Memoization decorator for functions taking one or more arguments. """
    class Memodict(dict):
        """The memoized function"""
        def __init__(self, f):
            super().__init__()
            self.func = f

        def __call__(self, *args):
            return self[args]

        def __missing__(self, key):
            ret = self[key] = self.func(*key)
            return ret
    return Memodict(func)


def chunks(string):
    """Iterate over all possible ordered partitions of a string."""
    if string:
        for partit in chunks(string[1:]):
            if partit:
                yield (string[0] + partit[0],) + partit[1:]
            yield (string[0],) + partit
    else:
        yield ()


def all_equal(elems):
    """Checks if all elements of an iterator are equal"""
    first = None
    for elem in elems:
        if first is None:
            first = elem
            continue
        if elem != first:
            return False
    return True


def insert_binary(elems, bin_func):
    """
    Takes a list of elements and returns all the possible lists of elements
    after reducing a consecutive pair with the binary function
    """
    for k in range(1, len(elems)):
        try:
            yield (elems[:k - 1] +
                   (bin_func(elems[k - 1], elems[k]),) +
                   elems[k + 1:])
        except (ValueError, ZeroDivisionError, OverflowError):
            pass


@memoize
def check_list(nums, operators):
    """Checks if a list of numbers can be made into a true statement"""
    if len(nums) <= 1:
        return False  # there's no place to insert an operator
    if len(nums) >= 2 and all_equal(nums):
        return True
    return any(
        check_list(new_nums, operators)
        for operator in operators
        for new_nums in insert_binary(nums, operator))


def check_number(num, operators):
    """Checks if a single number can be made into a true statement"""
    for partit in chunks(str(num)):
        partit = tuple(Fraction(num) for num in partit)
        if check_list(partit, operators):
            return True
    return False


# pylint: disable = C0103
# (invalid argument name)
def base_add(x, y):
    """add"""
    return x + y


def base_subtr(x, y):
    """add"""
    return x - y


def base_mult(x, y):
    """add"""
    return x * y


def base_div(x, y):
    """add"""
    return x / y


def base_pow(x, y):
    """pow"""
    if x not in {-1, 0, 1} and abs(y) > 100:
        raise ValueError
    return x ** y

# pylint: enable = C0103


def check_all_basic(nums):
    """Check all nums in a list"""
    operators = (base_add, base_subtr, base_mult, base_div, base_pow)
    no_good = []
    for num in nums:
        print(num, end='\r', flush=True)
        if not check_number(num, operators):
            no_good.append(num)
    print(1 - len(no_good)/len(nums))
    return no_good
