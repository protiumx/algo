import unittest
from solutions import max_substr


class TestAlgo_03(unittest.TestCase):
    def test(self):
        input = "abcbghjkb"
        self.assertEqual(max_substr(input), 6)


if __name__ == '__main__':
    unittest.main()
