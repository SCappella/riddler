"""Find nice paths in a grid"""


from enum import Enum, auto


class Grid:
    """Describes a grid"""
    def __init__(self, x_len, y_len):
        self.x_len = int(x_len)
        self.y_len = int(y_len)

    def check_nodes(self, nodes):
        """Checks if all nodes are in this grid and that they are adjecent"""
        if not nodes:
            return True

        if len(nodes) == 1:
            return nodes[0].is_in_grid(self)

        prev_node = nodes[0]

        for node in nodes[1:]:
            if not node.is_in_grid(self):
                return False

            if not prev_node.is_adjacent(node):
                return False

            prev_node = node

        return True

    def touchless_paths(self, x_start, y_start, length):
        """
        Iterate through all non-self-touching paths
        with given start point and length
        """
        def paths_rec(path, remaining_length):
            """An auxillary recursive function"""
            if remaining_length == 0:
                yield path
                return
            end_node = path.nodes[-1]
            for new_node in end_node.adjacent_nodes():
                new_path = path.add_node(new_node)
                if not new_path.is_touching_self():
                    for rec_path in paths_rec(new_path, remaining_length - 1):
                        yield rec_path

        initial_node = Node(self, x_start, y_start)
        initial_path = Path(self, [initial_node])
        for path in paths_rec(initial_path, length - 1):
            yield path

    def touchless_filtered(self, x_start, y_start, length, evens, odds):
        """Filter based on paths that avoid off parity squares"""
        if length % 2 == 0:
            avoid = evens
        else:
            avoid = odds

        for path in self.touchless_paths(x_start, y_start, length):
            if all((node.node_x, node.node_y) not in avoid
                   for node in path.nodes):
                yield path

    def touchless_symmetric(self, x_start, y_start, length, evens, odds):
        """Filter to only symmetric paths"""
        for path in self.touchless_filtered(x_start, y_start,
                                            length, evens, odds):
            if path.is_symmetric():
                yield path


class Node:
    """Descibes a point in a grid"""
    def __init__(self, grid, node_x, node_y):
        self.grid = grid
        self.node_x = node_x
        self.node_y = node_y
        if not self.is_in_grid(self.grid):
            raise ValueError("Invalid node coordinates")

    def __str__(self):
        return '({}, {})'.format(self.node_x, self.node_y)

    def is_adjacent(self, node):
        """Checks if another node is (orthoganally) adjacent in the grid"""
        return (abs(self.node_x - node.node_x) +
                abs(self.node_y - node.node_y) <= 1)

    def adjacent_nodes(self):
        """Iterate through adjacent nodes"""
        for x_diff in [-1, 1]:
            try:
                yield Node(self.grid, self.node_x + x_diff, self.node_y)
            except ValueError:
                pass
        for y_diff in [-1, 1]:
            try:
                yield Node(self.grid, self.node_x, self.node_y + y_diff)
            except ValueError:
                pass

    def is_in_grid(self, grid):
        """Checks if this node is in another grid"""
        return (0 <= self.node_x < grid.x_len and
                0 <= self.node_y < grid.y_len)

    @staticmethod
    def to_direction_trio(node0, node1, node2):
        """
        Detects the change in direction from node0 to node 2.
        The nodes must be adjacent.
        """
        if not node0.is_adjacent(node1) or not node1.is_adjacent(node2):
            raise ValueError("Nodes not adjacent")

        diff1_x = node0.node_x - node1.node_x
        diff1_y = node0.node_y - node1.node_y

        diff2_x = node2.node_x - node1.node_x
        diff2_y = node2.node_y - node1.node_y

        if diff1_x * diff2_x > 0:
            return Direction.backwards
        if diff1_x * diff2_x < 0:
            return Direction.forwards
        if diff1_y * diff2_y > 0:
            return Direction.backwards
        if diff1_y * diff2_y < 0:
            return Direction.forwards

        if diff1_x * diff2_y > 0:
            return Direction.right
        if diff1_x * diff2_y < 0:
            return Direction.left

        if diff1_y * diff2_x > 0:
            return Direction.left
        if diff1_y * diff2_x < 0:
            return Direction.right

        raise ValueError


