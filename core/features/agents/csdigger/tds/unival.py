



class node:
	def __init__(self, value):
		self.left = None
		self.right = None
		self.value = value
		self.id = None


root = node(5)
TMP = root.value
root.left = node(5)
root.right = node(1)

root.right.right = node(5)
root.left.left = node(1)
root.left.right = node(5)

root.id = 1
root.left.id = 2
root.right.id = 3
root.right.right.id = 6
root.left.left.id = 4
root.left.right.id = 5


COUNT = 0
is_unival_flag = None

print("|======== DFS Algorithm ========|\n")
def uvt(node):
	global TMP
	global is_unival_flag
	if node:
		print(f"\tnode id {node.id} with value {node.value}\n")
		if node.left is None:
			print(f"unival subtree found this node with id {node.id} has no left children!")
		if node.right is None:
			print(f"unival subtree found this node with id {node.id} has no right children!")
		is_unival_flag = is_unival(node.value, TMP) # compare this node value with its parrent value
		TMP = node.value # change the TMP with the value of this node for later comparison
		uvt(node.left) # go to the left till reach the leaf  
		uvt(node.right) # go to the right till reach the leaf

def is_unival(value, TMP):
	global COUNT
	if TMP is not value:
		return False
	else:
		COUNT += 1
		return True



uvt(root)

if not is_unival_flag: 
	print(f"\n\t [this tree is not a universal tree but has {COUNT} unival subtree]")
else:
	print(f"\n\t [this tree is a universal tree and has {COUNT} unival subtree]")
