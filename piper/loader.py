

# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██



 --------------------------------------------
|        Dataset Loader for CSV files
| -------------------------------------------
|
| load the csv dataset 
| through pipeline class
|
| __call__              : dataloader object using __call__ method
| save                  : save dataloader object(s)
| __load_dataloader     : load dataloader object(s)
|

'''


from ._classification_pipeline import pipeline as clapi
from ._clustering_pipeline import pipeline as cluspi
from .dataset import faker
import os.path
import torch
from torch.utils.data import DataLoader
import torch.utils.data as data_utils


class Loader:
	def __init__(self, *args, **kwargs):
		
		self.__training_dataloader = None
		self.__testing_dataloader = None
		self.__testing_dataloader_path = None
		self.__training_dataloader_path = None
		
		if "csv_path" in kwargs:
			self.__path = kwargs["csv_path"] # path to labeled csv pc_features
			path_str = str(self.__path)
			type_of_labeled_data = path_str[len(path_str)-10:len(path_str)-4] if path_str[len(path_str)-10:len(path_str)-4] == "latent" else path_str[len(path_str)-7:len(path_str)-4] 
			self.__training_dataloader_path = os.path.dirname(os.path.abspath(__file__)) + f'/dataset/pc_features_labeled_training_tensors-{type_of_labeled_data}-DATALOADER.pth'
			self.__testing_dataloader_path = os.path.dirname(os.path.abspath(__file__)) + f'/dataset/pc_features_labeled_testing_tensors-{type_of_labeled_data}-DATALOADER.pth'
		
		else:
			self.__path = os.path.dirname(os.path.abspath(__file__)) + '/dataset/pc_features.csv'
			self.__training_dataloader_path = os.path.dirname(os.path.abspath(__file__)) + f'/dataset/pc_features-DATALOADER.pth'

		

		if self.__check_path() == "T&T":
			self.__load_dataloader(load_testing_dataloader=True)
		elif self.__check_path() == "T":
			self.__load_dataloader(load_testing_dataloader=False)

		if not self.__check_path():
			print("\n________found no existing dataloader object________\n")
			print(f"\t➢   building dataloader object through the pipeline from {self.__path}\n")
			if kwargs["process"] == "classification":
				pipeline = clapi(self.__path)
				testing_tensors_dataset = data_utils.TensorDataset(torch.from_numpy(pipeline.x_test).float(), torch.from_numpy(pipeline.y_test).float())
				self.__testing_dataloader = DataLoader(testing_tensors_dataset, batch_size=kwargs["batch_size"], shuffle=True, **kwargs["dataloader_kwargs"], drop_last=True)
				training_tensors_dataset = data_utils.TensorDataset(torch.from_numpy(pipeline.x_train).float(), torch.from_numpy(pipeline.y_train).float())
				self.__training_dataloader = DataLoader(training_tensors_dataset, batch_size=kwargs["batch_size"], shuffle=True, **kwargs["dataloader_kwargs"], drop_last=True)
			if kwargs["process"] == "clustering":
				pipeline = cluspi()
				self.__training_dataloader = DataLoader(pipeline, batch_size=kwargs["batch_size"], shuffle=True, **kwargs["dataloader_kwargs"], drop_last=True)
			if "plotting_kwargs" in kwargs:
				pipeline.plot_data_(kwargs["plotting_kwargs"])
			self.__save()

		if "generate_fake_samples" in kwargs:
			new_dataloader_flag = False
			if kwargs["generate_fake_samples"]:
				print("\n________generating fake samples________\n")
				faker.generate(samples=12000) # overide perviouse dataset
				print(f"\t➢   new dataset saved at {self.__path}\n")
				new_dataloader_flag = True
			if new_dataloader_flag:
				print(f"\t➢   building new dataloader object through the pipeline from {self.__path}\n")
				pipeline = cluspi()
				pipeline.plot_data_(kwargs["plotting_kwargs"])
				self.__training_dataloader = DataLoader(pipeline, batch_size=kwargs["batch_size"], shuffle=True, **kwargs["dataloader_kwargs"], drop_last=True)
				self.__save()


	def __save(self): # save the dataloader objects for related process
		'''
			pickle does not save the pipeline class itself. rather, it saves a path to the file containing the class
			path to save by pickle : server._*_pipeline >>> * is either classification or clustering
		'''
		try:
			if self.__testing_dataloader:
				print("\n________saving dataloader objects________\n")
				torch.save(self.__training_dataloader, self.__training_dataloader_path)
				print(f"\t➢   saved training dataloader object at : {self.__training_dataloader_path}\n")
				torch.save(self.__testing_dataloader, self.__testing_dataloader_path)
				print(f"\t➢   saved testing dataloader object at : {self.__testing_dataloader_path}\n")
			else:
				print("\n________saving training dataloader object________\n")
				torch.save(self.__training_dataloader, self.__training_dataloader_path)
				print(f"\t➢   saved training dataloader object at : {self.__training_dataloader_path}\n")
		except IOError:
			if self.__testing_dataloader:
				print(f"\t➢   can't save testing dataloader object at : {self.__testing_dataloader_path}\n")
				print(f"\t➢   can't save training dataloader object at : {self.__training_dataloader_path}\n")
			else:
				print(f"\t➢   can't save training dataloader object at : {self.__training_dataloader_path}\n")


	def __call__(self):
		if self.__testing_dataloader:
			return self.__training_dataloader, self.__testing_dataloader
		else:
			return self.__training_dataloader


	def __load_dataloader(self, load_testing_dataloader):
		try:
			if load_testing_dataloader:
				print("\n________found existing dataloader objects________\n")
				self.__training_dataloader = torch.load(self.__training_dataloader_path)
				self.__testing_dataloader = torch.load(self.__testing_dataloader_path)
				print(f"\t➢   loaded training dataloader object from {self.__training_dataloader_path}\n")
				print(f"\t➢   loaded testing dataloader object from {self.__testing_dataloader_path}\n")
			else:
				print("\n________found existing dataloader object________\n")
				self.__training_dataloader = torch.load(self.__training_dataloader_path)
				print(f"\t➢   loaded training dataloader object from {self.__training_dataloader_path}\n")
		except IOError:
			if load_testing_dataloader:
				print(f"\t➢   can't load testing dataloader object from : {self.__testing_dataloader_path}\n")
				print(f"\t➢   can't load training dataloader object from : {self.__training_dataloader_path}\n")
			else:
				print(f"\t➢   can't load training dataloader object from : {self.__training_dataloader_path}\n")


	def __check_path(self):
		if self.__testing_dataloader_path and os.path.exists(self.__training_dataloader_path) and os.path.exists(self.__testing_dataloader_path):
			return "T&T"
		elif os.path.exists(self.__training_dataloader_path):
			return "T"
		else:
			return False



class ClusteringDatasetLoader(Loader):
	def __init__(self, batch_size, generate_fake_samples, plotting_kwargs, dataloader_kwargs):
		
		args = {"batch_size":batch_size, "generate_fake_samples": generate_fake_samples, 
			    "plotting_kwargs":plotting_kwargs, "dataloader_kwargs": dataloader_kwargs, "process":"clustering"}
		super().__init__(**args)



class ClassificationDatasetLoader(Loader):
	def __init__(self, csv_path, batch_size, dataloader_kwargs):
		
		args = {"csv_path": csv_path, "batch_size": batch_size, 
		        "dataloader_kwargs": dataloader_kwargs, "process":"classification"}
		super().__init__(**args)

