#include <gtest/gtest.h>
#include <unordered_map>

using namespace std;

int most_frequent(int arr[], int n)
{
  int max_count = 0, res = -1;
  unordered_map<int, int> hash;
  for (int i = 0; i < n; i++)
  {
    hash[arr[i]]++;
    if (max_count < hash[arr[i]])
    {
      res = arr[i];
      max_count = hash[arr[i]];
    }
  }
  return res;
}

TEST(MostFrequent, FindsMostFrequent)
{
  int input[5] = {0, 1, 2, 1, 4};
  EXPECT_EQ(most_frequent(input, 5), 1);
}
