


# STATUS : incomplete

# https://codeforces.com/problemset/problem/1373/G


rows, k, m, min_changes, cols = 5, 3, 5, 0, 5

class List(list):
    def __getitem__(self, index):
        if type(index) == int and index > 0:
           index -= 1
        if type(index) == slice:
           start, stop = index.start, index.stop
           if start and start > 0:
              start -= 1
           if stop and stop > 0:
              stop -=  1
           index = slice(start, stop, index.step)
        return super().__getitem__(index)
    def __setitem__(self, index, val):
        super().__setitem__(index-1, val)
        

CHESSBOARD = List(List([ '__' for _ in range(1, rows+1)]) for _ in range(1, rows+1))
inputs = [(4, 4), (3, 5), (2, 4), (3, 4), (3, 5)]

for points in inputs:
	if CHESSBOARD[points[1]][points[0]] != '_p_':
		CHESSBOARD[points[1]][points[0]] = '_p_'


def add_row():
	global min_changes, rows
	CHESSBOARD.insert(1, ['__' for i in range(1, cols+1)])
	min_changes += 1
	rows += 1


def move_pawn(point):
	
	pawn_added = False
	col = point[0]
	row = point[1]


	print(f"trying to move pawn from cell ({col},{row})")
	if row+1 <= rows:
		if col == k:				
			if CHESSBOARD[row+1][col] != '_p_':
				print(f"into cell ({col}, {row+1})")
				CHESSBOARD[row+1][col] = '_p_'
				CHESSBOARD[row][col] = '__'
			if CHESSBOARD[row+1][col] == '_p_':
				print(f"can't put in cell ({col},{row+1}) there is already a pawn in cell ({col}, {row+1})")
				CHESSBOARD[row][col] = '__'

		elif col-1 == k:
			if CHESSBOARD[row+1][col-1] != '_p_':
				print(f"putting pawn in cell ({col-1}, {row+1})")
				CHESSBOARD[row+1][col-1] = '_p_'
				CHESSBOARD[row][col] = '__'
			if CHESSBOARD[row+1][col-1] == '_p_':
				print(f"can't put in cell ({col-1},{row+1}) there is already a pawn in cell ({col-1}, {row+1})")
				CHESSBOARD[row][col] = '__'

		elif col+1 == k:
			if CHESSBOARD[row+1][col+1] != '_p_':
				print(f"putting pawn in cell ({col+1}, {row+1})")
				CHESSBOARD[row+1][col+1] = '_p_'
				CHESSBOARD[row][col] = '__'
			if CHESSBOARD[row+1][col+1] == '_p_':
				print(f"can't put in cell ({col+1},{row+1}) there is already a pawn in cell ({col+1}, {row+1})")
				CHESSBOARD[row][col] = '__'

	else:
		print("adding row")
		add_row()
		move_pawn(point)



if __name__ == "__main__":
	for i in range(m):
		move_pawn(inputs[i])
		print(min_changes)
		min_changes = 0

	print(CHESSBOARD)