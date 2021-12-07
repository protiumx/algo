import unittest
from solutions import most_frequent


class TestAlgo_00(unittest.TestCase):
    def test(self):
        input = [1, 2, 3, 2, 4, 1, 1]
        self.assertEqual(most_frequent(input), 1)


if __name__ == '__main__':
    unittest.main()
