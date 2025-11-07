import pytest
import numpy as np 
import py_impl

@pytest.fixture(scope="function")
def make_iterable(request):
    length = request.param
    iterable = [np.random.rand() for _ in range(length)]
    return iterable

@pytest.mark.parametrize("make_iterable", [10000, 100000, 1000000, 10000000], indirect=True)
def test_mprdc(benchmark, make_iterable):
    iterable = make_iterable
    benchmark(py_impl.normal_sum_squares, iterable)


