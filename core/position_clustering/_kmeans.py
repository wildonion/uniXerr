

# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██


 --------------------------------------------
|                KMeans model
| -------------------------------------------
| cluster students using KMeans
|
| 
|

'''

import sys
import torch
from sklearn.metrics import silhouette_score
from sklearn.cluster import KMeans
import numpy as np
import os
import seaborn as sns
from sklearn.preprocessing import StandardScaler
from mpl_toolkits.mplot3d import Axes3D
import matplotlib.patches as mpatches
import matplotlib.pyplot as plt
from sklearn.decomposition import PCA
from sklearn.manifold import TSNE
import numpy as np
import pandas as pd


class kmeans():
	def __init__(self, data, data_type):
		'''
			we don't need scale our data or reduce dimension to fit into the kmeans, pca and tsne algorithm
			cause the data is the latent space of VAE model and has a normal distribution and 2D dims.
			
			the normal distribution stretches from -Infinity to +Infinity. 
			the mean of the distribution is the location of the value with 
			the highest likelihood, which could be anywhere. 
			the mean can be positive, negative or zero.
			
			clustering is difficult to do in high dimensions because 
			the distance between most pairs of points is similar. 
			using an autoencoder lets you re-represent high dimensional 
			points in a lower-dimensional space. it doesn't do clustering 
			but it is a useful preprocessing step for a secondary clustering step.
		'''		

		if data_type == 'latent':
			self.__data = data
			self.__ds_info = f'\tmean : {np.mean(self.__data)}\n\tstd : {np.std(self.__data)}\n\tPDF : normal'
		elif data_type == 'raw':
			pca_pc_bn = PCA(n_components=2) # generally K-means works best for 2 dimensional numerical data, reduce from 4 features to 2.
			scaler = StandardScaler() # scale data before clustering
			self.__data = pca_pc_bn.fit_transform(scaler.fit_transform(data))
			self.__ds_info = f'\tmean : {np.mean(self.__data)}\n\tstd : {np.std(self.__data)}\n\tStandard : scaler\n\tPDF : standard normal\n\tDRA : PCA'
		else:
			print("[?] please specify a data type")
			sys.exit(1)

		self.positions = {0:'A', 1:'B', 2:'C', 3:'D', 4:'E', 5:'F'}

		clusterer = KMeans(n_clusters=6, random_state=10).fit(self.__data)
		self.__clusterer_labels = clusterer.labels_
		self.__clusterer_centers = clusterer.cluster_centers_
		np.savetxt(os.path.dirname(os.path.abspath(__file__))+'/utils/KMEANS-cluster_labels.out', self.__clusterer_labels, delimiter=',')
		np.savetxt(os.path.dirname(os.path.abspath(__file__))+'/utils/KMEANS-cluster_centers.out', self.__clusterer_centers, delimiter=',') # centeroids are (n_cluster, features)
		print(f"\t---total labels found : {np.unique(self.__clusterer_labels).max() + 1}")
		print(f"\t---all unique labels : {np.unique(self.__clusterer_labels)}")


	def __repr__(self):
		return self.__ds_info


	def __getitem__(self, sample_idx):
		return self.__clusterer_labels[sample_idx]


	def export_csv(self):
		print("\n________setting clustered labels on pc_features dataset________\n")
		curr_dir = os.path.dirname(os.path.abspath(__file__))
		pc_features = os.path.abspath(curr_dir + "/../../server/dataset/pc_features.csv")
		pc_features_labeled = os.path.abspath(curr_dir + "/../../server/dataset/pc_features_labeled.csv")
		Df = pd.read_csv(pc_features)
		Df['position'] = np.array(list(self.positions[clus_idx] for clus_idx in self.__clusterer_labels))
		Df.to_csv(pc_features_labeled, index=False)
		print(f"\t---new dataset saved in {pc_features_labeled}\n")


	def plot_clusters(self, method='pca'):
		print("\n________plotting after clustering________\n")
		print(f"\t---extracting components using {method} method")
		fig_path = os.path.dirname(os.path.abspath(__file__))+f'/utils/clusters-{self.__class__.__name__}-{method}.png'
		
		if method == 'pca':
			pca_data = PCA(n_components=2)
			reductioned_data = pca_data.fit_transform(self.__data)
			np.savetxt(os.path.dirname(os.path.abspath(__file__))+f'/utils/pca_comps_variance_{self.__class__.__name__}.out', pca_data.explained_variance_ratio_, delimiter=',')

		elif method == 'tsne':
			tsne_data = TSNE(n_components=2)
			reductioned_data = tsne_data.fit_transform(self.__data)

		else:
			print("[?] please specify a correct plotting method.")
			sys.exit(1)


		results = pd.DataFrame(reductioned_data, columns=['component_1','component_2'])
		colors_map = {0:'#eac435', 1:'#345995', 2:'#03cea4', 3:'#fb4d3d', 4:'#ca1551', 5:'#bbdef0'}
		pos = ['A', 'B', 'C', 'D', 'E', 'F']
		class_colours = ["#eac435","#345995","#03cea4","#fb4d3d","#ca1551","#bbdef0"]


		# https://stackoverflow.com/questions/26558816/matplotlib-scatter-plot-with-legend/26559256
		rectangles  = []
		for i in range(0, len(class_colours)):
			rectangles.append(mpatches.Rectangle((0, 0), 1, 1, fc=class_colours[i])) # creating a rectangle bar for each color
		
		plt.figure(figsize=(10,5))
		sns.scatterplot(x="component_1", y="component_2", hue=self.__clusterer_labels, data=results, legend='full', palette=colors_map)
		plt.title('Clusters found by KMeans', fontsize=10)
		plt.legend(rectangles, pos, loc=4, prop={'size': 6})
		plt.savefig(fig_path)
		print(f"\t---plot saved at {fig_path}\n")
