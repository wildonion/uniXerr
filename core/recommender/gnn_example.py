



'''
note that the adj_mat can be built using one the existing feature engineering and selection techniques like genetic algorithm
'''



import numpy as np

class node:
    # node is the feature vector which has the shape of (batch_size, 1).
    # each node will know about its feature through the learning process.
    def __init__(self, data):
        self.data = data
        self.weight = np.random.rand(self.data.T.shape)


class gnn:
    def __init__(self, input, adj_mat):
        self.input = input
        self.adj_mat = adj_mat
    
    def feedforward(self):
        # we can build the graph using self.adj_mat in which 
        # the connection between each feature or node has specified by 1 or 0.
        # there is a connection between i-th and j-th feature node 
        # if and only if there is an edge between them,  
        # which means the next node in the graph.
        connections = {row: col for row in range(len(self.adj_mat)) for col in range(len(self.adj_mat)) if self.adj_mat[row][col] == 1} # key : firs node , value : second nodes
        nodes = [node(self.input[:,f]) for f in range(len(self.input.shape[-1]))] # all node fearures from our sample
        embedded = {node: node.data * node.w for node in nodes} # node features and their embedded value with size self.input.shape[0] by self.input.shape[0]


    def train(self):
    	pass

if __name__ == "__main__":
     # build the gnn and represent features using adjacency matrix
    adj_mat, samples = [], [] # the size of the adj_mat is sample[1]=features by sample[1]=features
    model = gnn(samples, adj_mat)
    model.train()
