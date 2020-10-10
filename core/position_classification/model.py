

# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██



 --------------------------------------------
|          	     trainer class
| -------------------------------------------
| a classifier trainer 
| and tester for positions
| 
|
|
| access granted for 		   : dataloader_ object and classifier
| self.training_dataloader_    : tensor([x_train], [y_train])
| self.testing_dataloader_    : tensor([x_test], [y_test])
| self.__classifier  		   : classifier nn model
| self.__train       		   : train classifier model on training data
| self.__test        		   : test classifier model on testing data
| self.__get_sample  		   : get a sample batch of training dataloader
| self.__save        		   : save trained classifier model
| self.__plot_acc    		   : plot accuracy of training data
| self.__plot_loss   		   : plot loss of training data
| self.__plot_testing_accuracy : plot accuracy of testing data
| self.__call__      		   : start training on training dataloader
|
|
|


'''

from ._nn import Position
import numpy as np
import os
import subprocess
import operator
import sys
import matplotlib.pyplot as plt
import torch
from torch import nn
import torch.optim as optim
from torch.utils.tensorboard import SummaryWriter


class trainer:
	def __init__(self, device, epoch, data_type):
		self.training_dataloader_ = None
		self.testing_dataloader_ = None
		self.__model_path = os.path.dirname(os.path.abspath(__file__)) + f'/utils/pc_model_classifier-{data_type}.pth' 
		self.__checkpoint = None
		self.__classifier = None
		self.__data_type = data_type
		self.__tb_log = False
		self.__epoch = epoch
		self.__history = { "loss": [], "accuracy": []}
		self.__testing_history = {"accuracy": []}


		cuda = torch.cuda.is_available() if device is 'cuda' else None
		self.__device = torch.device("cuda" if cuda else "cpu")
		torch.backends.cudnn.benchmark = True
		

	def __train(self):
		if self.__tb_log:
			writer = SummaryWriter(f'runs/position_classification_training-{self.__data_type}')
			# CrossEntropyLoss (used for multi-class classification) does not expect a one-hot encoded vector as the target, 
			# but class indices in one-hot encoding. The input is expected to contain scores for each class
			# input has to be a 2D Tensor of size (minibatch, C). This criterion expects a class index (0 to C-1) as the target 
			# for each value of a 1D tensor of size minibatch
			criterion = torch.nn.CrossEntropyLoss()
			optimizer = torch.optim.SGD(self.__classifier.parameters(), lr=1e-3, momentum=0.9)
			for e in range(self.__epoch):
				running_loss = 0.0
				self.__classifier.train()
				for batch_ndx, sample in enumerate(self.training_dataloader_): # len(self.training_dataloader_) iterations
					inputs, labels = sample # get a batch sample
					inputs = inputs.to(self.__device)
					optimizer.zero_grad() # set all previous calculated gradients to zero
					output = self.__classifier(inputs).to(self.__device) # feed inputs into the model
					winners = output.argmax(dim=1) # calculate the most prob of predictions, return inputs.size(0) indices of most prob positions in each row
					labels_long_tensor = labels.nonzero(as_tuple=True) # all nonezero values with their coressponding indices
					corrects = (winners == labels_long_tensor[1]) # list of corrects - labels_long_tensor[0] is batch indices and labels_long_tensor[1] is indices of nonzero value
					accuracy = 100*corrects.sum().float()/float(labels.size(0)) # the ratio of number of correct predictions to the total number of input samples
					loss = criterion(output, labels.argmax(dim=1)) # calculate the loss between output and labels
					loss.backward() # calculate the gradient using computational graph for all weights
					optimizer.step() # update weights and other parameters like biases
					running_loss += loss.item()
					if (batch_ndx % 30 == 29): # print logging data every 30 sample of inputs.size(0) batch (mini-batch) - we divided the running_loss (sum of all every 30 samples loss) by 30 cause we want to show the loss of each 30 samples
						print('[epoch ⇀  %d, sample ⇀  %d, batch size ⇀  %d] \n ↳  loss: %.3f acc: %.3f' % (e + 1, batch_ndx + 1, inputs.size(0), running_loss/30, accuracy))
						writer.add_scalar('training loss', running_loss/30, e * len(self.training_dataloader_) + batch_ndx) # log the running loss on tensorboard
						writer.add_scalar('training accuracy', accuracy, e * len(self.training_dataloader_) + batch_ndx) # log the running accuracy on tensorboard
						running_loss = 0.0
				print("______________________________________________________")
				self.__history["loss"].append(loss.item())
				self.__history["accuracy"].append(accuracy)
			writer.flush()
			writer.close()
			print('\n➲   Finished Training')
			index, value = max(enumerate(self.__history["accuracy"]), key=operator.itemgetter(1))
			print("➲   Best accuracy was {} at epoch {}".format(value, index+1))


			self.__checkpoint = {
				'model_state_dict': self.__classifier.state_dict(),
				'optimizer_state_dict': optimizer.state_dict(),
				'epoch': e+1,
				'training_loss': self.__history["loss"][-1],
				'training_best_accuracy': value, # best accuracy
				'loss_tracker': self.__history["loss"]
			}
			
			self.__plot_acc()
			self.__plot_loss()



	def __test(self):
		print(f"\n________predicting on testing {self.__data_type} data using pre-trained classifier on {self.__device}________\n")
		if self.__tb_log:
			writer = SummaryWriter(f'runs/position_classification_testing-{self.__data_type}')
			corrects = 0
			total = 0
			with torch.no_grad():
				for data in self.testing_dataloader_: # total testing data = len(self.testing_dataloader_) * inputs.size(0)
					inputs, labels = data
					inputs = inputs.to(self.__device)
					outputs = self.__classifier(inputs)
					predicted = outputs.argmax(dim=1)
					labels_long_tensor = labels.nonzero(as_tuple=True) 
					correct = (predicted == labels_long_tensor[1]).sum().item()
					corrects += correct
					total += labels.size(0)
					accuracy = 100*(predicted == labels_long_tensor[1]).sum().float()/float(labels.size(0))
					self.__testing_history["accuracy"].append(accuracy)
					writer.add_scalar('testing accuracy', accuracy, total)
			print('\t➢   Accuracy of the network on %d testing data: %d %%\n' % (total, 100 * corrects / total))
			self.__plot_testing_accuracy()



	def __get_sample(self):
		batch_index = torch.randint(len(self.training_dataloader_), (1,), device=self.__device)[0]
		for batch_ndx, sample in enumerate(self.training_dataloader_):
			if batch_ndx == batch_index:
				inputs, labels = sample # sample is a mini-batch list with two elements : inputs and labels 
				break
		return inputs, labels


	def __call__(self, data):
		
		if data[0] is not None and type(data[0]) is torch.utils.data.dataloader.DataLoader and data[1] is not None and type(data[1]) is torch.utils.data.dataloader.DataLoader:
			self.training_dataloader_ = data[0]
			self.testing_dataloader_ = data[1]
		else:
			print("[?] please specify a training/testing pytorch dataloader object for training/testing the classifier.") 
			sys.exit(1)


		if os.path.exists(self.__model_path):
			pass
		else:
			print(f"\n________found no existing pre-trained model for {self.__data_type} data ________\n")
			print(f"\t➢   training on clustered {self.__data_type} data using the classifier trainer on {self.__device}\n")
			input_features = self.__get_sample()[0].shape[1]
			output_features = self.__get_sample()[1].shape[1]
			self.__classifier = Position(input_neurons=input_features, output_neurons=output_features).to(self.__device)
			self.__tb_log = True
			self.__train()
			self.__test()
			self.__save(checkpoint=self.__checkpoint)


	def __save(self, checkpoint):
		try:
			print("\n________saving trained classifier model________\n")
			torch.save(checkpoint, self.__model_path)
			print(f"\t➢   saved classifier model info at {self.__model_path}\n")
		except IOError:
			print(f"\t➢   can't save classifier model at : {self.__model_path}\n")


	def __plot_acc(self):
		print("\n________plotting classifier model training accuracy________\n")
		fig_path = os.path.dirname(os.path.abspath(__file__))+f'/utils/pc_model_training_acc-{self.__data_type}.png'
		plt.figure()
		plt.plot(self.__history['accuracy'])
		plt.title('Model accuracy')
		plt.ylabel('accuracy')
		plt.xlabel('epoch')
		plt.legend(['train'], loc='upper left')
		plt.savefig(fig_path)
		print(f"\t➢   plots saved at {fig_path}\n")

	def __plot_loss(self):
		print("\n________plotting classifier model training loss________\n")
		fig_path = os.path.dirname(os.path.abspath(__file__))+f'/utils/pc_model_training_loss-{self.__data_type}.png'
		plt.figure()
		plt.plot(self.__history['loss'])
		plt.title('Model loss')
		plt.ylabel('loss')
		plt.xlabel('epoch')
		plt.legend(['train'], loc='upper left')
		plt.savefig(fig_path)
		print(f"\t➢   plot saved at {fig_path}\n")

	def __plot_testing_accuracy(self):
		print("\n________plotting classifier model testing accuracy________\n")
		fig_path = os.path.dirname(os.path.abspath(__file__))+f'/utils/pc_model_testing_acc-{self.__data_type}.png'
		plt.figure()
		plt.plot(self.__testing_history['accuracy'], color='m')
		plt.title('Testing accuracy')
		plt.ylabel('accuracy')
		plt.xlabel('total testing data (batch No. in each)')
		plt.legend(['test'], loc='upper left')
		plt.savefig(fig_path)
		print(f"\t➢   plots saved at {fig_path}\n")