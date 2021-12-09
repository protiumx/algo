import { minesweeper, minesweeperDFS } from "$/algo_01";

describe("algo-01", () => {
  const input = [
    ["E", "E", "E", "E", "E"],
    ["E", "E", "M", "E", "E"],
    ["E", "E", "E", "E", "E"],
    ["E", "E", "E", "E", "E"],
  ];
  const expected = [
    ["B", "1", "E", "1", "B"],
    ["B", "1", "M", "1", "B"],
    ["B", "1", "1", "1", "B"],
    ["B", "B", "B", "B", "B"],
  ];

  it("it should update board properly", () => {
    expect(minesweeper(input, [3, 0])).toEqual(expected);
    expect(
      minesweeper(
        [
          ["M", "E"],
          ["E", "E"],
        ],
        [0, 0]
      )
    ).toEqual([
      ["X", "E"],
      ["E", "E"],
    ]);
  });

  it("it should update board properly with DFS", () => {
    expect(minesweeperDFS(input, [3, 0])).toEqual(expected);
    expect(
      minesweeperDFS(
        [
          ["M", "E"],
          ["E", "E"],
        ],
        [0, 0]
      )
    ).toEqual([
      ["X", "E"],
      ["E", "E"],
    ]);
  });
});
