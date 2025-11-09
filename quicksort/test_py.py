import pytest
import numpy as np
from py_impl import *

@pytest.mark.parametrize("size", [100000, 1000000, 10000000])
def test_quicksort_bench(benchmark, size):
    base_arr = read_array(size)
    def run():
        arr = base_arr.copy()
        quicksort(arr, 0, size - 1)

    benchmark(run)
