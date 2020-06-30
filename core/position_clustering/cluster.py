



# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██



 ------------------------------
|        labels class
| -----------------------------
|
| return the clusters from a 
| latent space or a numpy data
|
|
| plot        : plot the clusters
| __getitem__ : return the cluster of idx-th sample - [0, 19]
| get_position: return the related position of a cluster - ex : 0 -> A1 , 1 -> A2 , ... , 19 -> D5
| set         : export a csv of dataset with their labels
|
|

'''

import sys
import torch
from ._hdb import hdb
from ._kmeans import kmeans
import numpy as np

class labels:

	def __init__(self, data, data_type, cluster_method='kmeans'):

		print(f"\n________clustering using {cluster_method} method________\n")



		# 		 hdbscan has some drawbacks and issues with new dataset features
		# 		 you have to change the params each time you create a new dataset
		# 		 for now we were satisfied with kmeans method.



		if cluster_method == 'hdbscan':
			# param_kwargs = {'min_cluster_size':45, 'min_samples':5}
			# self.__model = hdb(data=data, data_type=data_type, param_kwargs=param_kwargs)
			print("[?] Not Implemented.")
			sys.exit(1)


		elif cluster_method == 'kmeans':
			if data is not None or type(data).__module__ == np.__name__:
				self.__model = kmeans(data=data, data_type=data_type)
			else:
				print("[?] please specify a numpyndarray data for clustering.")
				sys.exit(1)


		else:
			print("[?] please specify a correct clustering method.")
			sys.exit(1)
			

	def plot(self, method):
		self.__model.plot_clusters(method=method)


	def __getitem__(self, sample_idx):
		return self.__model[sample_idx]

	def get_position(self, cluster):
		return self.__model.positions[cluster]

	def dataset_info(self):
		return self.__model.__repr__()

	def set(self):
		self.__model.export_csv()







