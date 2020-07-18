


# TODO: build a virtualenv like python's one has and compile all codes using pypy and cython in a virtualenv
# TODO: use multithreading to run It and Human code separately at the same time (itFollows algo)
# TODO: a new programming paradigm(must build in your own lang and its compiler) to implement abstract concepts like itFollows and cancer
# TODO: change the address of object in python in run-time to build a virus like stux net algo to change its address constantly and bypass AVs
# TODO: implement algorithm designing codes in its slide inside AI folder to complete simple AI projects here using advanced OOP(below codes) in python by getting help from geeksforgeeks.org
# TODO: use articles pdf in AI folder to understand all patterns(serach , sort and path finding) in graph , tree(BST and ...) , matrices DS and MDP env (A*, CSP, MDP, MCST..) and basic AI algos for agent creation with all graph formula
# TODO: binary prespective for solving problems using different data structures and base number and logical operators also think about maze algos and color handling in python
# TODO: solve all important cs algos using python DS concept like TSP
# TODO: dna computing using turing machine[encode/decode a dna] also create a woman using dna coding in python to combine ANN with dna
# TODO: try to extend algorithms using cython, ctypes, cffi or by writing an extension in C++


import os
import timeit
import random
import asyncio
from typing import Callable
import sys
import time
import math


print((lambda name, number: str(number)+name)(input("[+] Enter A Name : "), 456))

# =====================================
# overloading * operator
class int2(int):
    def __init__(self, x):
        # super(int2, self).__init__()
        self = x

    def __matmul__(self, i2):
        return self * i2 # i2 is int2 type


a = int2(2.3)
b = int2(3.5)
c = a @ b
print(c)
# -------------------
class int2():
    def __init__(self, x):
        self.value = x
        # self = x

    def __matmul__(self, i2):
        return int2(self.value * i2.value)


a = int2(2)
b = int2(3)
c = a @ b
print(c.value)

# =====================================

N = int(input("Enter a number "))
s = []
while N > 0:
    d = N//10
    r = N - 10*d
    s.append(r)
    N = d

d = 0
for i in range(len(s)):
    d += s[i]*math.pow(10,len(s)-i)/10

print(int(d))


# =====================================

class A():
    def __init__(self, a=None):
        self.a = a
        self.avar = "im a varible for A"
    
    def where(self):
        print("im inside {}".format(self.a))
        print(self.avar)
    


class B(A):
    def __init__(self, b=None):
        super().__init__("A")
        
    def edit(self):
        self.where()
        self.avar = "i edited a var ; now im a varible for B"
        print(self.avar)


if __name__ == "__main__":
    for i in range(4):
        b = B(b="{}".format(i))
        print(b)
        b.edit()

# -----------------------------

class A:
    def __init__(self, val):
        self.val = val
        print("[+] IM INSIDE A WIHT VALUE {}".format(self.val))
    
    def __a(self):
        class B(A):
            def __init__(self):
                print("[+] IM INSIDE B")
                super().__init__(67)
        return B()
    
    def b(self):
        self.__a()

if __name__ == "__main__":
    a = A(3)
    print(a.b())


# =====================================
# calculate all possible subset for a set
# base prespective for n nested loop and other dynamic problems!
# for i in [0..2]:
#     for j in [0..2]:
#         for p in [0..2]:
#             for q in [0..2]:

# 0 0 0 0 -> 0 0 0 2 -> 0 0 1 0 -> 0 0 1 2
# 0000, 0001, 0002, 0010, 0011, 0012, 0020, ..., 2222, (1)0000
# i j p q ...n ta...
# i1 i2 i3 ... in

# setlst = [[0,1,2], [0,1,2], [0,1,2]]
setlst = [["cs","wo","force", "shi"], ["cs","wo","force", "shi"], ["cs","wo","force", "shi"]]
n = len(setlst)
k = len(setlst[0])
idx = 0
lst = []

# METHOD 1
def select():
    global setlst, lst, idx
    lst = []
    t = idx
    for i in range(n):
        lst.append(setlst[i][t%k])
        t = t//k
    idx += 1
    return idx != n**k+1