class Path:
    """Descibes a path in a grid"""
    def __init__(self, grid, nodes):
        self.grid = grid
        self.nodes = nodes
        if not self.grid.check_nodes(nodes):
            raise ValueError("Invalid nodes argument")

    def __str__(self):
        grid = list(list(' *'
                         for _ in range(self.grid.x_len))
                    for _ in range(self.grid.y_len))
        for index, node in enumerate(self.nodes):
            grid[node.node_y][node.node_x] = str(index).rjust(2)

        return '\n'.join(' '.join(line) for line in grid)

    def add_node(self, node):
        """Returns a new path with a new node added"""
        return Path(self.grid, self.nodes + [node])

    def is_touching_self(self):
        """Checks if a path is touching itself"""
        return any(any(prev_node.is_adjacent(head_node)
                       for prev_node in self.nodes[:index + 1])
                   for (index, head_node) in enumerate(self.nodes[2:]))

    def to_direction(self):
        """Returns a list of direction changes"""
        return [
            Node.to_direction_trio(
                self.nodes[index - 2],
                self.nodes[index - 1],
                self.nodes[index])
            for index in range(2, len(self.nodes))
        ]

    def reverse(self):
        """Returns a reversed version of self"""
        return Path(self.grid, self.nodes[::-1])

    def is_symmetric(self):
        """Tests if the path is symmetric by mirroring or rotation"""
        reversed_self = self.reverse()
        return (self.to_direction() == reversed_self.to_direction() or
                self.to_direction() == [
                    direct.reverse()
                    for direct in reversed_self.to_direction()])


class Direction(Enum):
    """left, right, back, forward"""
    left = auto()
    right = auto()
    backwards = auto()
    forwards = auto()

    def reverse(self):
        """Reverse right and left"""
        if self is Direction.left:
            return Direction.right
        if self is Direction.right:
            return Direction.left
        return self


def setup():
    """Set up grid, evens and odds"""
    grid = Grid(11, 11)

    evens = [
        (0, 8),
        (3, 0),
        (3, 2),
        (4, 0),
        (4, 1),
        (4, 9),
        (5, 0),
        (5, 2),
        (5, 6),
        (6, 0),
        (6, 1),
        (6, 2),
        (6, 6),
        (7, 0),
        (7, 2),
        (7, 6),
        (8, 0),
        (8, 1),
        (8, 2),
        (8, 3),
        (8, 5),
        (8, 6),
        (8, 9),
        (9, 0),
        (9, 2),
        (9, 3),
        (9, 4),
        (9, 5),
        (10, 1),
        (10, 2),
        (10, 3),
        (10, 4),
        (10, 5),
        (10, 6),
        (10, 7),
        (10, 8),
        (10, 9),
    ]

    odds = [
        (0, 1),
        (0, 2),
        (0, 4),
        (0, 7),
        (1, 4),
        (1, 5),
        (1, 7),
        (1, 8),
        (2, 1),
        (2, 2),
        (2, 3),
        (2, 4),
        (2, 7),
        (2, 9),
        (3, 3),
        (3, 4),
        (3, 6),
        (3, 7),
        (3, 8),
        (3, 9),
        (3, 10),
        (4, 2),
        (4, 3),
        (4, 5),
        (4, 6),
        (4, 7),
        (5, 3),
        (5, 5),
        (5, 7),
        (5, 8),
        (6, 3),
        (6, 5),
        (6, 7),
        (6, 9),
        (7, 3),
        (7, 4),
        (7, 5),
        (7, 7),
        (7, 8),
        (7, 10),
        (8, 4),
        (8, 7),
        (9, 6),
        (9, 7),
        (9, 8),
        (9, 9),
    ]

    return (grid, evens, odds)
