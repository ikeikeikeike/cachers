import cachers


VALUE = '*'


def _set(mod, klass):
    dc = getattr(mod, klass)(maxsize=1000)

    def inner():
        for i in range(0, 10000):
            dc[i] = VALUE * i

    return inner


def _get(mod, klass):
    dc = getattr(mod, klass)(maxsize=10000)

    for i in range(0, 10000):
        dc[i] = VALUE * i

    def inner():
        for i in range(0, 10000):
            dc[i]

    return inner


def _set_and_get(mod, klass):
    dc = getattr(mod, klass)(maxsize=10000)

    def inner():
        for i in range(0, 10000):
            dc[i] = VALUE * i

        for i in range(0, 10000):
            dc[i]

    return inner


def _set_and_delete(mod, klass):
    dc = getattr(mod, klass)(maxsize=10000)

    def inner():
        for i in range(0, 10000):
            dc[i] = VALUE * i
            del dc[i]

    return inner


def test_fifo_set(benchmark):
    benchmark(_set(cachers, "FIFOCache"))


def test_fifo_get(benchmark):
    benchmark(_get(cachers, "FIFOCache"))


def test_fifo_set_and_get(benchmark):
    benchmark(_set_and_get(cachers, "FIFOCache"))


def test_fifo_set_and_delete(benchmark):
    benchmark(_set_and_delete(cachers, "FIFOCache"))


def test_lru_set(benchmark):
    benchmark(_set(cachers, "LRUCache"))


def test_lru_get(benchmark):
    benchmark(_get(cachers, "LRUCache"))


def test_lru_set_and_get(benchmark):
    benchmark(_set_and_get(cachers, "LRUCache"))


def test_lru_set_and_delete(benchmark):
    benchmark(_set_and_delete(cachers, "LRUCache"))


try:
    import cachetools
except ImportError:
    print('Error: Cannot import cachetools')
else:
    def test_fifo_set_cachetools(benchmark):
        benchmark(_set(cachetools, "FIFOCache"))

    def test_fifo_get_cachetools(benchmark):
        benchmark(_get(cachetools, "FIFOCache"))

    def test_fifo_set_and_get_cachetools(benchmark):
        benchmark(_set_and_get(cachetools, "FIFOCache"))

    def test_fifo_set_and_delete_cachetools(benchmark):
        benchmark(_set_and_delete(cachers, "FIFOCache"))

    def test_lru_set_cachetools(benchmark):
        benchmark(_set(cachers, "LRUCache"))

    def test_lru_get_cachetools(benchmark):
        benchmark(_get(cachers, "LRUCache"))

    def test_lru_set_and_get_cachetools(benchmark):
        benchmark(_set_and_get(cachers, "LRUCache"))

    def test_lru_set_and_delete_cachetools(benchmark):
        benchmark(_set_and_delete(cachers, "LRUCache"))