while select():
    print(lst, "=>" , idx, ''.join(str(i) for i in lst))

# METHOD 2
i = [0 for _ in range(0,n)]
def finished():
    global i
    for j in i:
        if j is not 0:
            return False
    return True
def check():
    global i, k, n
    for j in range(len(i)):
        if i[j] >= k:
            i[j] = 0
            if j+1 < n:
                i[j+1] += 1
def inc():
    global i
    i[0] += 1
    check()

def select():
    global setlst, i, n, k
    tmp = []
    for j in range(len(i)):
        yield setlst[j][i[j]]

print([m for m in select()])
inc()
while not finished():
    print([m for m in select()])
    inc()

# METHOD 3
def com(lst):
    N = len(lst)
    for i in range(2**N):
        combo = []
        for j in range(N):
            if (i >> j) % 2 == 1:
                combo.append(lst[j])
                print(lst[j])
        yield combo

for i in com([1,2,3,5]):
    print(i, ", ", end="")

# METHOD 4 : TODO: fix the bugs and make it faster!!!!!! use multi processing using some AI algo
import copy
lst = [ [1,2,3], [4,5,6], [7,8,9] ]
y, combo, lt = [], [], copy.deepcopy(lst)
def firstElem(lst):
    if len(lst)>0:
        lst.remove(lst[0])
        return lst
    else:
        return
def allCombo(lst):
    global y, combo
    if len(lst)>0:
        a = lst[0][0]
        while a is not None:
            y.append(a)
            allCombo(firstElem(lst))
            lt[0].remove(a)
            a = lt[0][0]
        lt.remove(lt[0])
        allCombo(lt)
    else:
        combo.append(y)
    return combo
print(allCombo(lst))

# =====================================

# >>> d={"adventurous":"aventurero","bold":"audaz","courageous":"valiente"} 
# >>>d.items()
# [('courageous', 'valiente'), ('adventurous', 'aventurero'), ('bold', 'audaz')]
# >>> d.keys()
# ['courageous', 'adventurous', 'bold']
# >>> d.values()
# ['valiente', 'aventurero', 'audaz']
# >>> my_list = [('a', 1), ('b', 2)]
# >>> dict(my_list)
# {'a': 1, 'b': 2}


# TODO: runtime and time complexity issue!!! make the code simple!

def makelst(A):
    avglist = []
    for key in A.keys():
        avglist.append(float(A[key]["avg"]))
    return avglist

def sortavglst(avglist):
    n = len(avglist)
    for i in range(n):
        for j in range(0, n-i-1):
            if avglist[j] > avglist[j+1] :
                avglist[j], avglist[j+1] = avglist[j+1], avglist[j]
    return avglist
    

def createRATE(avgsortedlst, A):
    for key in A.keys():
        for i in range(len(avgsortedlst)):
            if A[key]["avg"] == avgsortedlst[i]:
                A[key]["rate"] = i+1

def calAVG(A):
    for key in A.keys():
        s = 0
        for i in A[key]["course_info"]:
            s+= float(i[1])
            avg = s/len(A[key]["course_info"])
            A[key]["avg"] = avg

def fillMe():
    A = {}
    for i in range(int(input("[+] REGISTERING FOR ? >>> "))):
        A[int(input("[+] STU ID >>> "))] = {"name": str(input("[+] NAME >>> ")), 
           "lname": str(input("[+] LASTNAME >>> ")),
           "avg": None,
           "rate": None,
           "course_info": [tuple(input("[+] WRITE COURSE NAME->MARK >>> ").split("->")) for cname in range(int(input("[+] COURSE NUMBER ? >>> ")))]
           }

    return A

