
# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██

'''

import numpy as np
import os
import zmq
import time
import sys
from core.server.streamer.pc.loader import DatasetLoader
from core.server.streamer.pc.streamer import DatasetStreamer
from core.kernel.position_clustering.model import trainer
from core.kernel.position_clustering.cluster import labels


if __name__ == "__main__":

	# context = zmq.Context()
	# socket = context.socket(zmq.REP)
	# socket.bind("tcp://*:5555")

	# while True:
	# 	message = socket.recv()
	# 	print(f"Received request: {message}")

	# 	# parse args from zmq eye using protocol buffers
	# 	# ...

	# 	time.sleep(1)

	# 	socket.send(b"World")


	'''
	 --------------------------------------------------------------------------------------------
	| 
	| 
	|
	'''


	training = 'offline' # online, pre - default offline
	latent_dim = 2 # use different dims for latent space like 10 - default is 2
	epoch = 3
	generate_fake_samples = False # default False
	cluster_method = 'kmeans' # hdbscan is not suitable for latent space of VAE and has some drawbacks for new dataset - default kmeans
	device = 'cpu' # cuda - default cpu
	num_workers = 4
	dataloader_kwargs = {'num_workers': num_workers, 'pin_memory': True} if device is 'cuda' else {}	
	batch_size = 8
	plot_method = 'pca' # tsne - default pca , if you want plot data before clustering on different methods just remove the pc_dataloader.pth

	'''				   
        |
	|
	 --------------------------------------------------------------------------------------------
	'''



	if training is 'offline':
		'''
			 ------------------------------------------------------------------------------------------
			| [?] accessing dataset pipeline methods and data members
			| 	  like plotting data and fetching a sample, is done
			| 	  through the pc_model.dataloader_.dataset object
			|
			|
			| ex : (plotting data before clustering) 
			|		pc_model.dataloader_.dataset.plot_data_()
			|     
			|			args :
			| 				plotting_method=plot_method
			|
			| ex : (get a sample from the dataset)
			|		pc_model.dataloader_.dataset[2]
			|
			|	
			| ex : (get the scaled data)
			|		pc_model.dataloader_.dataset.data
			|
			|
			| ex : (dataset information)
			|		pc_model.dataloader_.dataset.__repr__()
			|
			|
			| ex : (pre-trained model object)
			|		pc_mode.vae_model
			|
			|
			| ex : (plot the whole training loss in an entire epoch)
			|		pc_model.plot_loss()
			|
			|
			| ex : (get latent space of the data)
			|		pc_model(data=data=dataloader().dataset.data)
			| 
			|
			| ex : (decode the latent space to reconstruct the input point)
			|		pc_model.decode(latent=latent) 
			|
			|
			|
			| [?] if cluster_method is hdbscan to access 
			| 	  sample label and its score do like this :
			|
			| 		cluster_sample_label = cluster_[45][0]
			| 		cluster_sample_label_score = cluster_[45][1]
			|
			|
			| [?] cluster_.set() will export a csv of dataset with their labels
			|
			| [?] cluster_.plot() plot the clustered data with a specified method and clustering algo  
			 ------------------------------------------------------------------------------------------
		'''

		dataloader = DatasetLoader(
								   batch_size=batch_size, 
								   generate_fake_samples=generate_fake_samples,
								   plotting_kwargs=plot_method, 
								   dataloader_kwargs=dataloader_kwargs
								   ) # build a dataloader object if there is no one, otherwise it'll load the saved object

		
		# train vae model if there is no pre-trained one
		# otherwise it'll load the saved model
		pc_model = trainer(data=dataloader(), device=device, latent_dim=latent_dim, epoch=epoch, training=training)		
		latent = pc_model(data=dataloader().dataset.data) # get the latent space of dataset




		print("\n________VAE model state dict________\n")
		for param_tensor in pc_model.vae_model.state_dict():
			print("\t---",param_tensor, "\t\t", pc_model.vae_model.state_dict()[param_tensor].size())
		print(f"\n________the model________\n{pc_model.vae_model}")
		
		# print("\n________VAE model optimizer state dict________\n")
		# for var_name in pc_model.optimizer.state_dict():
		# 	print(var_name, "\t", pc_model.optimizer.state_dict()[var_name])
		print(f"\n________the optimizer________\n{pc_model.optimizer}")

		print("\n________VAE model last epoch saved________\n")
		print(f"\t---current check point epoch : {pc_model.epoch}")

		print("\n________VAE model last training loss saved________\n")
		print("\t---current check point loss : {:.6f}".format(pc_model.loss))
		pc_model.plot_loss() # plot training loss

		
		print("\n________testing VAE model________\n")
		sample_zero = dataloader().dataset.data[0]
		sample_zero_latent = pc_model(sample_zero)
		sample_zero_recons_decode_m = pc_model.decode(sample_zero_latent).data.numpy()
		sample_zero_recons_recons_m, mu, log_variance = pc_model.recons(sample_zero)
		print(f"\t---sample 0 of dataset : {sample_zero}")
		print(f"\t---getting the latent space of sample 0 : {sample_zero_latent}")
		print(f"\t---reconstructing the sample 0 from latent space using decode method : {sample_zero_recons_decode_m}")
		print(f"\t---reconstructing the sample 0 from latent space using recons method : {sample_zero_recons_recons_m.data.numpy()}")
		print(f"\t---mu : {mu.data.numpy()}") # mu is equals to the latent space cause we are not in training mode, in this case reparam method return mu
		print(f"\t---log variance : {log_variance.data.numpy()}")



		cluster_ = labels(data=latent, cluster_method=cluster_method)
		cluster_.set() # export a csv of dataset with their labels
		cluster_.plot(method=plot_method) # plot the clustered data
		print("\n________latent space of VAE information________\n")
		print(f"{cluster_.dataset_info()}\n") # dataset information for clustering
		cluster_sample_label = cluster_[0] # get the cluster number for 0th sample of the dataset
		print("\n________credit information________\n")
		print(f"\t---position for 0th sample of dataset is : {cluster_.get_position(cluster=cluster_sample_label)}\n")



		# 			for hdbscan method
		# cluster_sample_label = cluster_[23][0]
		# cluster_sample_label_score = cluster_[23][1]
		# print(f"\t---position for 23th sample of dataset and its score is : \
		# 		{cluster_.get_position(cluster=cluster_sample_label)} - ,\
		# 		{cluster_sample_label_score}\n")





	elif training is 'online':
		# TODO : 
		# 		online training using real time data streaming over kafka
		# datastreamer = DatasetStreamer(plotting_kwargs=plot_method)
		# datastreamer()
		# pc_model = model(data=None, device=device, training=training)
		raise NotImplementedError



	
