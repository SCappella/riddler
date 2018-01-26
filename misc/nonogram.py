"""
Tools for solving nonograms
"""


def possibility_iterate(clues, row_or_column_count, offset=0):
    """
    clues is the list of clues for the given column/row,
    row_count is the number of rows (resp. columns)

    This function gives a generator for all the possible ways
    to fill the column/row with the given clue

    Since this function is recursive, we include an initial offset,
    so everthing gets lined up right.
    Ignore that argument when manually calling this function.
    """
    if clues:
        for k in range(row_or_column_count - clues[0] + 1):
            for tail in possibility_iterate(
                    clues[1:],
                    (row_or_column_count - (clues[0] + k + 1)),
                    offset=offset + (clues[0] + k + 1)):
                yield (offset + k,) + tail
    else:
        yield ()


def poss_convert(possibility, clues, row_or_column_count):
    """
    Converts a tuple of indices (representing locations of blocks)
    to a tuple of bools (representing whether a square is shaded).
    """
    # I think this works since False is immutable
    output = [False] * row_or_column_count
    for location, length in zip(possibility, clues):
        for k in range(location, location + length):
            output[k] = True
    return tuple(output)


class RowOrColumn:

    """Data for a single row or column"""
    # too few public methods
    # pylint: disable = R0903

    def __init__(self, clues, row_or_column_count, index):
        self.clues = clues
        self.possibilities = set(poss_convert(poss, clues, row_or_column_count)
                                 for poss in possibility_iterate(
                                     clues,
                                     row_or_column_count))
        self.index = index

    def refine(self, others):
        """Refines based on the contrary direction possibilities"""
        for k, other in enumerate(others):
            self.possibilities = {
                poss for poss in self.possibilities
                if any(poss[k] == other_poss[self.index]
                       for other_poss in other.possibilities)}


class Puzzle:

    """Data for the full puzzle"""

    def __init__(self, row_clues, column_clues):
        self.rows = [
            RowOrColumn(clues, len(column_clues), k)
            for k, clues in enumerate(row_clues)
        ]
        self.columns = [
            RowOrColumn(clues, len(row_clues), k)
            for k, clues in enumerate(column_clues)
        ]

        self.total = (
            sum(len(row.possibilities) for row in self.rows) +
            sum(len(column.possibilities) for column in self.columns))

    def __str__(self):
        """
        Display the current puzzle as a grid.
        Known squares are denoted with █ (U+2588) and ░ (U+2591).
        Unknown squares are denoted with �
        (U+FFFD - it's also the replacement character)
        """
        out_string = ''
        for i, row in enumerate(self.rows):
            for j, column in enumerate(self.columns):
                if (all(x[j] for x in row.possibilities) and
                        all(x[i] for x in column.possibilities)):
                    out_string += '█'
                elif (all(not x[j] for x in row.possibilities) and
                      all(not x[i] for x in column.possibilities)):
                    out_string += '░'
                else:
                    out_string += '�'
            out_string += '\n'
        out_string = out_string[:-1]  # strip last \n
        return out_string

    def refine(self):
        """Refines all rows and columns"""
        for row in self.rows:
            row.refine(self.columns)

        for column in self.columns:
            column.refine(self.rows)

        self.total = (
            sum(len(row.possibilities) for row in self.rows) +
            sum(len(column.possibilities) for column in self.columns))

        print('-' * 40)
        print(self)

    def repeat_refine(self):
        """Refine until nothing is learned"""
        total = self.total
        while True:
            self.refine()
            if self.total == total:
                break
            total = self.total


def str_to_clues(string):
    """Convert a string to a list of clues: newlines are a new clue"""
    output = []
    for line in string.split('\n'):
        clue_line = []
        for clue in line:
            clue_line.append(int(clue))
        output.append(clue_line)
    return output


def str_to_clues_space(string):
    """Convert a string to a list of clues: newlines are a new clue"""
    output = []
    for line in string.split('\n'):
        clue_line = []
        for clue in line.split(' '):
            clue_line.append(int(clue))
        output.append(clue_line)
    return output


def setup_1():
    """Set up the first puzzle"""
    return Puzzle(
        [[1], [1], [3], [9], [5], [3], [2, 2], [2, 2], [1, 1, 1]],
        [[1], [1, 2], [2, 2], [5], [6, 1], [5], [2, 2], [1, 2], [1]]
    )


def setup_2():
    """Set up the second puzzle"""
    return Puzzle(
        [[2], [1, 1], [1, 1], [2], [4], [6], [6], [6], [6], [1, 6, 1], [8],
         [6], [2]],
        [[1], [1], [7], [2, 8], [1, 10], [1, 10], [2, 8], [7], [1], [1]]
    )


def setup_3():
    """Set up the second puzzle"""
    return Puzzle(
        [[3, 2, 2, 6], [1, 4, 4, 1, 1, 1], [4, 2, 1, 4],
         [1, 2, 1, 2, 1, 1, 1, 1], [1, 1, 2, 1, 2, 6, 1],
         [1, 1, 2, 3, 1, 1, 1], [3, 1, 3, 4], [1, 3, 1, 1, 1, 1],
         [1, 3, 3, 1, 2], [7, 1, 3, 2], [1, 1, 7, 1, 3], [1, 3, 3, 1],
         [4, 2, 2, 2, 1, 2], [2, 6, 3, 1, 2, 1], [3, 9, 2, 1], [3, 3, 2, 1],
         [3, 3, 4]],
        [[3, 1, 1, 1, 1], [1, 2, 3, 1, 2, 1], [1, 1, 1, 1, 2, 2, 1],
         [3, 1, 1, 2, 4], [1, 2, 4, 5], [2, 2, 5, 3], [2, 1, 2, 3], [1, 1, 3],
         [1, 2], [1, 1, 2, 1, 1], [2, 1, 1, 7], [2, 2, 5, 4], [1, 4, 9],
         [3, 1, 2, 1, 1], [1, 3, 1, 2], [1, 2, 1, 1], [1, 1, 1, 1, 1],
         [2, 2, 1, 2], [1, 1, 4, 1, 2, 1], [5, 1, 2, 2, 1], [1, 1, 1, 5, 1],
         [1, 7, 4]]
    )
