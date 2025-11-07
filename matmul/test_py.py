import pytest
from py_impl import read_matrices


@pytest.fixture(scope="function")
def make_matrices(request):
    """Fixture to generate matrices for a given size."""
    size = request.param
    m1, m2 = read_matrices(size)
    return m1, m2


@pytest.mark.parametrize("make_matrices", [10, 100, 1000], indirect=True)
def test_matmul_benchmark(benchmark, make_matrices):
    M1, M2 = make_matrices
    # benchmark the naive multiplication
    benchmark(M1.matmul_naive, M2)
