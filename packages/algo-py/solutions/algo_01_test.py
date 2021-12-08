import unittest
from solutions import *


input = [["E", "E", "E", "E", "E"], ["E", "E", "M", "E", "E"],
         ["E", "E", "E", "E", "E"], ["E", "E", "E", "E", "E"]]
expected = [["B", "1", "E", "1", "B"], ["B", "1", "M", "1", "B"],
            ["B", "1", "1", "1", "B"], ["B", "B", "B", "B", "B"]]


class TestAlgo_01(unittest.TestCase):

    def test(self):
        self.assertListEqual(minesweeper(input, [3, 0]), expected)

    def test_dfs(self):
        self.assertListEqual(minesweeper_dfs(input, [3, 0]), expected)


if __name__ == '__main__':
    unittest.main()
