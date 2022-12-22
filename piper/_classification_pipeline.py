



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
		path_str = str(csv_path)
		self.__type_of_labeled_data = path_str[len(path_str)-10:len(path_str)-4] if path_str[len(path_str)-10:len(path_str)-4] == "latent" else path_str[len(path_str)-7:len(path_str)-4] 
		data_frame = pd.read_csv(csv_path)
		self.__plotting_data_frame = dc(data_frame)
		data_frame.drop('user_id', axis=1, inplace=True) # drop user_id
		labels = data_frame.select_dtypes(include=[object]).apply(preprocessing.LabelEncoder().fit_transform)
		one_hot_labels = preprocessing.OneHotEncoder().fit_transform(labels).toarray()
		data_frame.drop('position', axis=1, inplace=True) # drop labels
		scaled_data = preprocessing.StandardScaler().fit_transform(data_frame)
		self.x_train, self.x_test, self.y_train, self.y_test = train_test_split(scaled_data, one_hot_labels, test_size=0.2, random_state=85)
		self.__plot_data_()

	def __plot_data_(self):
		print("\n________plotting before saving dataloader object________\n")
		plot_dir = os.path.dirname(os.path.abspath(__file__))+f'/dataset/pp_pc_beforeClassification-{self.__type_of_labeled_data}.png'
		students_position_percentage = (self.__plotting_data_frame.groupby('position').size()/self.__plotting_data_frame['position'].count())*100
		fig, ax = plt.subplots(figsize=(6, 3), subplot_kw=dict(aspect="equal"))
		wedges, texts, autotexts = ax.pie(students_position_percentage, labels=['A', 'B', 'C', 'D', 'E', 'F'], autopct='%1.2f%%', textprops=dict(color="w"))
		ax.legend(wedges, ['A', 'B', 'C', 'D', 'E', 'F'], title="Positions", loc="center left", bbox_to_anchor=(1, 0, 0.5, 1))
		ax.set_title("Positions Percentage")
		plt.setp(autotexts, size=8, weight="bold")
		plt.savefig(plot_dir)
		print(f"\t➢   plot saved at {plot_dir}\n")