def min_path_sum(grid):
    m = len(grid)
    n = len(grid[0])
    costs = [[0 for _ in range(n)] for _ in range(m)]
    costs[0][0] = grid[0][0]
    for i in range(1, m):
        costs[i][0] = grid[i][0] + costs[i-1][0]
    for j in range(1, n):      
        costs[0][j] = grid[0][j] + costs[0][j-1]
    for i in range(1, m):
        for j in range(1, n):
            costs[i][j] = grid[i][j] + min(costs[i-1][j], costs[i][j-1])

    return costs[m-1][n-1]
