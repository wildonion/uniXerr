
# UDCG : undirected cyclic graph

class Graph:
    def __init__(self):
        self.vertices = {}
 
    def add_vertex(self, key):
        vertex = Vertex(key)
        self.vertices[key] = vertex
 
    def get_vertex(self, key):
        return self.vertices[key]
 
    def add_edge(self, src_key, dest_key, weight=1):
        self.vertices[src_key].add_neighbour(self.vertices[dest_key], weight)
 
    def add_undirected_edge(self, v1_key, v2_key, weight=1):
        self.add_edge(v1_key, v2_key, weight)
        self.add_edge(v2_key, v1_key, weight)
 
    def does_undirected_edge_exist(self, v1_key, v2_key):
        return (self.does_edge_exist(v1_key, v2_key)
                and self.does_edge_exist(v1_key, v2_key))
 
    def does_edge_exist(self, src_key, dest_key):
        return self.vertices[src_key].does_it_point_to(self.vertices[dest_key])
 
    def __iter__(self):
        return iter(self.vertices.values())
 
 
class Vertex:
    def __init__(self, key):
        self.key = key
        self.points_to = {}
 
    def get_key(self):
        return self.key
 
    def add_neighbour(self, dest, weight):
        self.points_to[dest] = weight
 
    def get_neighbours(self):
        return self.points_to.keys()
 
    def get_weight(self, dest):
        return self.points_to[dest]
 
    def does_it_point_to(self, dest):
        return dest in self.points_to
 
 
def is_cycle_present(v, visited):
    parent = {v: None}
    return is_cycle_present_helper(v, visited, parent)
 
 
def is_cycle_present_helper(v, visited, parent):
    visited.add(v)
    for dest in v.get_neighbours():
        if dest not in visited:
            parent[dest] = v
            if is_cycle_present_helper(dest, visited, parent):
                return True
        else:
            if parent[v] is not dest:
                return True
    return False
 
g = Graph()
print('===================\nOPERATION ON UNDIRECTED GRAPH\n===================')
print('[+] add vertex <key> : eg add vertex 1')
print('[+] add edge <vertex1> <vertex2> : eg add edge 2 3 ')
print('[+] check cycle')
print('[+] graph info')
print('[+] exit')
 
while True:
    do = input('Q->WHAT DO U WANT TO DO >>>> ').split()
 
    operation = do[0]
    if operation == 'add':
        suboperation = do[1]
        if suboperation == 'vertex':
            key = int(do[2])
            if key not in g:
                g.add_vertex(key)
            else:
                print('[!] vertex exists.')
        elif suboperation == 'edge':
            v1 = int(do[2])
            v2 = int(do[3])
            if v1 not in g:
                print('[!] vertex {} does not exist.'.format(v1))
            elif v2 not in g:
                print('[!] vertex {} does not exist.'.format(v2))
            else:
                if not g.does_undirected_edge_exist(v1, v2):
                    g.add_undirected_edge(v1, v2)
                else:
                    print('[!] edge exists.')
 
    elif operation == 'check':
        present = False
        visited = set()
        for v in g:
            if v not in visited:
                if is_cycle_present(v, visited):
                    present = True
                    break
 
        if present:
            print('[+] THIS GRAPH HAS CYCLE')
        else:
            print('[+] THIS GRAPH HAS NO CYCLE!')
 
    elif operation == 'display':
        print('[*] all vertices: ', end='')
        for v in g:
            print(v.get_key(), end=' ')
        print()
 
        print('[*] all edges: ')
        for v in g:
            for dest in v.get_neighbours():
                w = v.get_weight(dest)
                print('(source={}, destination={}, weight={}) '.format(v.get_key(),
                                                             dest.get_key(), w))
        print()
 
    elif operation == 'quit':
        break