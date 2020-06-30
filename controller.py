
# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██


 ------------------------------------------------------------------------------------------
|
|
|                              __POSITION CLUSTERING DEVELOPMENT GUIDE__
|
| 
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
| ex : (get the unscaled data)
|
|		pc_model.dataloader_.dataset.get_raw()
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


from pathlib import Path
import typer
import numpy as np
import os
import time
import sys
from server.loader import DatasetLoader
from server.streamer import DatasetStreamer
from core.position_clustering.model import trainer as position_clustering_trainer
from core.position_clustering.cluster import labels
from core.position_classification.model import trainer as position_classification_trainer




app = typer.Typer(help="|> uniXerr CLI controller <|")
labeled_csv_path = os.path.dirname(os.path.abspath(__file__)) + '/server/dataset/pc_features_labeled.csv'




@app.command()
def cluster_positions(
		 generate_fake_samples: bool = typer.Option(False, "--generate-fake-samples", help="Generating fake samples for training."),
		 epoch: int = typer.Option(3, help="Number of epoch for training VAE.", min=3, max=40),
		 batch_size: int = typer.Option(8, help="Number of batch size for training VAE.", min=4),
		 device: str = typer.Option('cpu', help="Training device. cpu or cuda"),
		 num_workers: int = typer.Option(4, help="Number of workers for pytroch dataloader object.", min=4),
		 latent_dim: int = typer.Option(2, help="Dimension of VAE latent space.", min=2, max=10), 
		 ddo: bool = typer.Option(False, "--ddo", help="Force deletion with confirmation for dataloader object."),
		 dpm: bool = typer.Option(False, "--dpm", help="Force deletion with confirmation for pre-trained VAE model."),
		 cluster_on_latent: bool = typer.Option(True, "--cluster-on-raw-data", help="Clustering on pc_features dataset, default is set to VAE latent space"),
		 cluster_method: str = typer.Option('kmeans', help="Clustering method. kmeans or hdbscan; hdbscan is not suitable for latent space of VAE and has some drawbacks for new dataset."),
		 plot_method: str = typer.Option('pca', help="Plotting method for data. pca or tsne; if you want plot data before clustering on different methods just remove the pc_dataloader.pth with --ddo option.")
	 
		 ):


	if device != 'cuda' and device != 'cpu':
		typer.echo("Please specify a correct device.")
		sys.exit(1)

	if ddo:
		delete = typer.confirm("Are you sure you want to delete dataloader object?")
		typer.echo("\t---Deleting dataloader object!\n")
		if delete:
			try:
				os.remove(os.path.dirname(os.path.abspath(__file__))+'/server/dataset/pc_dataloader.pth')
			except:
				typer.echo("\t---Errot while deleting the file\n")


	if dpm:
		delete = typer.confirm("Are you sure you want to delete pre-trained VAE model?")
		typer.echo("\t---Deleting pre-trained VAE model!\n")
		if delete:
			try:
				os.remove(os.path.dirname(os.path.abspath(__file__))+'/position_clustering/utils/pc_model.pth')
			except:
				typer.echo("\t---Errot while deleting the file\n")



	dataloader_kwargs = {'num_workers': num_workers, 'pin_memory': True} if device is 'cuda' else {}
	dataloader = DatasetLoader(
							   batch_size=batch_size, 
							   generate_fake_samples=generate_fake_samples,
							   plotting_kwargs=plot_method, 
							   dataloader_kwargs=dataloader_kwargs
							   ) # build a dataloader object if there is no one, otherwise it'll load the saved object

	

	# train vae model if there is no pre-trained one, otherwise it'll load the saved model
	pc_model = position_clustering_trainer(data=dataloader(), device=device, latent_dim=latent_dim, epoch=epoch)		
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


	if cluster_on_latent:
		typer.echo("\n________Clustering on latent space of VAE model________\n")
		cluster_ = labels(data=latent, data_type='latent', cluster_method=cluster_method)
		print("\n________latent space of VAE information________\n")

	if not cluster_on_latent:
		typer.echo("\n________Clustering on pc_features raw dataset________\n")
		cluster_ = labels(data=dataloader().dataset.get_raw(), data_type='raw', cluster_method=cluster_method)
		print("\n________pc_features raw data information during clustering________\n")

	print(f"{cluster_.dataset_info()}\n") # dataset information during clustering
	cluster_.set() # export a csv of dataset with their labels
	cluster_.plot(method=plot_method) # plot the clustered data
	cluster_sample_label = cluster_[0] # get the cluster number for 0th sample of the dataset
	print("\n________credit information________\n")
	print(f"\t---position for 0th sample of dataset is : {cluster_.get_position(cluster=cluster_sample_label)}\n")
	



@app.command()
def classify_positions(csv_path: Path = typer.Option(labeled_csv_path, help="Path to labeled pc_features csv dataset.", 
				   	   exists=True, file_okay=True, dir_okay=False, writable=False, readable=True, resolve_path=True)
				   ):
	position_classification_trainer(csv_path=csv_path) # load the pc_features_labeled.csv for classification process
	# TODO : continue with position_classification section
	# code here
	# ...
