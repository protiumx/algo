import {MinPathSum} from "$/algo_04";

describe("algo-04", () => {
  it("should find the minimum path", () => {
    const input = [
      [1, 3, 1],
      [1, 5, 1],
      [4, 2, 1],
    ];
    expect(MinPathSum(input)).toEqual((7))
  });
});
