

# STATUS : buggy

# https://codeforces.com/problemset/problem/1375/G


import sys
from typing import List

def n_count_degree(nodes):
	nodes = [node[0] for node in nodes] + [node[1] for node in nodes]
	n_rep_node = {n: 0 for n in nodes}
	for node in nodes:
		if node in n_rep_node:
			n_rep_node[node]+=1
	return n_rep_node


class Node:
	def __init__(self, data):
		self.data: int = data
		self.children: List[Node] = []

	def add_child(self, node):
		self.children.append(node)

class Tree:
	def __init__(self, n, v_u):
		assert 3 <= n <= 2e+5
		assert 1 <= len(v_u) < n
		self.vertices    = n
		self.v_u         = v_u
		self.min_ops     = 0

		for i in range(len(v_u)):
			if v_u[i][0] == v_u[i][1]:
				print("same node in one tuple!")
				sys.exit(1)
			if 1 >= v_u[i][0] >= self.vertices:
				print("greater than node detected!")
				sys.exit(1)

		self.build_tree()
		self.show()


	def build_tree(self):

		self.tree = {Node(data): n_count_degree(self.v_u)[data] for data in n_count_degree(self.v_u)} # the value is the degree of each node

		for i in range(len(self.v_u)):
			for node in self.tree:
				if self.v_u[i][0] == node.data:
					child = self.v_u[i][1]
					node.add_child(Node(child))
				elif self.v_u[i][1] == node.data:
					child = self.v_u[i][0]
					node.add_child(Node(child))


	def is_star(self):
		n_rep_node = n_count_degree(self.v_u)
		rep_nodes = len(list(filter(lambda rep: True if rep>=2 else False, n_rep_node.values())))
		if rep_nodes >= 2: # more than 2 nodes are repeated
			return False
		elif rep_nodes == 1: # only one node is repeated
			return True
		else:
			print("Something went wrong!")
			sys.exit(1)


	def show(self):
		print("\n---------SHOWING TREE STRUCTURE---------\n")
		for node in self.tree:
			print(node.data,"____")
			if node.children:
				for child in node.children:
					print("\t____",child.data)


	def build_star(self):
		print("\n---------BUILDING STAR FROM TREE---------\n")
		'''
		**BUGGY**
			1 - Choose three vertices a, b, and c such that b is adjacent to both a and c.
			2 - For every vertex d other than b that is adjacent to a, remove the edge connecting d and a and add the edge connecting d and c.
			3 - Delete the edge connecting a and b and add the edge connecting a and c.
		'''
		for node in self.tree:
			if len(node.children) == 2:
				a = node.children[0]
				b = node
				c = node.children[1]
				print(f"choosing a = {a.data}, b = {b.data} and {c.data}.")
				if len(a.children) >= 1:
					for child in a.children:
						d = child
						c.add_child(d)
						a.children.remove(d)
					b.children.remove(a)
				if len(c.children) >= 1:
					for child in c.children:
						d = child
						a.add_child(d)
						c.children.remove(d)
					b.children.remove(c)
				self.min_ops += 1

		self.show()



if __name__ == "__main__":

	n = input("nodes >> ")
	print("edges [separated by space]: \n")
	v_u = list(map(lambda node: tuple((int(node[0]), int(node[1]))), [tuple(input().split(" ")) for _ in range(int(n)-1)]))


	t0 = Tree(int(n), v_u)
	if t0.is_star():
		print("tree is already a star!")
	else:
		t0.build_star()
		print(f"minimum operations >>> {t0.min_ops}")