This problem can be solved with [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm). The grid can be turned into a set of nodes (each square) edges (valid moves) and costs for each edge (moving into a numbered square).

Dijkstra's algorithm effectively keeps track of a set of feasible paths through the graph. At each step, the shortest path is extended with all possible steps, and these new paths are added to the set. Here, "shortest path" can refer to a variety of metrics, and here I used the sum of the costs for the path as the primary metric, and the length of the path as the secondary metric.

At the end, the shortest path according to the metrics was found. This means that the path has the lowest possible cost, and among paths that reach that lowest cost, this is (one of) the shortest. This path starts at the 18th square from the left on the top edge. A picture of the path can be found [here](https://github.com/SCappella/riddler/blob/master/2019-02-15-maze/solution.png).

My code can be found on [Github](https://github.com/SCappella/riddler/tree/master/2019-02-15-maze). This code allows some variation in the input grid, using 'u', 'd', 'l', 'r' for single directions, 'v' and 'h' for double directions, a digit for a square that has a cost, 'x' for dead squares, and 'g' for the end goal.
