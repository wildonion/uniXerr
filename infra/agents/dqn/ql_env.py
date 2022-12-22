



import numpy as np

class Env:
    def __init__(self):
        self.height = 5
        self.width = 5
        self.posX = 0
        self.posY = 0
        self.endX = self.width-1
        self.endY = self.height-1
        self.actions = [0, 1, 2, 3]
        self.stateCount = self.height*self.width
        self.actionCount = len(self.actions)

    def reset(self):
        self.posX = 0
        self.posY = 0
        self.done = False
        return 0, 0, False

    # take action
    def step(self, action):
        if action == 0: # left
            self.posX = self.posX-1 if self.posX > 0 else self.posX
        if action == 1: # right
            self.posX = self.posX+1 if self.posX < self.width - 1 else self.posX
        if action == 2: # up
            self.posY = self.posY-1 if self.posY > 0 else self.posY
        if action == 3: # down
            self.posY = self.posY+1 if self.posY < self.height - 1 else self.posY

        done = self.posX == self.endX and self.posY == self.endY
        # mapping (x,y) position to number between 0 and 5x5-1=24
        nextState = self.width * self.posY + self.posX
        reward = 1 if done else 0
        return nextState, reward, done

    # return a random action
    def randomAction(self):
        return np.random.choice(self.actions)

    # display environment
    def render(self):
        for i in range(self.height):
            for j in range(self.width):
                if self.posY == i and self.posX == j:
                    print("O", end='')
                elif self.endY == i and self.endX == j:
                    print("T", end='')
                else:
                    print(".", end='')
            print("")