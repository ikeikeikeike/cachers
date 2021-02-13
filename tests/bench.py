from IPython import get_ipython

import cachers

ipython = get_ipython()
assert ipython is not None, 'No IPython! Run with $ ipython ...'

value = 'value' * 1024

print('FIFO set')
dc = cachers.FIFOCache(maxsize=10000)
ipython.magic("timeit -n 100000 -r 7 dc['key'] = value")
print('FIFO get')
ipython.magic("timeit -n 100000 -r 7 dc['key']")
print('FIFO set/delete')
ipython.magic("timeit -n 100000 -r 7 dc['key'] = value; del dc['key']")

print("\n", "-" * 10, "\n")

try:
    import cachetools
except ImportError:
    print('Error: Cannot import cachetools')
else:
    print('cachetools.FIFO set')
    dc = cachetools.FIFOCache(maxsize=10000)
    ipython.magic("timeit -n 100000 -r 7 dc['key'] = value")
    print('cachetools.FIFO get')
    ipython.magic("timeit -n 100000 -r 7 dc['key']")
    print('cachetools.FIFO set/delete')
    ipython.magic("timeit -n 100000 -r 7 dc['key'] = value; del dc['key']")
