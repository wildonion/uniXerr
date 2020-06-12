# =====================================

import random

# BST problem

NODES = []

class Node:

	def __init__(self, data):

		self.left = None
		self.right = None
		self.data = data

# ======================
# insert ops
# ======================
	def insert(self, data):

		if self.data:
			if data[0] < self.data[0]:
				if self.left is None:
					self.left = Node(data)
					NODES.append({"left":self.left.data})
					print("THE NODES ARE", NODES)
					print("[+] left node created cause self.left was None and {} is less than {}\n".format(data, self.data))
				else:
					self.left.insert(data)
					print("[+] inserted into left cause self.left was not None and it's filling its left node up\n")
			elif data[0] > self.data[0]:
				if self.right is None:
					self.right = Node(data)
					NODES.append({"right":self.right.data})
					print("THE NODES ARE", NODES)
					print("[+] right node created cause self.right was None and {} is greater than {} \n".format(data, self.data))
				else:
					self.right.insert(data)
					print("[+] inserted into right cause self.right was not None and it's filling its right node up\n")
		else:
			self.data = data
# ======================
# finding method
# ======================
	def findval(self, lkpval):
		for node in NODES:
			for key in node:
				if lkpval == node[key][2]:
					if node[key][0] < self.data[0]:
						if self.left is None:
							return str(lkpval)+" Not Found"
						return self.left.findval(lkpval)
					elif node[key][0] > self.data[0]:
						if self.right is None:
							return str(lkpval)+" Not Found"
						return self.right.findval(lkpval)
					else:
						print(str(self.data) + ' is found')
# ======================
# print the tree
# ======================
	def PrintTree(self):
		if self.left:
			self.left.PrintTree()
		print( self.data),
		if self.right:
			self.right.PrintTree()

def run_ex():
	# if __name__ == "__main__": # can't run in another file if uncomment this; cause that file is not this process
	# ======================
	# test the app
	# ======================
	try:
		root = Node((0, 1111111, 'A_lname'))
		print("[!] THE ROOT IS {}".format(root.data))
		for i in range(int(input("Number Of Students >>> "))):
			root.insert((random.randint(i+1,30), int(input("enter stuid >>> ")), input("enter lastname >>> ")))

		
		print("\n=========================\n")
		print("TRY TO FIND SOMEONE !!!! ")
		print("\n=========================\n")
		while True:
			who = input("enter a last name to find it (press * on keyboard to stop the process) >>>> ")
			print(root.findval(who))
			if who is '*' and type(who) is str:
				break
		print("==============\nTHE WHOLE TREE\n==============")
		root.PrintTree()
	except KeyboardInterrupt:
		exit(1)