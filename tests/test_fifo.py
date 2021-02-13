import unittest

from cachers import FIFOCache

from . import CacheTestMixin


class LRUCacheTest(unittest.TestCase, CacheTestMixin):

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

    def test_fifo_str(self):
        cache = FIFOCache(maxsize=2)

        cache["1"] = "1"
        cache["2"] = "2"
        cache["3"] = "3"

        self.assertEqual(len(cache), 2)
        self.assertEqual(cache["2"], "2")
        self.assertEqual(cache["3"], "3")
        self.assertNotIn("1", cache)

        cache["2"]
        cache["4"] = "4"
        self.assertEqual(len(cache), 2)
        self.assertEqual(cache["3"], "3")
        self.assertEqual(cache["4"], "4")
        self.assertNotIn("2", cache)

        cache["5"] = "5"
        self.assertEqual(len(cache), 2)
        self.assertEqual(cache["4"], "4")
        self.assertEqual(cache["5"], "5")
        self.assertNotIn("3", cache)

