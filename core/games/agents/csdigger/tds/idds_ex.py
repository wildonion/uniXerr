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