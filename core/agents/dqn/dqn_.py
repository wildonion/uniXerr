import random
import gym
import numpy as np
from collections import deque
from keras.models import Sequential
from keras.layers import Dense, Activation, Flatten
from keras.optimizers import Adam
import matplotlib.pyplot as plt
from keras import backend as K
import tensorflow as tf
from rl.agents.dqn import DQNAgent
from rl.policy import BoltzmannQPolicy
from rl.memory import SequentialMemory

EPISODES = 1000 # a number of games we want the agent to play.

class DQNAgent:
    def __init__(self, state_size, action_size):
        self.state_size = state_size
        self.action_size = action_size
        # print(self.action_size, self.state_size)
        self.memory = deque(maxlen=2000)
        self.gamma = 0.95    # discount rate - aka decay or discount rate, to calculate the future discounted reward.
        self.epsilon = 1.0  # aka exploration rate, this is the rate in which an agent randomly decides its action rather than prediction.
        self.epsilon_min = 0.01 # we want the agent to explore at least this amount.
        self.epsilon_decay = 0.995 # we want to decrease the number of explorations as it gets good at playing games.
        self.learning_rate = 0.001 # Determines how much neural net learns in each iteration.
        self.model = self._build_model()
        self.target_model = self._build_model()
        self.update_target_model()


    def _huber_loss(self, y_true, y_pred, clip_delta=1.0):
        error = y_true - y_pred
        cond  = K.abs(error) <= clip_delta

        squared_loss = 0.5 * K.square(error)
        quadratic_loss = 0.5 * K.square(clip_delta) + clip_delta * (K.abs(error) - clip_delta)

        return K.mean(tf.where(cond, squared_loss, quadratic_loss))

    def _build_model(self):
        # Neural Net for Deep-Q learning Model
        model = Sequential()
        # 'Dense' is the basic form of a neural network layer
        # Input Layer of state size(4) and Hidden Layer with 24 nodes
        model.add(Dense(24, input_dim=self.state_size, activation='relu'))
        # Hidden layer with 24 nodes
        model.add(Dense(24, activation='relu'))
        # Output Layer with # of actions: 2 nodes (left, right)
        model.add(Dense(self.action_size, activation='linear'))
        model.compile(loss=self._huber_loss,
                      optimizer=Adam(lr=self.learning_rate))
        return model
    
    def update_target_model(self):
        """copy weights from model to target_model ; 
        cause the weights will update in every epoch we won't get our target 
        and cause of that we're using another brain here"""
        self.target_model.set_weights(self.model.get_weights()) # only replace the weights of our new model with the existing model

    def remember(self, state, action, reward, next_state, done):
        """ One of the challenges for DQN is that neural network used in the algorithm tends to forget the previous experiences 
            as it overwrites them with new experiences. So we need a list of previous experiences and observations to re-train 
            the model with the previous experiences. We will call this array of experiences memory and use remember() function to append state, action, reward, and next state to the memory.
        """
        
        self.memory.append((state, action, reward, next_state, done))

    def act(self, state):
        """ Our agent will randomly select its action at first by a certain percentage, called ‘exploration rate’ or ‘epsilon’. 
            This is because at first, it is better for the agent to try all kinds of things before it starts to see the patterns. 
            When it is not deciding the action randomly, the agent will predict the reward value based on the current state and 
            pick the action that will give the highest reward. np.argmax() is the function that picks the highest value between two elements in the act_values[0].
        """
        
        if np.random.rand() <= self.epsilon:
            return random.randrange(self.action_size) # choose a random number between possible actions(left or right) to reach a state
        act_values = self.model.predict(state) 
        # print(act_values)
        # act_values =:
        # [[0.2342021  0.33798292]]
        # [[0.20367415 0.3270337 ]]
        # .....
        # [[0.28844938 0.33308575]]
        # returns the maximum action between existing actions(left and right) which calculated by the NN
        # (eg, 0.67 go to right and 0.33 go to left then it'll choose the right direction for our state vector)
        return np.argmax(act_values[0])

    def replay(self, batch_size):
        """ A method that trains the neural net with experiences in the memory is called replay(). First, we sample some experiences from the memory and call them minibath.
            minibatch = random.sample(self.memory, batch_size)
            The above code will make minibatch, which is just a randomly sampled elements of the memories of size batch_size. We set the batch size as 32 for this example.
            To make the agent perform well in long-term, we need to take into account not only the immediate rewards but also the future rewards we are going to get. 
            In order to do this, we are going to have a ‘discount rate’ or ‘gamma’. This way the agent will learn to maximize the discounted future reward based on the given state 
            However, the problem is that we using the same parameters (weights) for estimating the target and the Q value. As a consequence, there is a big correlation between the TD target and the parameters (w) we are changing.
            Therefore, it means that at every step of training, our Q values shift but also the target value shifts. So, we’re getting closer to our target but the target is also moving. It’s like chasing a moving target! This lead to a big oscillation in training.
            By calculating the TD target, we face a simple problem: how are we sure that the best action for the next state is the action with the highest Q-value?
            We know that the accuracy of q values depends on what action we tried and what neighboring states we explored.
            As a consequence, at the beginning of the training we don’t have enough information about the best action to take. Therefore, taking the maximum q value (which is noisy) as the best action to take can lead to false positives. If non-optimal actions are regularly given a higher Q value than the optimal best action, the learning will be complicated.
            The solution is: when we compute the Q target, we use two networks to decouple the action selection from the target Q value generation. We:
            use our DQN network to select what is the best action to take for the next state (the action with the highest Q value).
            use our target network to calculate the target Q value of taking that action at the next state.
        """
        
        # print(self.memory)
        minibatch = random.sample(self.memory, batch_size)
        # minibactch =: deque([(array([[-0.07520104, -0.37955881,  0.06440256,  0.54503593]]), 1, 1.0, array([[-0.08279222, -0.18539812,  0.07530328,  0.27331995]]), False), 
        #                      (array([[ 0.00108801,  0.00934971, -0.04501215, -0.01184072]]), 0, 1.0, array([[ 0.00127501, -0.18509877, -0.04524897,  0.26630747]]), False), 
        #                      (array([[-0.08632872,  0.20244938,  0.0808758 , -0.26083994]]), 1, 1.0, array([[-0.08227973,  0.39632928,  0.075659  , -0.52695763]]), False),
        #                      (array([[-0.00380544,  1.36878757, -0.03151045, -1.91940555]]), 1, 1.0, array([[ 0.02357031,  1.56423369, -0.06989856, -2.22169174]]), False),  
        #                       ...., 
        #                      (array([[ 0.08225186,  1.56598078, -0.15335945, -2.27717757]]), 1, 1.0, array([[ 0.11357148,  1.76216002, -0.198903  , -2.61290389]]), False) ])
        # states, targets_f = [], []
        for state, action, reward, next_state, done in minibatch:
            # eg: reward =: 1.0 , state =: [[-0.02663719 -0.2388644  -0.01880481  0.2460182 ]], maximum integer action calculated from NN =: 1
            # target = reward # Q(s,a) = R
            target = self.model.predict(state)
            if done:
                # The loss is just a value that indicates how far our prediction is from the actual target. For example, 
                # the prediction of the model could indicate that it sees more value in pushing the right button when in fact 
                # it can gain more reward by pushing the left button. We want to decrease this gap between the prediction and the target (loss). We will define our loss function as follows:
                # We first carry out an action a, and observe the reward r and resulting new state s`. Based on the result, 
                # we calculate the maximum target Q and then discount it so that the future reward is worth less than immediate reward 
                # (It is a same concept as interest rate for money. Immediate payment always worth more for same amount of money). 
                # Lastly, we add the current reward to the discounted future reward to get the target value. Subtracting our current 
                # prediction from the target gives the loss. Squaring this value allows us to punish the large loss value more and treat the negative values same as the positive values.
                # Keras takes care of the most of the difficult tasks for us. We just need to define our target. We can express the target in a magical one-liner in python.
                # self.model.predict(next_state)[0] => calculated action from NN 
                # target = (reward + self.gamma *
                #           np.amax(self.model.predict(next_state)[0])) # returns the maximum action between all existing actions which calculated by the NN
                target[0][action] = reward
            else:
                t = self.target_model.predict(next_state)[0] # predict the value for Q(S’,A’) in which the output are left or right and we're gonna get the maximum action between predicted actions 
                # we can achieve optimal policies for our games by estimating the Q(s, a) function, 
                # which gives us an estimate of the discounted sum of rewards of taking action a in state s, 
                # and playing optimally thereafter. Playing the action with the maximum Q-value in any given state is the same as playing optimally!
                # The question is now: how do we estimate Q(s, a)? Well… how do you estimate any function these days? 
                # With a deep neural network of course! Or as you might call it, a Deep Q-Network (DQN).
                target[0][action] = reward + self.gamma * np.amax(t) # Q function algo
            # we are losing our weights in every epoch in training samples so cuase of this 
            # we are not able to reach the target value cause the target is also moving and
            # (we calculate the traget value with our first brain also) the solution is use another brain to copy 
            # our first brain synapses(weights) on target brains and calculate its value
            # and then we can train our samples in our first brain in 20 epochs so by this job 
            # we don't have to be worry about losing our weights and get far away from our target
            # cause we calculated our target value with another brain and the epoch is happening in another brain!
            """ML problems start with data—preferably, lots of data (examples or observations) 
            for which you already know the target answer. Data for which you already know 
            the target answer is called labeled data. ... You provide data that is labeled 
            with the target (correct answer) to the ML algorithm to learn from."""
            self.model.fit(state, target, epochs=20, verbose=0) # train the model on 20 epochs to close itself to the target
            # print(target)
            # After training, the model now can predict the output from unseen input. When you call predict() function on the model, 
            # the model will predict the reward of current state by giving the action based on the data you trained
            # target_f = self.model.predict(state) # prediction
            # # target_f =: 
            # # [[ 0.21216568 -0.00235922]]
            # # [[0.34335896 0.03951475]]
            # # [[0.04028061 0.02595955]]
            # # [[0.07161053 0.00580003]]
            # # .
            # # .
            # # .
            # # [[ 0.13222136 -0.03803551]]
            # # [[ 0.13657948 -0.04835601]]
            # # [[ 0.13386746 -0.04498314]]
            # target_f[0][action] = target 
            # # Filtering out states and targets for training
            # states.append(state[0])
            # targets_f.append(target_f[0])
            # print(target_f[0]) # eg: [ 1.162856   -0.01493281]
            # print(state[0]) # eg: [ 0.06730458 -0.00458529  0.02341822  0.1553169 ]
            # fit() method feeds input and output pairs to the model. Then the model will train on those data to approximate the output based on the input.
            # This training process makes the neural net to predict the reward value from a certain state.
            # epoch is a measure of the number of times all of the training vectors are used 
            # once to update the weights. For batch training all of the training samples pass 
            # through the learning algorithm simultaneously in one epoch before weights are updated.
        # model.fit(x=None, y=None, ...) => Trains the model for a given number of epochs (iterations on a dataset).
        # x: Numpy array of training data 
        # y: Numpy array of target (label) data
        # print(states, " ====== ", targets_f)
        # =======EPOCH=======
        # An epoch is a single step in training a neural network; 
        # in other words when a neural network is trained on every 
        # training samples only in one pass we say that one epoch 
        # is finished. So training process may consist more than one epochs.
        # history = self.model.fit(np.array(states), np.array(targets_f), epochs=5, verbose=0)
        # Keeping track of loss
        # loss = history.history['loss'][0]
        # print(loss)
        if self.epsilon > self.epsilon_min:
            self.epsilon *= self.epsilon_decay
        # return loss

    def load(self, name):
        self.model.load_weights(name)

    def save(self, name):
        self.model.save_weights(name)


