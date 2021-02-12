import sys
import unittest


class CacheTestMixin:

    Cache = None

    def test_defaults(self):
        cache = self.Cache(maxsize=1)
        self.assertEqual(0, len(cache))
        self.assertEqual(1, cache.maxsize)
        self.assertEqual(0, cache.currsize)
        self.assertTrue(repr(cache).startswith(cache.__class__.__name__))

    def test_insert(self):
        cache = self.Cache(maxsize=2)

        cache.update({1: 1, 2: 2})
        self.assertEqual(2, len(cache))
        self.assertEqual(1, cache[1])
        self.assertEqual(2, cache[2])

        cache[3] = 3
        self.assertEqual(2, len(cache))
        self.assertEqual(3, cache[3])
        self.assertTrue(1 in cache or 2 in cache)

        cache[4] = 4
        self.assertEqual(2, len(cache))
        self.assertEqual(4, cache[4])
        self.assertTrue(1 in cache or 2 in cache or 3 in cache)

    def test_update(self):
        cache = self.Cache(maxsize=2)

        cache.update({1: 1, 2: 2})
        self.assertEqual(2, len(cache))
        self.assertEqual(1, cache[1])
        self.assertEqual(2, cache[2])

        cache.update({1: 1, 2: 2})
        self.assertEqual(2, len(cache))
        self.assertEqual(1, cache[1])
        self.assertEqual(2, cache[2])

        cache.update({1: 'a', 2: 'b'})
        self.assertEqual(2, len(cache))
        self.assertEqual('a', cache[1])
        self.assertEqual('b', cache[2])

    def test_delete(self):
        cache = self.Cache(maxsize=2)

        cache.update({1: 1, 2: 2})
        self.assertEqual(2, len(cache))
        self.assertEqual(1, cache[1])
        self.assertEqual(2, cache[2])

        del cache[2]
        self.assertEqual(1, len(cache))
        self.assertEqual(1, cache[1])
        self.assertNotIn(2, cache)

        del cache[1]
        self.assertEqual(0, len(cache))
        self.assertNotIn(1, cache)
        self.assertNotIn(2, cache)

        with self.assertRaises(KeyError):
            del cache[1]
        self.assertEqual(0, len(cache))
        self.assertNotIn(1, cache)
        self.assertNotIn(2, cache)

    def test_pop(self):
        cache = self.Cache(maxsize=2)

        cache.update({1: 1, 2: 2})
        self.assertEqual(2, cache.pop(2))
        self.assertEqual(1, len(cache))
        self.assertEqual(1, cache.pop(1))
        self.assertEqual(0, len(cache))

        with self.assertRaises(KeyError):
            cache.pop(2)
        with self.assertRaises(KeyError):
            cache.pop(1)
        with self.assertRaises(KeyError):
            cache.pop(0)

        self.assertEqual(None, cache.pop(2, None))
        self.assertEqual(None, cache.pop(1, None))
        self.assertEqual(None, cache.pop(0, None))

    def test_popitem(self):
        cache = self.Cache(maxsize=2)

        cache.update({1: 1, 2: 2})
        self.assertIn(cache.pop(1), {1: 1, 2: 2})
        self.assertEqual(1, len(cache))
        self.assertIn(cache.pop(2), {1: 1, 2: 2})
        self.assertEqual(0, len(cache))

        with self.assertRaises(KeyError):
            cache.popitem()

    @unittest.skipUnless(sys.version_info >= (3, 7), 'requires Python 3.7')
    def test_popitem_exception_context(self):
        # since Python 3.7, MutableMapping.popitem() suppresses
        # exception context as implementation detail
        exception = None
        try:
            self.Cache(maxsize=2).popitem()
        except Exception as e:
            exception = e
        self.assertIsNone(exception.__cause__)
        self.assertTrue(exception.__suppress_context__)

    def test_pickle(self):
        import pickle

        source = self.Cache(maxsize=2)
        source.update({1: 1, 2: 2})

        cache = pickle.loads(pickle.dumps(source))
        self.assertEqual(source, cache)

        self.assertEqual(2, len(cache))
        self.assertEqual(1, cache[1])
        self.assertEqual(2, cache[2])

        cache[3] = 3
        self.assertEqual(2, len(cache))
        self.assertEqual(3, cache[3])
        self.assertTrue(1 in cache or 2 in cache)

        cache[4] = 4
        self.assertEqual(2, len(cache))
        self.assertEqual(4, cache[4])
        self.assertTrue(1 in cache or 2 in cache or 3 in cache)

        self.assertEqual(cache, pickle.loads(pickle.dumps(cache)))

    def test_pickle_maxsize(self):
        import pickle
        import sys

        # test empty cache, single element, large cache (recursion limit)
        for n in [0, 1, sys.getrecursionlimit() * 2]:
            source = self.Cache(maxsize=n)
            source.update((i, i) for i in range(n))
            cache = pickle.loads(pickle.dumps(source))
            self.assertEqual(n, len(cache))
            self.assertEqual(source, cache)
