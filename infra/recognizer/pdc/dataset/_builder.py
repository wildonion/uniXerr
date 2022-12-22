


import pandas as pd
from torch.utils.data import Dataset
import sys, os
import numpy as np
import math


__all__ = ['PersianAlphabetDataset']


class PersianAlphabetDataset(Dataset):
	def __init__(self, csv_files, transform=None):
		if not os.path.exists(csv_files[0]) or not os.path.exists(csv_files[1]): print("[?] CSV Dataset Not Found!"); sys.exit(1)
		self.transform  = transform
		self.images     = pd.read_csv(csv_files[0], header=None)
		self.labels     = pd.read_csv(csv_files[1], header=None)

		self.images = self.images.values.astype('float32')
		self.labels = self.labels.values.astype('int32')-1 # 0 to len(self.labels) - 1
		
		labels_sequences = {number[0]: np.count_nonzero(self.labels == number[0]) for number in self.labels} # or (unique, counts) = np.unique(self.labels, return_counts=True)
		img_label        = {i: self.labels[i, 0] for i in range(len(self.labels))}
		one_hot_labels   = np.zeros((len(self.images), len(labels_sequences)))
		for j in range(len(one_hot_labels)):
			one_hot_labels[j, img_label[j]] = 1
		self.labels = one_hot_labels


		image_size = int(np.sqrt(self.images.shape[1]))
		self.images = self.images.reshape([-1, 1, image_size, image_size]) # reshape to torch image in here - we can also do it in ToTensor transform function


	def __len__(self):
		return len(self.images)


	def __getitem__(self, idx):
		sample = self.images[idx]
		label  = self.labels[idx]
		if self.transform:
			sample = self.transform(sample)
		return sample, label