def printMe(A):
    print("\n\n")
    for key in A.keys():
        print("STUID =========\n")
        print("{}\n".format(A[key]))
        print("FIRSTNAME ========\n")
        print("{}\n".format(A[key]["name"]))
        print("LASTNAME ========\n")
        print("{}\n".format(A[key]["lname"]))
        print("AVERAGE ========\n")
        print("{}\n".format(A[key]["avg"]))
        print("RATE ========\n")
        print("{}\n".format(A[key]["rate"]))
        print("COURSE INFO =========\n")
        for i in A[key]["course_info"]:
            print("{} -> {}".format(i[0], i[1]))
        print("\n")

if __name__ =="__main__":
    A = fillMe()
    calAVG(A)
    createRATE(sortavglst(makelst(A)), A)
    printMe(A)

# =====================================
# multi same dict key and their values

# METHOD 1
class Dictlist(dict):
    def __setitem__(self, key, value):
        try:
            self[key]
        except KeyError:
            super(Dictlist, self).__setitem__(key, [])
        self[key].append(value)

d = dictlist.Dictlist()
d['test'] = 1
d['test'] = 2
d['test'] = 3
# >>> d
# {'test': [1, 2, 3]}
d['other'] = 100
# >>> d
# {'test': [1, 2, 3], 'other': [100]}

# METHOD 2
class DictList(dict):
    def __setitem__(self, key, value):
        try:
            # Assumes there is a list on the key
            self[key].append(value) 
        except KeyError: # if fails because there is no key
            super(DictList, self).__setitem__(key, value)
        except AttributeError: # if fails because it is not a list
            super(DictList, self).__setitem__(key, [self[key], value])

dl = DictList()
dl['a'] = 1
dl['b'] = 2
dl['b'] = 3


# OUTPUT: {'a': 1, 'b': [2, 3]}

        
# =====================================

# topol algo

tsbst: list = [None]

def subset(sett: list) -> list:
        biN:Callable[str, str] = lambda el : ''.join(reversed([str((el>>i) & 1) for i in range(len(sett))]))
        tpx:int = 2**len(sett)
        for i in range(1,tpx):
                # breakpoint()
                # print(list(biN(i%tpx)))
                yield [i for i,j in zip(sett, list(biN(i%tpx))) if int(bool(i)) is int(j)]


# subset((lambda x : [input() for n in range(int(input("[+] NUMBER OF MEMBER SET : ")))])([]))
for sbst in subset(sys.argv[1].split(",")):
        tsbst.append(set(sbst))


smallestTopol: set = [None, tsbst[len(tsbst)-1]]
largestTopol: list = tsbst

print(f"\n[+] THE LARGEST TOPOLOGY IS\n\n\t {largestTopol}\n")
print(f"[+] THE SMALLEST TOPOLOGY IS\n\n\t {smallestTopol}\n")


# =====================================

# itFollows - black scary python virus

import time

class Human():
    def __init__(self):
        pass
    def __haveSex(self):
        pass
    def __isAlive(self):
        pass
    def __transpose(self):
        pass
    def __isFollow(self):
        pass

class It(Human):
    def __init__(self):
        pass
    def __walking(self):
        pass
    def __changeAvatar(self):
        pass


# =====================================

# python script to link the some urls into their related files

# we have a file called urls.txt and ex: script_1.py , script_2.py , script_3.py
# we want to make a link between urls exist in urls.txt and its relates script
# NOTE: this process is constantly running when a new script comes to the directory
# NOTE: this process will split the every script into two element => [0] : script , [1] : urls
# NOTE: this process will put all urls([1]) in urls.txt file and make a link to its related script([0])
# TODO: use multithreading for this purpose  





# =====================================

# Binary space partitioning structure






# =====================================

# CSPISSUES
# TODO: use all pdfs inside Introduction to Artificial Intelligence-Winter 2018(teach.cs.toronto.edu~csc384hwinterlectures.html) folder

# -----------------------
# N-QUEENS
# -----------------------
# N-Queens agent using CSP Constraint Propagation: ForwardChecking
variables = []
for i in range(N):
    variables.append('Q'+str(i+1))

# N-Queens using CSP Constraint Propagation: Generalized Arc Consistency








