





# STATUS : 

# https://codeforces.com/problemset/problem/1374/C


import sys


min_ops = 0
open_stack = []
close_stack = []


def move_ith(s):
	global min_ops


def is_reg(s):
	global open_stack, close_stack
	for i in range(1, len(s)):
		if s[i] == "(":
			open_stack.append(s[i])
		if s[i] == ")":
			if i == 0:
				return False
				break
			else:
				close_stack.append(s[i])
	if len(open_stack) == len(close_stack):
		return True
	else:
		return False


test_cases, i = int(input("test cases >>> ")), 0
while i < test_cases < 2000:
	_input = input()
	s = _input.replace(" ", "")
	if 2 <= len(s) <= 50:
		if len(s) % 2 == 0:
			if not is_reg(s):
				print("making it reg...")
				move_ith(s)
		else:
			print("the length must be even!")
			sys.exit(1)
	print("=============")
	i+=1
