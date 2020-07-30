



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
| 6 - position [char] - the position of the student 
|
|
|


'''

from copy import deepcopy as dc
import sys
import numpy as np
import os
import matplotlib.pyplot as plt
import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn import preprocessing


class pipeline():
	def __init__(self, csv_path):
		data_frame = pd.read_csv(csv_path)
		self.__plotting_data_frame = dc(data_frame)
		data_frame.drop('user_id', axis=1, inplace=True) # drop user_id
		labels = data_frame.select_dtypes(include=[object]).apply(preprocessing.LabelEncoder().fit_transform)
		one_hot_labels = preprocessing.OneHotEncoder().fit_transform(labels).toarray()
		data_frame.drop('position', axis=1, inplace=True) # drop labels
		scaled_data = preprocessing.StandardScaler().fit_transform(data_frame)
		self.x_train, self.x_test, self.y_train, self.y_test = train_test_split(scaled_data, one_hot_labels, test_size=0.2, random_state=85)