if __name__ == "__main__":
    env = gym.make('CartPole-v1')
    state_size = env.observation_space.shape[0]
    # print(state_size) # 4 =: left , right , balance , slide or drop the pole
    action_size = env.action_space.n
    # print(action_size) # 2
    agent = DQNAgent(state_size, action_size)
    smodel = agent._build_model()
    smodel.summary()
    # agent.load("cartpole-dqn.h5")
    done = False
    batch_size = 32

    for e in range(EPISODES):
        # reset state in the beginning of each game
        state = env.reset()
        state = np.reshape(state, [1, state_size]) # turn the state into a one dimensional matrix which is a vector
        # time represents each frame of the game
        # Our goal is to keep the pole upright as long as possible until score of 500
        # the more time the more score
        for time in range(700):
            env.render()
            # Decide action
            action = agent.act(state) # maximum action ; pass our vector state to our NN in which we have state_size neurons
            # Advance the game to the next frame based on the action.
            # Reward is 1 for every frame the pole survived
            next_state, reward, done, _ = env.step(action)
            reward = reward if not done else -10
            # we are turning our next_state into a one dimensional matrix which is a vector
            # to calculate the maximum future reward for next state ; cause our model input 
            # is a one dimensional matrix which is a vector in which in our case is 4 neurons
            next_state = np.reshape(next_state, [1, state_size])
            # Remember the previous state, action, reward, and done
            agent.remember(state, action, reward, next_state, done)
            # make next_state the new current state for the next frame.
            state = next_state
            # done becomes True when the game ends
            # ex) The agent drops the pole
            if done:
                agent.update_target_model()
                print("episode: {}/{}, score: {}, e: {:.2}"
                      .format(e, EPISODES, time, agent.epsilon))
                break
            if len(agent.memory) > batch_size:
                # train the agent with the experience of the episode
                # loss = agent.replay(batch_size)
                agent.replay(batch_size)
                # Logging training loss every 10 timesteps
                # if time % 10 == 0:
                #     print("episode: {}/{}, time: {}, loss: {:.4f}"
                #         .format(e, EPISODES, time, loss))  
