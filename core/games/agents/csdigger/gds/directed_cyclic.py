# DSG : directed cyclic graph

"""
White >> the vertex hasn't been visited yet
Gray >> we've visited the vertex but haven't visited all vertices in its subtree ... its children 
Black >> we've visited all vertices in subtree and left the vertex ... has been saw all its children 
NOTE Initially all vertices have white color 
NOTE When we visit the vertex, we should paint it with gray color 
NOTE When we leave the vertex we paint it with black color
TODO algorithm:::::::
     - IGNORE vertex painted black (since we have processed)
     - ERROR if vertex is GRAY (cycle!!!)
     - process vertex if WHITE
""" 

def cycle(G):
    # we color all nodes white in intialization process
    color = { u : "white" for u in G  }
    # we define find_cycle variable a s list so later on we can change this
    found_cycle = [False]

    # visiting all node , start iteration
    for u in G:                          
        if color[u] == "white":
            dfs(G, u, color, found_cycle)
        if found_cycle[0]: # if we found a cycle for u 
            break
    return found_cycle[0]


def dfs(G, u, color, found_cycle):
    # stop the dfs algo if we found a cycle
    if found_cycle[0]:                          
        return
    # Gray nodes are in the current path ... sort of the children of parent node
    color[u] = "gray"
    # G[u] is a list of all children nodes
    for v in G[u]:
        if v in color:
            if color[v] == "gray": 
                found_cycle[0] = True       
                return
            # we have call dfs algo on it recursively fot all the children also
            if color[v] == "white":   
                dfs(G, v, color, found_cycle)
        else:
            break
    color[u] = "black"



# -------------------------------------------
graph =           { 1 : [2],
                    2 : [3, 4],
                    4 : [6, 5],
                    5 : [6],
                    6 : [3]
                  }


print("[+] HAS CYCLE >>> ", cycle(graph)) 



graph2       = { 0 : [1],
                    1 : [2],
                    2 : [3],
                    3 : [4],
                    4 : [3] }



print("[+] HAS CYCLE >>> ", cycle(graph2)) 