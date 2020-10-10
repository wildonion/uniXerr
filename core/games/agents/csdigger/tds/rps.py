

# ROCK | SCISSORS | PAPER

class Node:
    def __init__(self, data):
        self.data     = data
        self._to      = []
        self._from    = []
        self.right    = None
        self.left     = None

class Game:
    def __init__(self):
        self.tree     = None
        self.status   = None
    def tree(self, root): # show in preorder
        if root:  
            print(root.data) 
            self.show_tree(root.left) 
            self.show_tree(root.right)


class Player:
    def __init__(self, name, command="", score=0):
        self.name    = name
        self.command = command
        self.score   = score
        
class RPS(Game):
    def __init__(self, p1, p2):
        super().__init__()
        root             = Node("S")
        root.left        = Node("P")
        root._to.append(root.left)

        root.right       = Node("R")
        root._from.append(root.right)

        root.left.left   = Node("R")
        root.left._to.append(root.left.left)

        root.left.right  = Node("S")
        root.left._from.append(root.left.right)

        root.right.left  = Node("S")
        root.right._to.append(root.right.left)

        root.right.right = Node("P")
        root.right._from.append(root.right.right)

        self.tree        = root
        self.p1_score    = p1.score
        self.p2_score    = p2.score
        self.p1_name     = p1.name
        self.p2_name     = p2.name
        self.p1_cmd      = p1.command if p1.command != "" else ""
        self.p2_cmd      = p2.command if p2.command != "" else ""

    def __check_status(self):
        return "e" if self.p1_cmd == self.p2_cmd else "c"

    def score(self):
        self.status = self.__check_status()
        self.__compare(self.tree)

    def __compare(self, root):
        if root: 
            self.__compare(root.left)
            if len(root._to) > 0:
                for n in root._to:
                    if root.data == self.p1_cmd:
                        if self.p2_cmd == n.data:
                            print(f"one score more for player [{self.p1_name}]")
                            self.p2_score+=1
                    if root.data == self.p2_cmd:
                        if self.p1_cmd == n.data:
                            print(f"one score more for player [{self.p2_name}]")
                            self.p1_score+=1

            if len(root._from) > 0:
                for _n in root._from:
                    if root.data == self.p1_cmd:
                        if self.p2_cmd == _n.data:
                            print(f"one score less for player [{self.p1_name}]")
                            if self.p2_score > 0: 
                                self.p2_score-=1
                    if root.data == self.p2_cmd:
                        if self.p1_cmd == _n.data:
                            print(f"one score less for player [{self.p2_name}]")
                            if self.p1_score > 0: 
                                self.p1_score-=1
            self.__compare(root.right)


if __name__ == "__main__":
    

    p1_name = input("Enter Name For Player 1 >>> ")
    p2_name = input("Enter Name For Player 2 >>> ")
    p1 = Player(name=p1_name)
    p2 = Player(name=p2_name)

    while True:
        print("======================")
        cmd_1 = input("Player 1 Command >>>> ")
        cmd_2 = input("Player 2 Command >>>> ")

        if cmd_1 != "" and cmd_2 != "":
            p1.command = cmd_1
            p2.command = cmd_2
            g = RPS(p1, p2)
            g.score()
            if g.status == "e":
                print("equal commands")

        else:
            print("Enter A Correct Command Please!")