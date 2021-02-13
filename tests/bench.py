from IPython import get_ipython

import cachers

ipython = get_ipython()
assert ipython is not None

value = '*'

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

try:
    import cachetools
except ImportError:
    print('Error: Cannot import cachetools')
else:
    print('cachetools.FIFO set')
    dc = cachetools.FIFOCache(maxsize=1000)
    ipython.magic("timeit -n 10 -r 7 for i in range(0, 10000): dc[i] = value * i")

    print('cachetools.FIFO get')
    dc = cachetools.FIFOCache(maxsize=10000)
    for i in range(0, 10000):
        dc[i] = value * i
    ipython.magic("timeit -n 10 -r 7 for i in range(0, 10000): dc[i]")

    print('cachetools.FIFO set/delete')
    dc = cachetools.FIFOCache(maxsize=10000)
    ipython.magic("timeit -n 10 -r 7 for i in range(0, 10000): dc[i] = value * i; del dc[i]")
