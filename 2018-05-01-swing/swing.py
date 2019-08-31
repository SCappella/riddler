"""
A path finding algorithm for swings:
see https://www.janestreet.com/puzzles/swing-time-2/
"""


from math import gcd


class Post(tuple):
    """A convenience class for manipulating posts"""

    def __add__(self, other):
        return Post(x + y for x, y in zip(self, other))

    def __sub__(self, other):
        return Post(x - y for x, y in zip(self, other))

    def __eq__(self, other):
        return all(x == y for x, y in zip(self, other))

    def __hash__(self):
        return hash(tuple(self))

    def __str__(self):
        return "Post{}".format(tuple(self))

    def __repr__(self):
        return "Post{}".format(tuple(self))

    def reduced(self):
        """Return a gcd reduced version of self"""
        common_factor = 0
        for val in self:
            common_factor = gcd(common_factor, val)

        return Post(tuple(x//common_factor for x in self))

    def between(self, other):
        """
        An iterator over the integer lattice points between self and other.
        Includes self, but not other.
        """
        current = self
        reduced = (other - self).reduced()
        while current != other:
            yield current
            current = current + reduced


class SwingGrid:
    """Keeps track of a grid of posts"""

    def __init__(self, dimensions, posts):
        self.grid = [[0 for _ in range(dimensions[0])]
                     for _ in range(dimensions[1])]
        for post in posts:
            self.grid[post[0]][post[1]] = 1

        self.posts = set(Post(post) for post in posts)

        self.position = Post((0, 0))
        self.dimensions = dimensions

    def copy(self):
        """Returns a copy of this grid"""
        return SwingGrid(self.dimensions.copy(), self.posts.copy())

    def remove_post(self, post):
        """Remove a single post by its position"""
        self.grid[post[0]][post[1]] = 0
        self.posts.remove(post)
        return self

    def get_swingable(self):
        """
        Find all the posts that are in line of sight
        of the current position
        """
        for post in self.posts:
            direction_x = post[0] - self.position[0]
            direction_y = post[1] - self.position[1]
            number_of_steps = gcd(direction_x, direction_y)
            basic_x = direction_x // number_of_steps
            basic_y = direction_y // number_of_steps

            if all(
                    self.grid[self.position[0] + mult *
                              basic_x][self.position[1] + mult * basic_y] == 0
                    for mult in range(1, number_of_steps)
            ):
                yield post


def setup():
    """Set up the puzzle"""
    posts = [
        (0, 10), (0, 16), (0, 19),
        (2, 19),
        (3, 10), (3, 15),
        (4, 7), (4, 15), (4, 19),
        (5, 0),
        (7, 1), (7, 7), (7, 12), (7, 17),
        (8, 16),
        (9, 2),
        (10, 19),
        (11, 3), (11, 6), (11, 15),
        (12, 13),
        (13, 4), (13, 17),
        (14, 1), (14, 16), (14, 19),
        (15, 5), (15, 8), (15, 12), (15, 14), (15, 17),
        (16, 3), (16, 19),
        (17, 6), (17, 15), (17, 18),
        (18, 12), (18, 17), (18, 18),
        (19, 0), (19, 4), (19, 11), (19, 14), (19, 16),
    ]

    return SwingGrid((20, 20), posts)