#         if e % 10 == 0:
#             agent.save("cartpole-dqn.h5")

# # --------------------------------------------------------------------------------------------------------------------------------------------------------

# Get the environment and extract the number of actions.
env = gym.make(ENV_NAME)
np.random.seed(123)
env.seed(123)
nb_actions = env.action_space.n

# Next, we build a very simple model regardless of the dueling architecture
# if you enable dueling network in DQN , DQN will build a dueling network base on your model automatically
# Also, you can build a dueling network by yourself and turn off the dueling network in DQN.
model = Sequential()
model.add(Flatten(input_shape=(1,) + env.observation_space.shape))
model.add(Dense(16))
model.add(Activation('relu'))
model.add(Dense(16))
model.add(Activation('relu'))
model.add(Dense(16))
model.add(Activation('relu'))
model.add(Dense(nb_actions, activation='linear'))
print(model.summary())

# Finally, we configure and compile our agent. You can use every built-in Keras optimizer and
# even the metrics!
memory = SequentialMemory(limit=50000, window_length=1)
policy = BoltzmannQPolicy()
# enable the dueling network
# you can specify the dueling_type to one of {'avg','max','naive'}
dqn = DQNAgent(model=model, nb_actions=nb_actions, memory=memory, nb_steps_warmup=10,
               enable_dueling_network=True, dueling_type='avg', target_model_update=1e-2, policy=policy)
dqn.compile(Adam(lr=1e-3), metrics=['mae'])

# Okay, now it's time to learn something! We visualize the training here for show, but this
# slows down training quite a lot. You can always safely abort the training prematurely using
# Ctrl + C.
dqn.fit(env, nb_steps=50000, visualize=False, verbose=2)

# After training is done, we save the final weights.
dqn.save_weights('duel_dqn_{}_weights.h5f'.format(ENV_NAME), overwrite=True)

# Finally, evaluate our algorithm for 5 episodes.
dqn.test(env, nb_episodes=5, visualize=False)