# -----------------------
# SUDOKU
# -----------------------
# sudoku agent using CSP Constraint Propagation: ForwardChecking
variables = []

# sudoku agent using CSP Constraint Propagation: ForwardChecking


# =====================================

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



# =====================================

# DFS binary tree

# ---------------
# DFS BINARY TREE
# ---------------


def dfs(tree, value):
    if tree.root is None:
        return []
    visited, stack, node = [], [tree.root], 0
    while stack:
        nde = stack.pop()
        if value == nde.value:
            node += 1
            print("[+] REACHED OUT TO NODE {0}".format(node))
            break
        else:
            node += 1
            visited.append(nde)
            stack.extend(filter(None, [nde.right, nde.left]))  
            # append right first, so left will be popped first
    return visited

class node(object):
    # initializing our nodes
    def __init__(self, value):
        self.value = value
        self.left = None
        self.right = None


class binary_tree(object):
    # initializing the root node by creating an object of our node class
    def __init__(self, root):
        self.root = node(root)


tree = binary_tree(1) # setting the value for root
tree.root.left = node(2) # setting the value for left side of root
tree.root.right = node(3) # setting the value for right side of root
tree.root.left.left = node(4) # setting the value for left side of the left side of the root
tree.root.left.right = node(5)
tree.root.right.right = node(6) # setting the value for right side of the right side of the root
tree.root.right.left = node(7)

print("[+] SEARCHING PATTERN : ", [v.value for v in dfs(tree, 4)])


# =====================================

# IDDS binary tree

# ----------------
# IDDS BINARY TREE
# ALGORITHMS 
# ----------------

# getting the maximum depth of the tree
def maxDepth(node):
    if node is None:
        return 0
    else:
        lDepth = maxDepth(node.left)
        rDepth = maxDepth(node.right)

        if lDepth>rDepth:
            return lDepth+1
        else:
            return rDepth+1

# getting the children of the desired node
def getChildren(node):
    children = []
    hasLeft = node.left is not None
    hasRight = node.right is not None
    if not hasLeft and not hasRight:
        return []
    if hasLeft:
        children.append(node.left)
    if hasRight:
        children.append(node.right)
    return children   

# idds algo
def iddfs(tree, value):
    print("[+] SEARCHING PATTERN ..")
    for depth in range(0, maxDepth(tree.root)):
        found, remaining = dls(tree.root, depth, value) # calling the dls algo recursively for every depth
        if found is not None:
            return str("[+] FOUND {0} AT DEPTH {1}").format(found.value, depth+1) # return the founded node object value
        elif not remaining:
            return None

def dls(node, depth, value):
    if depth == 0:
        if node.value == value:
            return node, True # we found our goal so don't need to traverse the rest of tree immediately return the found node object 
        else:
            return None, True
    elif depth > 0:
        any_remaining = False
        print(node.value)
        children = getChildren(node)
        for child in children:
            found, remaining = dls(child, depth-1, value) # calling the dls algo recursively for every child
            if found is not None:
                return found, True
            if remaining:
                any_remaining = True # like cuttof!
        return None, any_remaining
    else:
        return False

# ----------------
# IDDS BINARY TREE
# TREE CREATION
# ----------------

class node(object):
    # initializing our nodes
    def __init__(self, value):
        self.value = value
        self.left = None
        self.right = None


class binary_tree(object):
    # initializing the root node by creating an object of our node class
    def __init__(self, root):
        self.root = node(root)


tree = binary_tree(1) # setting the value for root

# initiating the left side of tree
tree.root.left = node(2)
tree.root.left.left = node(4)
tree.root.left.left.left = node(8)
tree.root.left.left.right = node(9)
tree.root.left.right = node(5)
tree.root.left.right.left = node(10)
tree.root.left.right.right = node(11)

# initiating the right side of tree
tree.root.right = node(3) 
tree.root.right.left = node(6) 
tree.root.right.left.left = node(12)
tree.root.right.left.right = node(13)
tree.root.right.right = node(7)
tree.root.right.right.left = node(14)
tree.root.right.right.right = node(15)



