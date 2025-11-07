import pytest
import numpy as np
from py_impl import Graph

@pytest.fixture(scope="function")
def make_graph(request):
    size = request.param
    g = Graph(size)
    return g

@pytest.fixture(scope="function")
def read_graph(request):
    size = request.param
    g = Graph.read_test_graph(size)
    return g

@pytest.mark.parametrize("make_graph", [100, 1000,10000], indirect=True)
def test_graph_generation_benchmark(benchmark, make_graph):
    g = make_graph
    benchmark(g.complete_graph, 0.02)

@pytest.mark.parametrize("read_graph", [100, 1000, 10000], indirect=True)
def test_bfs_benchmark(benchmark, read_graph):
    g = read_graph 
    dest = np.random.randint(0, g.size)
    benchmark(g.bfs, 0, dest)
