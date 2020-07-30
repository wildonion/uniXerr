

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
| extract the latent space of data using
| variational autoencoder on both
| offline and online training method 
|
| access granted for : dataloader_ object
| save				 : save the trained model
| load               : load existing model if there is
| train              : train the model on dataloader objcet
| __call__           : get the latent space of the data
| decode             : decode the input data
| recons	     : reconstruction of input data
| 

'''


from ._vae import VAE
import numpy as np
import os
import sys
import matplotlib.pyplot as plt
import torch
from torch import nn
import torch.optim as optim
from torch.autograd import Variable


MODEL_PATH = os.path.dirname(os.path.abspath(__file__)) + '/utils/pc_model.pth' 

class trainer():

	def __init__(self, data, device, latent_dim=2, epoch=30):
		
		cuda = torch.cuda.is_available() if device is 'cuda' else None
		self.__device = torch.device("cuda" if cuda else "cpu")
		torch.backends.cudnn.benchmark = True

		if data is not None and type(data) is torch.utils.data.dataloader.DataLoader:
			self.dataloader_ = data
		else:
			print("[?] please specify a training pytorch dataloader object for training VAE model.") 
			sys.exit(1)


		self.epoch = epoch
		self.loss_tracker = []
		self.loss = 0.0

		print("\n________dataset information during extracting features using VAE________\n")
		print(f"{self.dataloader_.dataset}\n")

		if os.path.exists(MODEL_PATH):
			print("\n________found existing pre-trained VAE model________\n")
			self.__load(latent_dim=latent_dim)


		else:
			# -------------------------------------------------
			#  training Variational Autoencoder Model
			# -------------------------------------------------
			print("\n________found no existing pre-trained model________\n")
			print(f"\t---training on latent space using VAE on {self.__device}\n")


			if self.epoch > 40:
				print("[?] please specify an epoch < 40 or at most 40.")
				sys.exit(1)
			else:
				self.vae_model = VAE(pc_features=self.__show_a_sample().shape[1], latent_dim=latent_dim).to(self.__device)
				self.__train(log_interval=500)



	def __train(self, log_interval):
		self.optimizer = torch.optim.Adam(self.vae_model.parameters(), lr=1e-3)
		for e in range(self.epoch):
			self.vae_model.train()
			for i_batch, sample_batch in enumerate(self.dataloader_):
				self.optimizer.zero_grad()
				reconstructed_batch, mu, log_variance = self.vae_model(sample_batch.float().to(self.__device))
				self.loss = self.vae_model.loss(reconstructed_batch, sample_batch, mu, log_variance)
				self.loss.backward() # calculate the gradient using computational graph for all weights
				self.optimizer.step() # update weights and other parameters like biases
				if (i_batch % log_interval == 0) and (e % 10 == 0):
					print("\t\tEpoch: {} [{}/{}]\tLoss: {:.6f}"
						.format(e, i_batch, len(self.dataloader_), self.loss.data/len(sample_batch)))
			self.loss_tracker.append(self.loss.data)
			print(f"\t\t=====> Epoch: {e} done! - Batch shape: {sample_batch.shape}")

		checkpoint = {
			'model_state_dict': self.vae_model.state_dict(),
			'optimizer_state_dict': self.optimizer.state_dict(),
			'epoch': e+1,
			'loss': self.loss,
			'loss_tracker': self.loss_tracker
		}
		
		self.__save(checkpoint=checkpoint)


	def __show_a_sample(self):
		batch_index = torch.randint(len(self.dataloader_), (1,), device=self.__device)[0]
		for i_batch, sample_batch in enumerate(self.dataloader_):
			if i_batch == batch_index:
				break
		return sample_batch


	def __save(self, checkpoint):
		
		try:
			print("\n________saving trained VAE model________\n")
			torch.save(checkpoint, MODEL_PATH)
			print(f"\t---saved VAE model info at {MODEL_PATH}________\n")
		except IOError:
			print(f"\t---can't save VAE model at : {MODEL_PATH}\n")


	def __load(self, latent_dim):
		try:
			checkpoint = torch.load(MODEL_PATH)
			print(f"\t---loaded pre-trained model from {MODEL_PATH}\n")
		except IOError:
			print(f"\t---can't load pre-trained model from : {MODEL_PATH}\n")

		
		self.vae_model = VAE(pc_features=self.__show_a_sample().shape[1], latent_dim=latent_dim)
		self.vae_model.load_state_dict(checkpoint['model_state_dict'])
		
		self.optimizer = optim.Adam(self.vae_model.parameters(), lr=1e-3)
		self.optimizer.load_state_dict(checkpoint['optimizer_state_dict'])

		self.epoch = checkpoint['epoch']
		self.loss = checkpoint['loss']
		self.loss_tracker = checkpoint['loss_tracker']

		self.vae_model.eval()

	def __call__(self, data):
		'''
			data -> encode -> mu, log_variance -> reparam
			return : numpyndarray
		'''
		data = Variable(torch.from_numpy(data), requires_grad=False)
		self.vae_model.eval()
		return self.vae_model.get_latent_z(data.float()).data.numpy()

	def decode(self, latent):
		'''
			reparam -> decode
			return : pytorch tensor 
		'''
		latent = Variable(torch.from_numpy(latent), requires_grad=False)
		rp = self.vae_model.decode(latent.float())
		return rp

	def recons(self, data):
		'''
			data -> encode -> mu, log_variance -> reparam -> decode
			this method is the combination of decode and __call__ method.
			return : pytorch tensor
		'''
		self.vae_model.eval()
		data = torch.from_numpy(data)
		data = Variable(data, requires_grad=False)
		reconstructed_batch, mu, log_variance = self.vae_model(data.float().to(self.__device))
		return reconstructed_batch, mu, log_variance

	def plot_loss(self):
		print("\n________plotting VAE model training loss________\n")
		fig_path = os.path.dirname(os.path.abspath(__file__))+'/utils/pc_model_loss.png'
		plt.figure()
		plt.plot(np.array(self.loss_tracker), label='loss')
		plt.xlabel('epoch', fontsize=10)
		plt.ylabel('loss', fontsize=10)
		plt.legend()
		plt.savefig(fig_path)
		print(f"\t---plot saved at {fig_path}\n")
