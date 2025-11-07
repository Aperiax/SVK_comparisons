from random import Random
from math import inf, pi
import timeit


def simulation(max_iter: int):
    in_: int = 0
    out_:int = 0

    iter: int = 0
    res: float = 0

    rand = Random()

    for _ in range(max_iter):

        x=  rand.random()
        y=  rand.random()

        inside = x*x + y*y < 1

        in_ += inside
        out_ += 1 - inside
        res = 4*(in_ / (in_+out_))

        iter += 1

    return (res, iter)



if __name__ == "__main__":
    num_runs = 10
    sizes = [1000000, 2000000, 3000000, 10000000]  # pick sizes you want
    res = []

    for size in sizes:
        temp_ = []
        for _ in range(num_runs):
            t = timeit.timeit(lambda: simulation(size), number=1)
            temp_.append(t)
        avg = sum(temp_) / len(temp_)
        res.append(avg)

    print("Tested sizes: 1000000, 2000000, 3000000, 10000000")
    print(f"Averages: {res}")
