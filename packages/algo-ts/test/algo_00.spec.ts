import { mostFrequent } from "$/algo_00";

describe("algo_00", () => {
  it("should find the most frequent element", () => {
    const input = [1, 2, 4, 1, 4, 5, 2, 1];
    expect(mostFrequent(input)).toEqual(1);
  });
});
