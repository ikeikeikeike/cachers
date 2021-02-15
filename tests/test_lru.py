import unittest

from cachers import LRUCache

from . import CacheTestMixin


class LRUCacheTest(unittest.TestCase, CacheTestMixin):

    Cache = LRUCache

    def test_lru(self):
        cache = LRUCache(maxsize=2)

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
        self.assertEqual(cache[2], 2)
        self.assertEqual(cache[4], 4)
        self.assertNotIn(3, cache)

        cache[5] = 5
        self.assertEqual(len(cache), 2)
        self.assertEqual(cache[4], 4)
        self.assertEqual(cache[5], 5)
        self.assertNotIn(2, cache)

    def test_lru_order(self):
        cache = LRUCache(maxsize=100)

        for i in range(0, 100):
            cache[i] = i
        for i in reversed(range(0, 100)):
            self.assertEqual(cache[i], i)

        for left, (right, _) in zip(reversed(range(0, 100)), cache.data):
            self.assertEqual(left, right)
