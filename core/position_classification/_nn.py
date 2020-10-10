

# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██



 --------------------------------------------
|        Position Classification model
| -------------------------------------------
| neural network classifer
| for position classification
|
|
| probability of position A : p_a
| probability of position B : p_b
| probability of position C : p_c
| probability of position D : p_d
| probability of position E : p_e
| probability of position F : p_f
|

'''

import numpy as np
import os
import sys
import torch
from torch import nn
from torch.autograd import Variable


class Position(nn.Module):
	def __init__(self, input_neurons, output_neurons):
		super(Position, self).__init__()
		self.i_l = nn.Linear(input_neurons, 16) # i_l.weight  : torch.Size([16, 4]), is transposed because of back-propagation algorithm
		self.o_l = nn.Linear(16, output_neurons) # o_l.weight : torch.Size([6, 16]), is transposed because of back-propagation algorithm
		self.relu = nn.ReLU()
		self.softmax = nn.Softmax(dim=1) # dim=1, sum of each rows will be 1 : p_a + p_b + p_c + p_d + p_e + p_f

	def forward(self, x):
		output = self.relu(self.i_l(x))
		output = self.softmax(self.o_l(output))
		return output