if __name__ == "__main__":
    print(iddfs(tree, 6)) # find the value 6 in tree with idds algo




# =====================================

# mgcoloring

# SOVLED USING BACKTRACKING
  
class Graph(): 
  
    def __init__(self, vertices): 
        self.V = vertices 
        self.graph = [[0 for column in range(vertices)]\
                              for row in range(vertices)]
    # A utility function to check if the current color assignment is safe for vertex v 
    def CanWeGo(self, v, colour, c): 
        for i in range(self.V): 
            if self.graph[v][i] == 1 and colour[i] == c: 
                return False
        return True
      
    # A recursive utility function to solve m coloring problem 
    def TurnTheColorOn(self, m, colour, v): 
        if v == self.V: 
            return True
  
        for c in range(1, m+1): 
            if self.CanWeGo(v, colour, c) == True: 
                colour[v] = c 
                if self.TurnTheColorOn(m, colour, v+1) == True: 
                    return True
                colour[v] = 0
  
    def DoTheJob(self, m): 
        colour = [0] * self.V 
        if self.TurnTheColorOn(m, colour, 0) == False: 
            return False
  
        # Print the solution 
        print("Solution exist and Following are the assigned colours:")
        for c in colour: 
            print(c) 
        return True
  
# Driver Code 
g  = Graph(4) 
g.graph = [[0,1,1,1], [1,0,1,0], [1,1,0,1], [1,0,1,0]] 
m=3
g.DoTheJob(m)



# =====================================

# NPAgent - n-puzzle


# https://blog.goodaudience.com/solving-8-puzzle-using-a-algorithm-7b509c331288

class Node:
    def __init__(self,data,level,fval):
        """ Initialize the node with the data, level of the node and the calculated fvalue """
        self.data = data # 2D dimensional matrix ; eg: [['1', '2', '3'], ['_', '4', '6'], ['7', '5', '8']]
        self.level = level
        self.fval = fval

    def generate_child(self):
        """ Generate child nodes from the given node by moving the blank space
            either in the four directions {up,down,left,right} """
        x,y = self.find(self.data,'_') # return the position of _ 
        # print(x,y)
        """ val_list contains position values for moving the blank space in either of
            the 4 directions [up,down,left,right] respectively. """
        val_list = [[x,y-1],[x,y+1],[x-1,y],[x+1,y]] # for _ with position 1,0 it'll be [[1, -1], [1, 1], [0, 0], [2, 0]]
        # print(val_list)
        children = []
        for i in val_list:
            child = self.shuffle(self.data,x,y,i[0],i[1]) # return new puzzle state
            if child is not None:
                child_node = Node(child,self.level+1,0) # generating new node as children for our new state
                children.append(child_node)
        return children # return the list of all states that can be happen by shuffling the _ directions
        
    def shuffle(self,puz,x1,y1,x2,y2):
        """ Move the blank space in the given direction and if the position value are out
            of limits then return None """
        if x2 >= 0 and x2 < len(self.data) and y2 >= 0 and y2 < len(self.data):
            temp_puz = []
            temp_puz = self.copy(puz)
            temp = temp_puz[x2][y2]
            temp_puz[x2][y2] = temp_puz[x1][y1]
            temp_puz[x1][y1] = temp
            return temp_puz
        else:
            return None

    def copy(self,root):
        """ Copy function to create a similar matrix of the given node"""
        temp = []
        for i in root:
            t = []
            for j in i:
                t.append(j)
            temp.append(t)
        return temp    
            
    def find(self,puz,x):
        """ Specifically used to find the position of the blank space """
        for i in range(0,len(self.data)):
            for j in range(0,len(self.data)):
                if puz[i][j] == x:
                    return i,j


