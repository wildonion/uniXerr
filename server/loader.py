

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
| save                  : save a dataloader object
| __load_dataloader     : load a dataloader object
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
		
		self.dataloader = None
		self.__testing_tensors_dataset = None
		
		if "csv_path" in kwargs:
			self.__path = kwargs["csv_path"] # path to labeled csv pc_features
			path_str = str(self.__path)
			type_of_labeled_data = path_str[len(path_str)-10:len(path_str)-4] if path_str[len(path_str)-10:len(path_str)-4] == "latent" else path_str[len(path_str)-7:len(path_str)-4] 
			self.__dataloader_path = os.path.dirname(os.path.abspath(__file__)) + f'/dataset/pc_labeled_dataloader-{type_of_labeled_data}.pth'
			self.__testing_tensors_dataset_path = os.path.dirname(os.path.abspath(__file__)) + f'/dataset/pc_labeled_testing_tensors-{type_of_labeled_data}.pth'
		
		else:
			self.__path = os.path.dirname(os.path.abspath(__file__)) + '/dataset/pc_features.csv'
			self.__dataloader_path = os.path.dirname(os.path.abspath(__file__)) + f'/dataset/pc_dataloader.pth'

		

		if self.__check_path():
			self.__load_dataloader()

		if not self.__check_path():
			print("\n________found no existing dataloader object________\n")
			print(f"\t---building dataloader object through the pipeline from {self.__path}\n")
			if kwargs["process"] == "classification":
				pipeline = clapi(self.__path)
				self.__testing_tensors_dataset  = data_utils.TensorDataset(torch.from_numpy(pipeline.x_test).float(), torch.from_numpy(pipeline.y_test).float())
				training_tensors_dataset = data_utils.TensorDataset(torch.from_numpy(pipeline.x_train).float(), torch.from_numpy(pipeline.y_train).float())
				self.dataloader = DataLoader(training_tensors_dataset, batch_size=kwargs["batch_size"], shuffle=True, **kwargs["dataloader_kwargs"], drop_last=True)
			if kwargs["process"] == "clustering":
				pipeline = cluspi()
				self.dataloader = DataLoader(pipeline, batch_size=kwargs["batch_size"], shuffle=True, **kwargs["dataloader_kwargs"], drop_last=True)
			if "plotting_kwargs" in kwargs:
				pipeline.plot_data_(kwargs["plotting_kwargs"])
			self.__save()

		if "generate_fake_samples" in kwargs:
			new_dataloader_flag = False
			if kwargs["generate_fake_samples"]:
				print("\n________generating fake samples________\n")
				faker.generate(samples=12000) # overide perviouse dataset
				print(f"\t---new dataset saved at {self.__path}\n")
				new_dataloader_flag = True
			if new_dataloader_flag:
				print(f"\t---building new dataloader object through the pipeline from {self.__path}\n")
				pipeline = cluspi()
				pipeline.plot_data_(kwargs["plotting_kwargs"])
				self.dataloader = DataLoader(pipeline, batch_size=kwargs["batch_size"], shuffle=True, **kwargs["dataloader_kwargs"], drop_last=True)
				self.__save()


	def get_testing_tensors_dataset(self):
		return self.__testing_tensors_dataset

	def get_testing_tensors_dataset_path(self):
		return self.__testing_tensors_dataset_path

	def __save(self): # save the dataloader object for related process
		'''
			pickle does not save the pipeline class itself. rather, it saves a path to the file containing the class
			path to save by pickle : server._*_pipeline - * is either classification or clustering
		'''
		try:
			print("\n________saving dataloader object________\n")
			torch.save(self.dataloader, self.__dataloader_path)
			print(f"\t---saved dataloader object at : {self.__dataloader_path}\n")
		except IOError:
			print(f"\t---can't save dataloader object at : {self.__dataloader_path}\n")


	def __call__(self):
		return self.dataloader


	def __load_dataloader(self):
		try:
			print("\n________found existing dataloader object________\n")
			self.dataloader = torch.load(self.__dataloader_path)
			print(f"\t---loaded dataloader object from {self.__dataloader_path}\n")
		except IOError:
			print(f"\t---can't load dataloader object from : {self.__dataloader_path}\n")


	def __check_path(self):
		if os.path.exists(self.__dataloader_path):
			return True
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

		if self.__check_testing_tensors_dataset_path():
			self.__load_testing_tensors_dataset()
		if not self.__check_testing_tensors_dataset_path():
			self.__save_testing_tensors_dataset()


	def __check_testing_tensors_dataset_path(self):
		if os.path.exists(self.get_testing_tensors_dataset_path()):
			return True
		else:
			return False


	def __load_testing_tensors_dataset(self):
		try:
			print("\n________found existing testing tensors dataset________\n")
			self.testing_dataset_ = torch.load(self.get_testing_tensors_dataset_path())
			print(f"\t---loaded tensors from {self.get_testing_tensors_dataset_path()}\n")
		except IOError:
			print(f"\t---can't load tensors from : {self.get_testing_tensors_dataset_path()}\n")


	def __save_testing_tensors_dataset(self):
		try:
			print("\n________saving testing tensors dataset________\n")
			torch.save(self.get_testing_tensors_dataset(), self.get_testing_tensors_dataset_path())
			print(f"\t---saved tensors at : {self.get_testing_tensors_dataset_path()}\n")
			self.testing_dataset_ = self.get_testing_tensors_dataset()
		except IOError:
			print(f"\t---can't save tensors at : {self.get_testing_tensors_dataset_path()}\n")

