import timeit
import numpy as np

def read_array(size): 
    return np.fromfile(f"/home/aperiax/School/SVK/arr_{size}", dtype=np.uint64).tolist()

def partition(arr, low, high):
    pivot = arr[high]
    i = low

    for j in range(low, high):
        mask = (arr[j] <= pivot)
        arr[i * mask + j * (1 - mask)], arr[j] = arr[j], arr[i * mask + j * (1 - mask)]
        i += mask

    arr[i], arr[high] = arr[high], arr[i]
    return i

def quicksort(arr, low, high):
    while low < high:
        p = partition(arr, low, high)
        if p - low < high - p:
            if p > 0 :
                quicksort(arr, low, p - 1)
            low = p + 1
        else:
            quicksort(arr, p + 1, high);
            if p == 0:
                break
            high = p - 1;

if __name__ == "__main__":

    sizes = [100000, 1000000, 10000000]
    res = []
    
    for size in sizes:
        print(size)
        temp= []
        arr= read_array(size)
        for run in range(10):
            print(run)
            t = timeit.timeit(lambda: quicksort(arr.copy(), 0, size-1), number=1)
            temp.append(t)

        avg = sum(temp) / len(temp)
        res.append(avg)


    print("QUICKSORT")
    print(f"Sizes tested: {sizes}")
    print(f"Averages (timeit), {10} runs: {res}")
