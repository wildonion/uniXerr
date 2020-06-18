

# path finding with an unknown algorithm!!!
# we want to have a path from D to A with a minimum time

import random

graph = {"A<->B": 2, "A<->C": 2, "B<->D": 6, "C<->D": 2, "D<->E": 5, "A<->E": 3}
path  = "D<->A"
nodes = path.split("<->")

def minTime(nodes):
    return min([nodes[i][1] for i in range(len(nodes))])

def solve(method): # TODO : solve with various algorithms for csdigger project
    selectedNodesToStart, find, selectedNodesToEnd = [], False, []
    for node in graph:
        graph_nodes = node.split("<->")
        if nodes[0] in graph_nodes:
            find = True
            selectedNodesToStart.append((node, graph[node]))
        if nodes[1] in graph_nodes:
            find = True
            selectedNodesToEnd.append((node, graph[node]))
    if not find:
        print("[-] can't find a node to start path searching")
    print("\n[+] printing all solutions ↵ \n")
    while len(selectedNodesToStart):
        best_node        = random.choice(list(filter(lambda t: t[1] == minTime(selectedNodesToStart), selectedNodesToStart)))
        best_nodeSplited = best_node[0].split("<->")
        FromNodeIndex    = best_nodeSplited.index(nodes[0])
        FromNode         = best_nodeSplited[FromNodeIndex]
        best_nodeSplited.pop(FromNodeIndex)
        toNode           = best_nodeSplited[0]
        print(f"\tmoving from {FromNode} to {toNode} .... ")
        # rest of the code here
        # ...
        selectedNodesToStart.pop(selectedNodesToStart.index(best_node))
