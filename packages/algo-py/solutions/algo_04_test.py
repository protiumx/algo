import unittest
from solutions import min_path_sum


class TestAlgo_04(unittest.TestCase):
    def test(self):
        input = [
                [1,3,1],
                [1,5,1],
                [4,2,1],
                ]
        self.assertEqual(min_path_sum(input), 7)


if __name__ == '__main__':
    unittest.main()
