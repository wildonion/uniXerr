


# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██


 -----------------------------------------------------------------------------------
|
| 1 - user_id [int] (registered user id fetched from DB)
| 2 - rollcall_score [float] - average score between 5 (too much absences) to 15 (no absences) for a week
| 3 - class_activity [float] - average score between 5 to 15 for a week
| 4 - discipline [float] - average score between 5 to 15 for a week
| 5 - total_quizzes_avg [float] - the total average of quizzes in a week
|
| __init__   : load the data from csv file
| plot_data_ : plot the data before clustering
| __call__   : normalize and prepare the data for dataloader object
|
|
| Reference : https://pytorch.org/tutorials/beginner/data_loading_tutorial.html
|


'''

import sys
import numpy as np
import os
import matplotlib.pyplot as plt
import pandas as pd
from sklearn import preprocessing
from sklearn.decomposition import PCA
from sklearn.manifold import TSNE
import torch
from torch.utils.data import Dataset


class pipeline(Dataset):
	def __init__(self):
		csv_path = os.path.dirname(os.path.abspath(__file__)) + '/dataset/pc_features.csv'
		self.__data_frame = pd.read_csv(csv_path)
		self.__data_frame['rollcall_score'] = self.__data_frame['rollcall_score'].astype('float64')
		self.__data_frame['class_activity'] = self.__data_frame['class_activity'].astype('float64')
		self.__data_frame['discipline'] = self.__data_frame['discipline'].astype('float64')
		self.__data_frame['total_quizzes_avg'] = self.__data_frame['total_quizzes_avg'].astype('float64')
		self.__call__() # scaling the features


	def __repr__(self):
		return f'\tmean : {np.mean(self.data)}\n\tstd : {np.std(self.data)}\n\tscaler : minmax'


	def __len__(self):
		return len(self.data)

	def __getitem__(self, idx):
		return self.data[idx]

	def __call__(self):
		'''
			standard scaler produce a negative loss in KLD
			because it'll put input data in range [-1, 1]
		'''
		minmax_scaler = preprocessing.MinMaxScaler()
		self.data = minmax_scaler.fit_transform(self.__data_frame.iloc[:, 1:].values)


	def get_raw(self):
		return self.__data_frame.iloc[:, 1:].values


	def plot_data_(self, plot_method='pca'):
		print("\n________plotting before saving dataloader object________\n")
		print(f"\t➢   normalizing data using StandardScaler")
		print(f"\t➢   plotting data using {plot_method} method")
		normalized_data_frame = preprocessing.StandardScaler().fit_transform(self.get_raw())

		if plot_method == 'pca':	
			pca_pc_bn = PCA(n_components=2)
			principalComponents_pc_bn = pca_pc_bn.fit_transform(normalized_data_frame)
			principal_pc_Df_bn = pd.DataFrame(data=principalComponents_pc_bn, columns=['principal_component_1', 'principal_component_2'])
			np.savetxt(os.path.dirname(os.path.abspath(__file__))+'/dataset/pca_comps_variance.out', pca_pc_bn.explained_variance_ratio_, delimiter=',') # load using np.loadtxt()
			plt.figure(figsize=(10,5))
			plt.xticks(fontsize=10)
			plt.yticks(fontsize=12)
			plt.xlabel('Principal Component - 1', fontsize=10)
			plt.ylabel('Principal Component - 2', fontsize=10)
			plt.title("Principal Component Analysis of Position Clustering Dataset", fontsize=10)
			plt.scatter(principal_pc_Df_bn.principal_component_1, principal_pc_Df_bn.principal_component_2, alpha=0.25)
			plt.savefig(os.path.dirname(os.path.abspath(__file__))+'/dataset/pca_pc_beforeClustering.png')
			print(f"\t➢   plot saved at {os.path.dirname(os.path.abspath(__file__))+'/dataset/pca_pc_beforeClustering.png'}\n")


		elif plot_method == 'tsne':
			tsne_pc_bn = TSNE(n_components=2, verbose=1, perplexity=3, n_iter=1000, learning_rate=20)
			tsnecomponents_pc_bn = tsne_pc_bn.fit_transform(normalized_data_frame)
			tsne_pc_Df_bn = pd.DataFrame(data=tsnecomponents_pc_bn, columns=['tsne_component_1', 'tsne_component_2'])
			plt.figure(figsize=(10,5))
			plt.xticks(fontsize=10)
			plt.yticks(fontsize=12)
			plt.xlabel('Component - 1', fontsize=10)
			plt.ylabel('Component - 2', fontsize=10)
			plt.title("T-SNE Analysis of Position Clustering Dataset", fontsize=10)
			plt.scatter(tsne_pc_Df_bn.tsne_component_1, tsne_pc_Df_bn.tsne_component_2, alpha=0.25)
			plt.savefig(os.path.dirname(os.path.abspath(__file__))+'/dataset/tsne_pc_beforeClustering.png')
			print(f"\t➢   plot saved at {os.path.dirname(os.path.abspath(__file__))}+'/dataset/tsne_pc_beforeClustering.png'\n")

		else:
			print("[?] please specify a correct plotting method.")
			sys.exit(1)