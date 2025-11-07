import pytest
from py_impl import simulation 

@pytest.mark.parametrize("maxiter", [1000000, 2000000, 3000000, 10000000])
def test_mc(benchmark, maxiter):
    benchmark(simulation, maxiter)