class Puzzle:
    def __init__(self,size):
        """ 
        Initialize the puzzle size by the specified size,open and closed lists to empty 
        """
        self.n = size
        self.open = []
        self.closed = []

    def accept(self):
        """ Accepts the puzzle from the user """
        puz = []
        for i in range(0,self.n): # for size = 3 it'll be a 3 lists in puz list
            temp = input().split(" ")
            puz.append(temp)
        return puz
        
    def f(self,start,goal):
        """ Heuristic Function to calculate hueristic value f(x) = h(x) + g(x) """
        print("\n------------------------------------------------------\n")
        print("f = h -> {} + g -> {}".format(self.h(start.data,goal), start.level))
        print("------------------------------------------------------")
        return self.h(start.data,goal)+start.level # start.level is g ; number of total actions that we had so far in each depth

    def h(self,start,goal):
        """ Calculates the different between the given puzzles """
        temp = 0
        for i in range(0,self.n):
            for j in range(0,self.n):
                if start[i][j] != goal[i][j] and start[i][j] != '_':
                    temp += 1 # increase the number of misplaced tiles by comparing the current start[i][j] with the goal[i][j]
        return temp
    
    def process(self):
        """ Accept Start and Goal Puzzle state"""
        print("[+] Enter the start state matrix \n")
        start = self.accept()
        # print(start)
        print("[+] Enter the goal state matrix \n")        
        goal = self.accept()
        start = Node(start,0,0)
        # start.fval = self.f(start,goal)
        """ Put the start node in the open list"""
        self.open.append(start) # start node is being generated
        print("\n")
        while True:
            cur = self.open[0] # we won't store our entire nodes our open list due to lack of memory space and optimality issue 
            for i in cur.data:
                for j in i:
                    print(j,end=" ")
                print("")
            print("")
            print("  | ")
            print("  | ")
            print(" \\\'/ \n")
            """ If the difference between current and goal node is 0 we have reached the goal node"""
            if(self.h(cur.data,goal) == 0):
                self.f(cur, goal)
                break
            for i in cur.generate_child():
                i.fval = self.f(i,goal)
                self.open.append(i)
            self.closed.append(cur) # we checked this state(node)
            del self.open[0] # deleting the seen state from frontier list ; this list always contain one elem
            """ sort the opne list based on f value """
            # so next time in our while loop the node with the least fval will pick up and explore
            self.open.sort(key = lambda x:x.fval,reverse=False)
            print("**********************************")
            print("[+] THE LEAST f IS PICKED UP!!!!\nSEE BELOW NEW STATE ")
            print("**********************************\n")



if __name__ == "__main__":
    puz = Puzzle(3) # 3x3 ; also it can be any dimension
    puz.process()



# =====================================

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


#----------------------------------------------------------------------------------------------------
# HOST UR CODE ON PYTHONANYWHERE : https://www.pythonanywhere.com/user/wildonion/files/home/wildonion
#----------------------------------------------------------------------------------------------------

# also use flask for its api
# u can build this app for android using nodejs react-native or google flutter or haxe
# module : python-telegram-bot
# python telegram bot to choose the best watermelon from its sound for the summer ;-)
# this script will upload on pythonanywhere cloud and for api setup for telegram bot webhooks it'll use flask => https://blog.pythonanywhere.com/148/ 
# i'll take the melon sound using telegram voice feature then here with an simple AI algorithm detect wheter it's a ripe melon or an unripe one 
# perhaps u want to use AI for this project to collect dataset using ANN 
# python api with django/flask on pythonanywhere for telegram bot webhooks and other apps 

'''
SOURCES:
https://github.com/bangnguyenanh/notopi
https://medium.freecodecamp.org/create-a-news-app-using-react-native-ced249263627
https://medium.com/@austinhale/building-a-mobile-app-in-10-days-with-react-native-c2a7a524c6b4
https://medium.com/surabayadev/setting-up-react-native-android-without-android-studio-35a496e1dfa3
'''

# code
# ....



# -------------------------------------------------------------------------------------------
# harmonic calculator
from math import log
from timeit import *

def hur(n):
    if n is 1:
        return 1
    else:
        return hur(n-1) + 1/n
print(hur(3))

def H(n):
    gamma = 0.57721566490153286060651209008240243104215933593992
    print(gamma + log(n) + 0.5/n - 1./(12*n**2) + 1./(120*n**4))

