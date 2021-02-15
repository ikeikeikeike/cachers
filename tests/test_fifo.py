import unittest

from cachers import FIFOCache

from . import CacheTestMixin


class FIFOCacheTest(unittest.TestCase, CacheTestMixin):

    Cache = FIFOCache

    def test_fifo(self):
        cache = FIFOCache(maxsize=2)

        cache[1] = 1
        cache[2] = 2
        cache[3] = 3

        self.assertEqual(len(cache), 2)
        self.assertEqual(cache[2], 2)
        self.assertEqual(cache[3], 3)
        self.assertNotIn(1, cache)

        cache[2]
        cache[4] = 4
        self.assertEqual(len(cache), 2)
        self.assertEqual(cache[3], 3)
        self.assertEqual(cache[4], 4)
        self.assertNotIn(2, cache)

        cache[5] = 5
        self.assertEqual(len(cache), 2)
        self.assertEqual(cache[4], 4)
        self.assertEqual(cache[5], 5)
        self.assertNotIn(3, cache)

    def test_fifo_order(self):
        cache = FIFOCache(maxsize=100)

        for i in range(0, 100):
            cache[i] = i
        for i in range(0, 100):
            self.assertEqual(cache[i], i)

        for i in range(200, 250):
            cache[i] = i
        for i in range(50, 100):
            self.assertEqual(cache[i], i)
        for i in range(200, 250):
            self.assertEqual(cache[i], i)
        for left, (right, _) in zip(list(range(50, 100)) + list(range(200, 250)), cache.keys):
            self.assertEqual(left, right)
