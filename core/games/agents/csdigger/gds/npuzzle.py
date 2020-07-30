
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
