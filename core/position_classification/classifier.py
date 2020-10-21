



# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██



 --------------------------------------------
|             Position Classifier
| -------------------------------------------
| predict and save positions using
| a pre-trained model.
|
|
|
| self.model                 : classifier pre-trained model
| self.__save                : save prediction on input data into a csv file
| self.__load                : load pre-trained classifier model
| self.predict               : predict output on input data
| self.__call__ 			 : make a prediction on a input data
|
|
|


'''


from sklearn import preprocessing
from ._nn import Position 
import matplotlib.pyplot as plt
import sys
import os
import pickle
import pathlib
import pandas as pd
import numpy as np
import torch
from torch.autograd import Variable


class predictor:
	def __init__(self, **model_conf):
		self.__model_path = model_conf["path"]
		self.__positions = model_conf["positions"]
		self.__data_type = model_conf["data_type"]
		self.__input_features = model_conf["features"]["in"]
		self.__output_features = model_conf["features"]["out"]
		self.__input_data = None
		self.model = None

		cuda = torch.cuda.is_available() if model_conf["device"] is 'cuda' else None
		self.__device = torch.device("cuda" if cuda else "cpu")
		torch.backends.cudnn.benchmark = True


		if os.path.exists(self.__model_path):
			print(f"\n________found existing pre-trained classifier model trained on {self.__data_type} data for prediction________\n")
			self.__load()
		else:
			print(f"\t➢   something went wrong with loading pre-trained classifier model, please make sure there is one at {self.__model_path}")
			sys.exit(1)


	def __load(self):
		try:
			checkpoint = torch.load(self.__model_path)
			print(f"\t➢   loaded pre-trained model from {self.__model_path}\n")
		except IOError:
			print(f"\t➢   can't load pre-trained model from : {self.__model_path}\n")


		self.model = Position(input_neurons=self.__input_features, output_neurons=self.__output_features).to(self.__device)
		self.model.load_state_dict(checkpoint['model_state_dict'])
		self.optimizer = torch.optim.SGD(self.model.parameters(), lr=1e-3, momentum=0.9)
		self.optimizer.load_state_dict(checkpoint['optimizer_state_dict'])
		self.epoch = checkpoint['epoch']
		self.training_loss = checkpoint['training_loss']
		self.training_best_accuracy = checkpoint['training_best_accuracy']
		self.loss_tracker = checkpoint['loss_tracker']
		self.model.eval()


	def __save(self, predictions):
		classifier_obj_path = os.path.dirname(os.path.abspath(__file__))+f'/utils/classifier.obj'
		curr_dir = os.path.dirname(os.path.abspath(__file__))
		input_data = os.path.abspath(curr_dir + f"/../../server/dataset/input_data.csv")
		input_data_classified = os.path.abspath(curr_dir + f"/../../server/dataset/input_data_classified_positions_using-pre-trained_model_on-{self.__data_type}.csv")
		numpy_predictions = predictions.detach().numpy()
		Df = pd.read_csv(input_data)
		Df['position'] = np.array(list(self.__positions[pred] for pred in numpy_predictions))
		Df.to_csv(input_data_classified, index=False)
		print(f"\t➢   new dataset saved in {input_data_classified}\n")
		with open(classifier_obj_path, "wb") as classifier_file:
			pickle.dump(self, classifier_file)
		print(f"\t➢   classifier object saved in {classifier_obj_path}\n")


	def __predict(self):
		if self.__input_data is not None:
			print(f"\n________predicting on input data using pre-trained classifier on {self.__device}________\n")
			_input = Variable(torch.from_numpy(self.__input_data), requires_grad=False)
			_input = _input.to(self.__device)
			outputs = self.model(_input.float())
			predictions = outputs.argmax(dim=1)
			if len(predictions) == 1: # we just classified only one user which is done through /user/classify/position route and we are returning the result back to the route
				position = np.array(list(self.__positions[pred] for pred in predictions.detach().numpy()))
				return position, self.__data_type # we need data type for saving predicted position into its related column
			else: # we just classified a bunch of input data using a csv file and we just saved the results into a csv file to insert them in db using /users/add/positions
				self.__save(predictions)
		else:
			print(f"[?] can't predict on a None data object.")
			sys.exit(1)


	def __call__(self, *args):
		for arg in args:
			if type(arg) is pathlib.PosixPath:
				if os.path.dirname(arg):
					df = pd.read_csv(arg)
					df.drop('user_id', axis=1, inplace=True)
					input_data = df.to_numpy()
					self.__input_data = preprocessing.StandardScaler().fit_transform(input_data)
					self.__predict()
				else:
					print(f"[?] make sure there is csv file at {arg}.")
					sys.exit(1)	
			elif arg is None:
				print(f"[?] data object can't be None.")
				sys.exit(1)
			elif type(arg) is np.ndarray:
				self.__input_data = preprocessing.StandardScaler().fit_transform(arg)
				return self.__predict()
			else:
				print(f"[?] please specify a numpyndarray data object or a csv path of data for prediction.")
				sys.exit(1)
		
