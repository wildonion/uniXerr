

# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██



 ------------------------------------------------
|  coin generation (credit scoring) model
| -----------------------------------------------
| generate score (number of coins) 
| based on positions and other features
|
|
| minimum number of score for everyone generated
| by uinXerr protocol is 10, because the minimum 
| of each feature is 5.
|
|
|	     COIN GENERATION IDEAS
| ✅ idea-1: a hybrid evolutionary algorithms like GA
| ✅ idea-2: an expert system based on neuro-fuzzy system
|
|
| https://github.com/PyTorchLightning/pytorch-lightning
| https://blog.openmined.org/upgrade-to-federated-learning-in-10-lines/
|

'''

import os
import sys
import pandas as pd
import torch
from torch import nn
from torch.autograd import Variable
# from pytorch_lightning.core.lightning import LightningModule


# class CoinGenerationModel(LightningModule):
# 	def __init__(self, device):
# 		super().__init__()

# 	def forward(self):
# 		pass

# 	def training_step(self):
# 		pass


# 	def configure_optimizers(self):
# 		pass

# 	def train_dataloader(self):
# 		pass
