from itertools import chain, product, combinations

class Position:
    trans_left = str.maketrans('123456789abcdef', 'bc7d84e952fa631')
    trans_right = str.maketrans('bc7d84e952fa631', '123456789abcdef')

    def __hash__(self):
        return hash((frozenset(self.stripe), frozenset(self.solid), frozenset(self.eight)))

    def __eq__(self, other):
        return set(self.stripe) == set(other.stripe) and \
               set(self.solid) == set(other.solid) and \
               set(self.eight) == set(other.eight)

    def __init__(self, stripe='1234567', solid='89abcde', eight='f'):
        self.stripe = stripe
        self.solid = solid
        self.eight = eight

    def __to_flat_string(self):
        string = ''
        for char in '123456789abcdef':
            if char in self.stripe:
                string += '1'
            elif char in self.solid:
                string += '0'
            elif char in self.eight:
                string += '8'
        return string

    def __str__(self):
        flstr = self.__to_flat_string()
        return f'    {" ".join(flstr[:1])}\n' + \
               f'   {" ".join(flstr[1:3])}\n' + \
               f'  {" ".join(flstr[3:6])}\n' + \
               f' {" ".join(flstr[6:10])}\n' + \
               f'{" ".join(flstr[10:15])}'


    @classmethod
    def puzzle_init(cls):
        return cls('2679cdf', '1348abe', '5')

    @classmethod
    def iter_all(cls):
        for stripes in combinations('123456789abcdef', 7):
            stripes = ''.join(stripes)
            remaining_stripes = set('123456789abcdef').difference(stripes)
            for solids in combinations(remaining_stripes, 7):
                solids = ''.join(solids)
                remaining_solids = remaining_stripes.difference(solids)
                for eights in combinations(remaining_solids, 1):
                    eights = ''.join(eights)
                    yield Position(stripes, solids, eights)

    def rotate_left(self):
        return Position(
            self.stripe.translate(self.trans_left),
            self.solid.translate(self.trans_left),
            self.eight.translate(self.trans_left))

    def rotate_right(self):
        Position(
            self.stripe.translate(self.trans_right),
            self.solid.translate(self.trans_right),
            self.eight.translate(self.trans_right))

    def swaps(self):
        for pos1, pos2 in chain(product(self.stripe, self.solid), product(self.stripe, self.eight), product(self.solid, self.eight)):
            trans = str.maketrans(pos1 + pos2, pos2 + pos1)
            yield Position(
                self.stripe.translate(trans),
                self.solid.translate(trans),
                self.eight.translate(trans))


class Positions:
    def __init__(self, init):
        self.positions = {init}
        self.new = {init}

    def iterate_once(self):
        new = set()
        new.update(p.rotate_left() for p in self.new)
        new.update(p.rotate_left() for p in self.new)
        new.update(chain(*(p.swaps() for p in self.new)))
        self.new = new.difference(self.positions)
        self.positions.update(new)


def height(position):
    positions = Positions(position)
    k = 0
    while len(positions.positions) < 51480:
        positions.iterate_once()
        k += 1
    return k
