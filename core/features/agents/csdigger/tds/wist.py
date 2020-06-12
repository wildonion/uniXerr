

# WIST algo


import random

# creating Node class
class Node:
    def __init__(self, data):
        self.childeren = [] # childeren of a node
        self.data = data # weight of a node
        self.Min = self.data # Min of a node
        self.Mout = 0 # Mout of a node
    

# check if a node is leaf or not
def checkleaf(node):
    return node is not None and len(node.childeren) == 0

# the weight independent set tree algorithm
def wist(node):
    for i in range(len(node.childeren)):
        wist(node.childeren[i])
    if checkleaf(node): # if the node was leaf
        node.Min = node.data # Min is equal to the node weight
        node.Mout = 0 # Mout is 0
    else: # if the node was not leaf
        node.Min = node.data # Min is equal to the node weight
        node.Mout = 0 # Mout is 0
        for i in node.childeren: # 
            node.Min += i.Mout # update the node.Min with its childeren Mout
            node.Mout =  max(i.Mout, i.Min) # update the node.Mout



def solve(node_data):
    root = Node(node_data) # create the root
    wist(root) # calling the wist method on the root


    for i in range(node_data):
        root.childeren.append(Node(i+1)) # filling our root childeren with its append method

    for i in range(2):
        for n in root.childeren:
            n.childeren.append(Node(random.randint(n.data, i+node_data%4))) # filling our each childeren of our root node with its append method


    # print the calculated info for each childeren of root
    for n in root.childeren:
        print("[+] DATA: {} , MIN {} , MOUT {}".format(n.data, n.Min, n.Mout))
        for v in n.childeren:
            print("\t[+] DATA: {} , MIN {} , MOUT {}".format(v.data, v.Min, v.Mout))