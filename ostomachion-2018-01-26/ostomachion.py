"""
Divide a list into equal sums.
Run with 'python3 ostomachion.py' to see a summary of the results.
"""


class LabeledInt(int):
    """
    An integer that keeps a label with it.
    """
    def __new__(cls, _label, *args, **kwargs):
        return super().__new__(cls, *args, **kwargs)

    def __init__(self, label, *_args, **_kwargs):
        super().__init__()
        self.label = label

    def __repr__(self):
        return self.label + str(int(self))

    def __str__(self):
        return self.label + str(int(self))

    def __eq__(self, other):
        try:
            return super().__eq__(other) and self.label == other.label
        except AttributeError:
            return super().__eq__(other)

    def __hash__(self):
        return hash((int(self), self.label))


def find_sub_with_sum(numbers, num_sum):
    """
    Iterate through the parts of a set of numbers with the given sum.
    """
    new_numbers = numbers.copy()
    popped = set()
    while new_numbers:
        number = new_numbers.pop()
        if number == num_sum:
            yield (frozenset({number}), new_numbers.union(popped))
        elif number < num_sum:
            for combo, remainder in find_sub_with_sum(
                    new_numbers, num_sum - number):
                yield (combo.union({number}), remainder.union(popped))
        popped.add(number)


def div_equal(numbers, num_of_parts):
    """
    Iterate through the ways of dividing the list of numbers
    into some number of parts where the sums of each part are equal.
    Note that this only works if all the numbers are non-negative.
    """

    if num_of_parts == 0 and not numbers:
        yield frozenset({frozenset()})
    if num_of_parts == 1:
        yield frozenset({frozenset(numbers)})
    elif num_of_parts > 0 and numbers:
        part_sum = sum(numbers)//num_of_parts

        for part, remainder in find_sub_with_sum(numbers, part_sum):
            for combo in div_equal(remainder, num_of_parts - 1):
                yield combo.union({part})


def filter_pairs(combos, pairs):
    """
    Filter a list of combos so that no part contains both of each pair
    """
    return {
        combo for combo in combos if all(
            not any(
                pair[0] in part and pair[1] in part
                for part in combo)
            for pair in pairs)
    }


def setup():
    """Set up our variables"""
    return ({LabeledInt(label, k)
             for k, label in [(3, 'a'), (3, 'b'), (6, 'a'), (6, 'b'),
                              (6, 'c'), (6, 'd'), (9, 'a'), (12, 'a'),
                              (12, 'b'), (12, 'c'), (12, 'd'), (12, 'e'),
                              (21, 'a'), (24, 'a')]},
            {
                (LabeledInt('a', 12), LabeledInt('b', 12)),
                (LabeledInt('a', 12), LabeledInt('c', 12)),
                (LabeledInt('a', 12), LabeledInt('a', 6)),
                (LabeledInt('b', 12), LabeledInt('c', 12)),
                (LabeledInt('c', 12), LabeledInt('b', 6)),
                (LabeledInt('c', 12), LabeledInt('a', 3)),
                (LabeledInt('c', 12), LabeledInt('a', 21)),
                (LabeledInt('b', 6), LabeledInt('a', 3)),
                (LabeledInt('a', 3), LabeledInt('a', 21)),
                (LabeledInt('a', 21), LabeledInt('c', 6)),
                (LabeledInt('a', 21), LabeledInt('a', 6)),
                (LabeledInt('c', 6), LabeledInt('e', 12)),
                (LabeledInt('c', 6), LabeledInt('d', 12)),
                (LabeledInt('e', 12), LabeledInt('d', 6)),
                (LabeledInt('d', 6), LabeledInt('a', 9)),
                (LabeledInt('d', 6), LabeledInt('d', 12)),
                (LabeledInt('a', 9), LabeledInt('b', 3)),
                (LabeledInt('b', 3), LabeledInt('a', 24)),
                (LabeledInt('a', 24), LabeledInt('d', 12)),
                (LabeledInt('d', 12), LabeledInt('a', 6)),
            })


def print_combos(combos):
    """Nicely format a set of combos"""
    return '\n'.join(
        ', '.join(
            str(set(part)) for part in combo)
        for combo in combos)


def main():
    """Do the thing"""
    nums, pairs = setup()
    combos = set(div_equal(nums, 4))
    print("Number of colorings into equal area: {}".format(len(combos)))
    four_color_combos = filter_pairs(combos, pairs)
    print("Number of coloring with equal area and no adjacent regions with the"
          " same color: {}".format(len(four_color_combos)))
    print(print_combos(four_color_combos))

if __name__ == '__main__':
    main()
