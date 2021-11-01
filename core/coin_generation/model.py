

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
| credit score generation (number of coins) 
| based on position clustering
|
|
| minimum number of score for everyone generated
| by uinXerr protocol is 100, because the minimum 
| of each feature is 5.
|
|
|
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
