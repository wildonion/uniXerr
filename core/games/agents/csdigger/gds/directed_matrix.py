
class graph:
	def __init__(self):
		self.vertices = {}
		self.edges = []
		self.directed_matrix = []

	def add_vertex(self, v):
		if type(v) == list:
			for node in v:
				if node in self.vertices.keys(): # we don't need to check this cause dictionary keys is unique! the python dict handle that for us.
					continue
				else:
					self.vertices[node] = []
		else:
			self.vertices[v] = []

	def add_edge(self, vertices):
		if type(vertices) == list:
			for edge in vertices:
				_from = edge[0]
				_to   = edge[1]
				_ed   = edge[2]
				doesnt_exist_from = _from not in self.vertices.keys()
				doesnt_exist_to = _to not in self.vertices.keys()
				if doesnt_exist_from or doesnt_exist_to:
					print(f"vertex {_from} or {_to} doesn't exits in graph")
					continue
				else:
					self.edges.append(_ed)
					self.vertices[_from].append(_to)
					self.directed_matrix.append([_from, _to, _ed])
		elif type(vertices) == tuple:
			_from = edge[0]
			_to   = edge[1]
			_ed   = edge[2]
			doesnt_exist_from = _from not in self.vertices.keys()
			doesnt_exist_to = _to not in self.vertices.keys()
			if doesnt_exist_from or doesnt_exist_to:
				print(f"vertex {_from} or {_to} doesn't exits in graph")
				pass
			else:
				self.edges.append(_ed)
				self.vertices[_from].append(_to)
				self.directed_matrix.append([_from, _to, _ed])

	def get_vertices(self):
		print("\nSHOWING VERTICES\n")
		print(f"{list(self.vertices.keys())}")
		print()


	def get_edges(self):
		print("\nSHOWING EDGES\n")
		print(f"{self.edges}")
		print()

	def visualize(self):
		print("\nSHOWING GRAPH STRUCTURE\n")
		for key in self.vertices:
			for node in self.vertices[key]:
				print(f"{key} -> {node}")


	def visualize_matrix(self):
		print("\nSHOWING DIRECTED MATRIX\n")
		print("FROM\tTO\tEDGE")
		for row in range(len(self.directed_matrix)):
			for col in range(len(self.directed_matrix[0])):
				try:
					print(f"{self.directed_matrix[row][col]}", end="\t")
				except IndexError:
					print("\n[!] unequal dims")
					sys.exit(1)
			print()



g = graph()
g.add_vertex(['0', '1', '2', '3', '4', '4'])
g.add_edge([('0', '1', '-10->'), ('0', '2', '-4->'), ('1', '4', '-5->'), ('3', '4', '-6->'), ('1', '3', '-7->'), 
			('2', '4', '-13->'), ('2', '0', '-14->'), ('3', '0', '-9->'), ('4', '2', '-12->'), ('4', '0', '-8->')])
g.visualize()
g.get_vertices()
g.get_edges()
g.visualize_matrix()