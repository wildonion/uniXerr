

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


from ._pipeline import pipeline
from .dataset import faker
from torch.utils.data import DataLoader
import os.path
import torch

class DatasetLoader():
	def __init__(self, batch_size, generate_fake_samples, plotting_kwargs, dataloader_kwargs):
		
		self.pc_dl = None
		self.__path = os.path.dirname(os.path.abspath(__file__)) + '/dataset/pc_dataloader.pth'
		new_dataloader_flag = False
		
		if generate_fake_samples:
			print("\n________generating fake samples________\n")
			faker.generate(samples=12000) # overide perviouse dataset
			print(f"\t---new dataset saved at {os.path.dirname(os.path.abspath(__file__)) + '/dataset/pc_features.csv'}\n")
			new_dataloader_flag = True 
		
		if self.__check_path():
			self.__load_dataloader()

		if (new_dataloader_flag) or (not self.__check_path()):
			print("\n________found no existing dataloader object________\n")
			print(f"\t---building dataloader object through the pipeline from {os.path.dirname(os.path.abspath(__file__)) + '/dataset/pc_features.csv'}\n")
			pc_ds_pipeline = pipeline()
			pc_ds_pipeline.plot_data_(plotting_kwargs)
			self.pc_dl = DataLoader(pc_ds_pipeline, batch_size=batch_size, shuffle=True, **dataloader_kwargs, drop_last=True)
			self.__save()


	def __check_path(self):
		if os.path.exists(self.__path):
			return True
		else:
			return False

	def __call__(self):
		return self.pc_dl


	def __save(self): # save the dataloader
		'''
			pickle does not save the pipeline class itself. rather, it saves a path to the file containing the class
			path to save by pickle : server._pipeline
		'''
		try:
			print("\n________saving dataloader object________\n")
			torch.save(self.pc_dl, self.__path)
			print(f"\t---saved dataloader object at : {self.__path}\n")
		except IOError:
			print(f"\t---can't save dataloader object at : {self.__path}\n")

	def __load_dataloader(self): # load the dataloader
		try:
			print("\n________found existing dataloader object________\n")
			self.pc_dl = torch.load(self.__path)
			print(f"\t---loaded dataloader object from {self.__path}\n")
		except IOError:
			print(f"\t---can't load dataloader object from : {self.__path}\n")
