

# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██




 --------------------------------------------
|          	     trainer class
| -------------------------------------------
| classify controller for positions
| by training an nn model.
|
|
| 

https://www.kaggle.com/leostep/pytorch-dense-network-for-house-pricing-regression
https://medium.com/@benjamin.phillips22/simple-regression-with-neural-networks-in-pytorch-313f06910379
https://www.kaggle.com/graymant/breast-cancer-diagnosis-with-pytorch
https://www.kaggle.com/ratnesh88/breast-cancer-prediction-using-pytorch

'''

from ._nn import Position
import numpy as np
import os
import sys
import matplotlib.pyplot as plt
import torch
from torch import nn
import torch.optim as optim
from torch.autograd import Variable

class trainer:
	def __init__(self, data, device, epoch):
		self.dataloader_ = data
		print(self.dataloader_)
