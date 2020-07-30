
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