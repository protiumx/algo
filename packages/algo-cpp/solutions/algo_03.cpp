#include <gtest/gtest.h>
#include <iostream>
#include <stdio.h>

using namespace std;

int max_substr(string s) {
  int max_count = 0;
  int start = 0;
  // We need space for all ASCII chars
  int indexes[128];
  fill_n(indexes, 127, -1);

  for (int i = 0; i < s.length(); i++) {
    const char c = s[i];
    if (indexes[c] >= start) {
      start = indexes[c] + 1;
    }
    indexes[c] = i;
    max_count = max(max_count, i - start + 1);
  }
  return max_count;
}

TEST(MaxSubstr, Test) {
  string input = "abcb13*jb";
  const int expected = 6;
  ASSERT_EQ(max_substr(input), expected);
}
