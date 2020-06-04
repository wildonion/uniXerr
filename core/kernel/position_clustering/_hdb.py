





# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██


 --------------------------------------------
|               HDBSCAN model
| -------------------------------------------
| cluster students using HDBSCAN
|
| 
|

'''

import pandas as pd
import numpy as np
import os
import seaborn as sns
from sklearn import preprocessing
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from sklearn.decomposition import PCA
from sklearn.manifold import TSNE
import numpy as np
# import hdbscan


class hdb():
	def __init__(self, data, param_kwargs):
		
		self.__data = data
		self.positions = {-1:'ooup', 0:'A', 1:'B', 2:'C', 3:'D', 4:'E'}

		# min_cluster_size : minimum number of samples in a cluster
		# min_sample       : minimum number of samples to make the a cluster and prevent from noisy clustering
		clusterer = hdbscan.HDBSCAN(**param_kwargs)
		clusterer.fit(self.__data)
		self.__clusterer_labels = clusterer.labels_
		self.__clusterer_probabilities = clusterer.probabilities_
		np.savetxt(os.path.dirname(os.path.abspath(__file__))+'/utils/HDBSCAN-cluster_labels.out', self.__clusterer_labels, delimiter=',')
		np.savetxt(os.path.dirname(os.path.abspath(__file__))+'/utils/HDBSCAN-cluster_prob.out', self.__clusterer_probabilities, delimiter=',')
		print(f"\t---total unique labels found : {np.unique(self.__clusterer_labels).max() + 1}")
		print(f"\t---all unique label : {np.unique(self.__clusterer_labels)}")



	def __getitem__(self, sample_idx):
		'''
			return the cluster of idx-th sample and its score - [0, 19] , [0, 1]
		'''
		return self.__clusterer_labels[sample_idx], self.__clusterer_probabilities[sample_idx]


	def export_csv(self):
		print("\n________setting clustered labels on pc_features dataset________\n")
		curr_dir = os.path.dirname(os.path.abspath(__file__))
		pc_features = os.path.abspath(curr_dir + "/../../server/streamer/pc/dataset/pc_features.csv")
		Df = pd.read_csv(pc_features)
		Df['position'] = np.array(list(self.positions[clus_idx] for clus_idx in self.__clusterer_labels))
		Df.to_csv(curr_dir+"/utils/pc_features_labeled.csv", index=False)
		print(f"\t---new dataset saved in {curr_dir+'/utils/pc_features_labeled.csv'}\n")


	def plot_clusters(self, method='pca'):
		print("\n________plotting after clustering________\n")
		print(f"\t---extracting components using {method} method")
		fig_path = os.path.dirname(os.path.abspath(__file__))+f'/utils/clusters-{self.__class__.__name__}-{method}.png'
		

		if method is 'pca':
			pca_data = PCA(n_components=2)
			reductioned_data = pca_data.fit_transform(self.__data)
			np.savetxt(os.path.dirname(os.path.abspath(__file__))+f'/utils/pca_comps_variance_{self.__class__.__name__}.out', pca_data.explained_variance_ratio_, delimiter=',')

		if method is 'tsne':
			tsne_data = TSNE(n_components=2)
			reductioned_data = tsne_data.fit_transform(self.__data)
		

		results = pd.DataFrame(reductioned_data, columns=['component_1','component_2'])
		colors_map = {-1:'#e6194b', 0:'#eac435', 1:'#345995', 2:'#03cea4', 3:'#fb4d3d', 4:'#ca1551', 5:'#bbdef0'}
		pos = ['ooup', 'A', 'B', 'C', 'D', 'E', 'F']
		class_colours = ["#e6194b", "#eac435","#345995","#03cea4","#fb4d3d","#ca1551","#bbdef0"]

		# https://stackoverflow.com/questions/26558816/matplotlib-scatter-plot-with-legend/26559256
		rectangles  = []
		for i in range(0, len(class_colours)):
		    rectangles.append(mpatches.Rectangle((0, 0), 1, 1, fc=class_colours[i])) # creating a rectangle bar for each color


		plt.figure(figsize=(10,5))
		sns.scatterplot(x="component_1", y="component_2", hue=self.__clusterer_labels, data=results, legend='full', palette=colors_map)
		plt.title('Clusters found by HDBSCAN', fontsize=10)
		plt.legend(rectangles, pos, loc=4, prop={'size': 6}) # loc=4 : lower right
		plt.savefig(fig_path)
		print(f"\t---plot saved at {fig_path}\n")