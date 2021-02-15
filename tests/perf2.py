from IPython import get_ipython

import cachetools as cachers

ipython = get_ipython()
assert ipython is not None

value = '*'

for _ in range(0, 10):

    print('FIFO set')
    dc = cachers.FIFOCache(maxsize=1000)
    ipython.magic("timeit -n 10 -r 7 for i in range(0, 10000): dc[i] = value * i")

    print('FIFO get')
    dc = cachers.FIFOCache(maxsize=10000)
    for i in range(0, 10000):
        dc[i] = value * i
    ipython.magic("timeit -n 10 -r 7 for i in range(0, 10000): dc[i]")

    print('FIFO set/delete')
    dc = cachers.FIFOCache(maxsize=10000)
    ipython.magic("timeit -n 10 -r 7 for i in range(0, 10000): dc[i] = value * i; del dc[i]")

    print("\n", "-" * 10, "\n")

    print('LRU set')
    dc = cachers.LRUCache(maxsize=1000)
    ipython.magic("timeit -n 10 -r 7 for i in range(0, 10000): dc[i] = value * i")

    print('LRU get')
    dc = cachers.LRUCache(maxsize=10000)
    for i in range(0, 10000):
        dc[i] = value * i
    ipython.magic("timeit -n 10 -r 7 for i in range(0, 10000): dc[i]")

    print('LRU set/delete')
    dc = cachers.LRUCache(maxsize=10000)
    ipython.magic("timeit -n 10 -r 7 for i in range(0, 10000): dc[i] = value * i; del dc[i]")

    print("\n", "-" * 10, "\n")