H(20)

# =======================
# prime detection
def detectP(n):
    primes = []
    for possiblePrime in range(2, n+1):
        isPrime = True
        for x in range(2, int(possiblePrime ** 0.5) + 1):
            if possiblePrime % x == 0:
                isPrime = False
                break
        if isPrime:
            primes.append(possiblePrime)
    return primes

print(detectP(10))
print(timeit('detectP(34)', globals=globals(), number=10000))


# -------------------------------------------------------------------------------------------
# designing pattern on a simple OOP problem
# employees insured and salary

employees = []
#   ======================================

#                 CLASSES

import os
class Employee():
  
  def __init__(self, pcode):
    self.name = ""
    self.lname = ""
    self.meli_code = 0
    self.pcode = pcode
    self.fixed_sal = 0.0
    self.hours_of_work = 0
    self.tax = None
    self.insurance = 0.0
    self.overtime= 0.0
    self.final_sal = 0.0
  
  
  def status(self):
    if self.hours_of_work > 40:
      hours_of_overtime = self.hours_of_work - 40
      self.overtime = hours_of_overtime * 30000
    self.insurance = self.fixed_sal * 0.05
    
    if self.fixed_sal < 4000000:
      self.tax = "moaf"
    if 4000000 < self.fixed_sal < 5000000:
      self.tax = (self.fixed_sal - 4000000) * 0.10
    if 5000000 < self.fixed_sal < 7000000:
      self.tax = (self.fixed_sal - 4000000) * 0.15
    if self.fixed_sal > 7000000:
      self.tax = (self.fixed_sal - 4000000) * 0.20
      
    self.final_sal = (self.overtime + self.fixed_sal) - (self.insurance + self.tax)
  
  def addinfo(self, name, lname, meli_code, fixed_sal, how):
    self.name = name
    self.lname = lname
    self.meli_code = meli_code
    self.fixed_sal = fixed_sal
    self.hours_of_work = how
  
  def showinfo(self):
    print("\n==============\nEMPLOYEE INFO\n==============\n")
    print("[+] NAME          : ", self.name)
    print("[+] LAST NAME     : ", self.lname)
    print("[+] MELI CODE     : ", self.meli_code)
    print("[+] PERSONAL CODE : ", self.pcode)
    print("[+] FIXED SALARY  : ", self.fixed_sal)
    print("[+] HOURS OF WORK : ", self.hours_of_work)
    print("[+] TAX           : ", self.tax)
    print("[+] INSURANCE     : ", self.insurance)
    print("[+] OVERTIME      : ", self.overtime)
    print("[+] FINAL SALARY  : ", self.final_sal)
  
  
#   ==================================

#           FUNCTIONS   

def showallempinfo():
  for i in range(len(employees)):
    employees[i].showinfo()
    print("\t********")

    
def findbypcode():
#   os.system("cls")
  while True:
    pcode = input("[+] Enter personal code to find an employee >> ")
    if not pcode:
      break
    for i in range(len(employees)):
      if employees[i].pcode == int(pcode):
        print("[+] FOUND ONE MATCH ... ")
        employees[i].showinfo()
        break
      else:
        print("[+] NOTHING FOUND... ")
        break
      
def savetofile(emps, filename):
  with open(filename+".txt", "w") as f:
    for i in range(len(emps)):
      f.write("\n==============\nEMPLOYEE INFO\n==============\n")
      f.write("[+] NAME          : ")
      f.write(str(employees[i].name)+"\n")
      f.write("[+] LAST NAME     : ")
      f.write(str(employees[i].lname)+"\n")
      f.write("[+] MELI CODE     : ")
      f.write(str(employees[i].meli_code)+"\n")
      f.write("[+] PERSONAL CODE : ")
      f.write(str(employees[i].pcode)+"\n")
      f.write("[+] FIXED SALARY  : ")
      f.write(str(employees[i].fixed_sal)+"\n")
      f.write("[+] HOURS OF WORK : ")
      f.write(str(employees[i].hours_of_work)+"\n")
      f.write("[+] TAX           : ")
      f.write(str(employees[i].tax)+"\n")
      f.write("[+] INSURANCE     : ")
      f.write(str(employees[i].insurance)+"\n")
      f.write("[+] OVERTIME      : ")
      f.write(str(employees[i].overtime)+"\n")
      f.write("[+] FINAL SALARY  : ")
      f.write(str(employees[i].final_sal)+"\n")
      
