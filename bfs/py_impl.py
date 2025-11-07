import timeit
import numpy as np
from collections import deque
from bitarray import bitarray  # closest to bitvec in Python

class Graph:
    def __init__(self, size):
        self.size = size
        self.adj = [[] for _ in range(size)]

    def add_edge(self, to, from_):
        self.adj[from_].append(to)
        self.adj[to].append(from_)

    def min_tree(self):
        prufer = [np.random.randint(0, self.size) for _ in range(self.size - 2)]
        np.random.shuffle(prufer)

        degree = [1] * self.size
        for node in prufer:
            degree[node] += 1

        edges = []
        leaf = 0
        for _ in range(self.size - 2):
            while degree[leaf] != 1:
                leaf += 1
            node = prufer.pop(0)
            edges.append((leaf, node))
            degree[leaf] -= 1
            degree[node] -= 1
            leaf = 0

        remaining = [i for i, d in enumerate(degree) if d == 1]
        edges.append((remaining[0], remaining[1]))
        return edges

    def node_id_to_indx(self, edge):
        a, b = sorted(edge)
        assert a != b, "SELF LOOP!"
        return ((a * (2 * self.size - a - 1)) // 2) + (b - a - 1)

    def connect_to_density_(self, density):
        edges_ = self.min_tree()
        edges_.sort()
        total_possible = (self.size * (self.size - 1)) // 2
        edges_to_make = int(density * total_possible) - len(edges_)

        adjacency = bitarray(total_possible)
        adjacency.setall(0)
        for edge in edges_:
            adjacency[self.node_id_to_indx(edge)] = 1

        extra = 0
        while extra < edges_to_make:
            test_edge = (np.random.randint(0, self.size), np.random.randint(0, self.size))
            if test_edge[0] == test_edge[1]:
                continue
            idx = self.node_id_to_indx(test_edge)
            if adjacency[idx]:
                continue
            edges_.append(test_edge)
            adjacency[idx] = 1
            extra += 1
        return edges_

    def complete_graph(self, density):
        edges = self.connect_to_density_(density)
        for a, b in edges:
            self.add_edge(a, b)

    def bfs(self, start, dest):
        explored = bitarray(self.size)
        explored.setall(0)
        queue = deque()
        parent = [None] * self.size

        queue.append(start)
        explored[start] = 1

        while queue:
            current_ = queue.popleft()

            if current_ == dest:
                path = []
                curr = current_
                while curr is not None:
                    path.append(curr)
                    curr = parent[curr]
                path.reverse()
                return path

            for neighbor in self.adj[current_]:
                if not explored[neighbor]:
                    explored[neighbor] = 1
                    parent[neighbor] = current_
                    queue.append(neighbor)
        return None

    @staticmethod
    def read_test_graph(size: int):

        path = f"/home/aperiax/School/SVK/graph_{size}"

        g = Graph(size)

        with open(path, "r") as f: 
            for line in f:
                nums = line.split(" ")
                assert len(nums) == 2, "wrong length after split"

                g.add_edge(int(nums[0]), int(nums[1]))
                
        return g

if __name__ == "__main__":
    num_runs = 10
    sizes = [100, 1000, 10000]
    res_graphgen = []
    res_bfs = []

    for size in sizes:
        temp_graphgen = []
        temp_bfs = []

        for _ in range(num_runs):
            g = Graph(size)
            t = timeit.timeit(lambda: g.complete_graph(0.02), number=1)
            temp_graphgen.append(t)

            g = Graph.read_test_graph(size)
            t_bfs = timeit.timeit(lambda: g.bfs(0, np.random.randint(0, size)), number=1)
            temp_bfs.append(t_bfs)

        avg_graphgen = sum(temp_graphgen) / len(temp_graphgen)
        avg_bfs = sum(temp_bfs) / len(temp_bfs)

        res_graphgen.append(avg_graphgen)
        res_bfs.append(avg_bfs)

    print("GRAPH GENERATION")
    print(f"Sizes tested: {sizes}")
    print(f"Averages (timeit, {num_runs} runs): {res_graphgen}")

    print("BFS")
    print(f"Sizes tested: {sizes}")
    print(f"Averages (timeit, {num_runs} runs): {res_bfs}")
