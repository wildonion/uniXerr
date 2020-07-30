





# STATUS : 

# https://codeforces.com/problemset/problem/1374/C


import sys


def is_reg(s):
	if len(s) % 2 == 0:
		open_brack  = s[:int(len(s)/2)]
		close_brack = s[int(len(s)/2):]
		
		for i in range(len(open_brack)):
			if open_brack[i] != "(":
				print("not reg brack")
				sys.exit(1)

		for j in range(len(close_brack)):
			if close_brack[j] != ")":
				print("not reg brack")
				sys.exit(1)

	else:
		print("the length must be even!")
		sys.exit(1)



_input = "( () ) ()"
s = _input.replace(" ", "")
is_reg(_input)