def readfromfile(path):
  with open(path+".txt", "r") as f:
    for line in f:
      print(line)
    
  
  
def save3emptofile():
  data = []
  for i in range(3):
    code = input(f"\tEnter meli code or personal code of person {i+1}, ba comma joda konid, like : 00198222, 33 >> ")
    if "," in code:
      meli_code = code.split(",")[0]
      pcode = code.split(",")[1]
      for j in range(len(employees)):
        if employees[j].meli_code == meli_code and employees[j].pcode == pcode:
          data.append(employees[j])
    else:
      for j in range(len(employees)):
        if employees[j].meli_code == code or employees[j].pcode == code:
          data.append(employees[j])
  
  savetofile(data, "data")
      
      

#   ==============================

#             MAIN


if __name__ == "__main__":
  
#   initalizing....
  for i in range(int(input("[+] Employee.N >> "))):
    name = str(input("\tEnter name >> "))
    lname = str(input("\tEnter last name >> "))
    meli_code = int(input("\tEnter meli code >> "))
    fixed_sal = float(input("\tEnter fixed salary >> "))
    how = int(input("\tEnter hours of work >> "))
    e = Employee(i+1)
    e.addinfo(name, lname, meli_code, fixed_sal, how)
    e.status()
    employees.append(e)
    if i > 1 :
      print("\t@@@@@@@@@@@@@@@@@@@")
    
  
  showallempinfo()
  print("[[[[[[ FINDING EMPLOYEE ]]]]]]")
  findbypcode()
  print("[[[[[[ SAVING TO  FILE .... ]]]]]]")
  savetofile(employees, "all_employees")
  print("[[[[[[ READING FROM FILE ]]]]]]")
  readfromfile("all_employees")
  print("[[[[[[ SAVING 3 EMPLOYEE TO FILE ]]]]]]")
  save3emptofile()

    
# -------------------------------------------------------------------------------------------
# cycle detection algo in directed graph

def cycle_exists(G):                     # - G is a directed graph
    color = { u : "white" for u in G  }  # - All nodes are initially white
    found_cycle = [False]                # - Define found_cycle as a list so we can change
                                         # its value per reference, see:
                                         # http://stackoverflow.com/questions/11222440/python-variable-reference-assignment
    for u in G:                          # - Visit all nodes.
        if color[u] == "white":
            dfs_visit(G, u, color, found_cycle)
        if found_cycle[0]:
            break
    return found_cycle[0]


def dfs_visit(G, u, color, found_cycle):
    if found_cycle[0]:                          # - Stop dfs if cycle is found.
        return
    color[u] = "gray"                           # - Gray nodes are in the current path
    for v in G[u]:                              # - Check neighbors, where G[u] is the adjacency list of u.
        if color[v] == "gray":                  # - Case where a loop in the current path is present.  
            found_cycle[0] = True       
            return
        if color[v] == "white":                 # - Call dfs_visit recursively.   
            dfs_visit(G, v, color, found_cycle)
    color[u] = "black" 

graph_example_1 = { 0 : [1],
                    1 : [2],
                    2 : [3],
                    3 : [4],
                    4 : [3] }


print("Cycle?", cycle_exists(graph_example_1))  

# -------------------------------------------------------------------------------------------

'''

https://codeforces.com/problemset/problem/1373/G

time limit per test   : 3 seconds
memory limit per test : 256 megabytes
input                 : standard input
output                : standard output

'''

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

	
# -------------------------------------------------------------------------------------------

# DIRECTED GRAPH MATRIX

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





