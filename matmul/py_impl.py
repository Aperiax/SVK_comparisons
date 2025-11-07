import numpy
import timeit

class matrix:
    def __init__(self, rows, cols, list = []) -> None:

        self.rows = rows
        self.cols = cols
        self.data = list

    def matmul_naive(self, other):
        
        assert self.cols == other.rows, "wrong dimensions of matrices"
        tmp = list()

        for r in range(self.rows):
            for c in range(other.cols):
                acc = 0
                for i in range(self.cols):
                    a = self.data[r * self.cols + i]
                    b = other.data[i * other.cols + c]

                    acc += a * b

                tmp.append(acc);


        return matrix(self.rows, other.cols, tmp)

def read_matrices(size):
    temp = []
    for ident in [1, 2]:

        with open(f"/home/aperiax/School/SVK/matrix_{ident}_{size}", "r") as f:
            m_temp = matrix(rows=size, cols=size, list=[])
            for line in f:
                m_temp.data.append(float(line))
            temp.append(m_temp)

    return temp[0], temp[1]


if __name__ == "__main__":
    num_runs = 10
    sizes = [10, 100, 1000]  # pick sizes you want
    res_naive = []

    for size in sizes:
        temp_naive = []
        M1, M2 = read_matrices(size)

        # print(f"starting size {size} run")
        for _ in range(num_runs):
            # timeit takes a callable; repeat=1 gives one-shot timing per run
            t = timeit.timeit(lambda: M1.matmul_naive(M2), number=1)
            temp_naive.append(t)
        avg = sum(temp_naive) / len(temp_naive)
        res_naive.append(avg)

    print("MATMUL NAIVE")
    print("Tested sizes: 10, 100, 1000, 3000")
    print(f"Averages sequential (timeit), {num_runs} runs: {res_naive}")
