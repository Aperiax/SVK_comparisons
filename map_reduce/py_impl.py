import numpy as np
import timeit

def normal_sum_squares(iterable: list[float]) -> float:
    return sum(x * x for x in iterable)


if __name__ == "__main__":
    num_runs = 10
    sizes = [10000, 100000, 1000000, 10000000]  # pick sizes you want
    res_naive = []

    for size in sizes:
        temp_naive = []
        iterable = [np.random.random() for _ in range(size)]
        for _ in range(num_runs):
            t = timeit.timeit(lambda: normal_sum_squares(iterable), number=1)
            temp_naive.append(t)
        avg = sum(temp_naive) / len(temp_naive)
        res_naive.append(avg)

    print("SUMSQRSMPRD")
    print("Tested sizes: 1e4, 1e5, 1e6, 1e7")
    print(f"Averages (timeit), {num_runs} runs: {res_naive}")
