





# STATUS : 

# https://codeforces.com/problemset/problem/1374/C


min_ops = 0
open_stack = []

def move_ith(s, index):
	global min_ops
	if s[index] == ")": # couldn't find the openning bracket for that
		s = s[index : ] + s[ : index] 
		index = is_reg(s)
		if index == -1:
			return -1
		else:
			move_ith(s, index)

	elif s[index] == "(": # couldn't find the closing bracket for that
		pass
	else:
		pass


def is_reg(s):
	global open_stack
	for i in range(len(s)):
		if s[i] == "(":
			if i == len(s)-1:
				return i
			else:
				open_stack.append(s[i])
		if s[i] == ")":
			if i == 0:
				return 0
			if open_stack:
				if open_stack[len(open_stack)-1] == "(":
					open_stack.pop()
			else:
				return i
	return -1


test_cases, i = int(input("test cases >>> ")), 0
while i < test_cases < 2000:
	_input = input()
	s = _input.replace(" ", "")
	if 2 <= len(s) <= 50:
		if len(s) % 2 == 0:
			index = is_reg(s)
			if index != -1:
				move_ith(s, index)
			else:
				print("no need to make it reg")
		else:
			print("the length must be even!")
			break
	print("=============")
	i+=1
