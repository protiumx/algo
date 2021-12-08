import queue

DIRECTIONS = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
]


def minesweeper(board, click):
    clickX, clickY = click[0], click[1]
    if board[clickX][clickY] == "M":
        board[clickX][clickY] = "X"
        return board

    board[clickX][clickY] = "B"
    m, n = len(board), len(board[0])
    q = queue.Queue()
    q.put(clickX*n + clickY)

    while not q.empty():
        point = q.get()
        px, py = point // n, point % n
        mines = __get_adjacent_mines(board, px, py, m, n)
        if mines > 0:
            board[px][py] = str(mines)
        else:
            for direction in DIRECTIONS:
                i, j = px + direction[0], py + direction[1]
                if __is_valid_index(i, j, m, n) and board[i][j] == "E":
                    board[i][j] = "B"
                    q.put(i * n + j)
    return board


def minesweeper_dfs(board, click):
    clickX, clickY = click[0], click[1]
    m, n = len(board), len(board[0])
    if board[clickX][clickY] == "M":
        board[clickX][clickY] = "X"
    else:
        __dfs(board, clickX, clickY, m, n)
    return board


def __dfs(board, i, j, m, n):
    if not __is_valid_index(i, j, m, n) or board[i][j] != "E":
        return

    mines = __get_adjacent_mines(board, i, j, m, n)
    if mines > 0:
        board[i][j] = mines.toString()
    else:
        board[i][j] = "B"
        for direction in DIRECTIONS:
            __dfs(board, i + direction[0], j + direction[1], m, n)


def __get_adjacent_mines(board, x, y, m, n):
    count = 0
    for direction in DIRECTIONS:
        i, j = x + direction[0], y + direction[1]
        if __is_valid_index(i, j, m, n) and board[i][j] == "M":
            count += 1
    return count


def __is_valid_index(i, j, m, n):
    return 0 <= i and i < m and 0 <= j and j < n
