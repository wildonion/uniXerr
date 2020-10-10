
# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██


'''


from pathlib import Path
import typer
import numpy as np
import pandas as pd
import os
import time
import sys
from server.loader import ClusteringDatasetLoader, ClassificationDatasetLoader
from core.position_clustering.model import trainer as position_clustering_trainer
from core.position_clustering.cluster import labels
from core.position_classification.model import trainer as position_classification_trainer
from core.position_classification.classifier import predictor




app = typer.Typer(help="【  uniXerr CLI controller  】")



# TODO : send a csv file for input data prediction and the type of labeled data for loading/training classifier model from uPC telegram bot
data_type = "raw"
labeled_csv_path = os.path.dirname(os.path.abspath(__file__)) + f'/server/dataset/pc_features_labeled-{data_type}.csv'
csv_input_data_for_classification = os.path.dirname(os.path.abspath(__file__))+'/core/position_classification/utils/input_data.csv'




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
		typer.secho("Please specify a correct device.", fg=typer.colors.RED, bold=True)
		sys.exit(1)

	if ddo:
		delete = typer.confirm("Are you sure you want to delete dataloader object?")
		if delete:
			typer.secho("\t➢   Deleting dataloader object!\n", fg=typer.colors.YELLOW, bold=True)
			try:
				os.remove(os.path.dirname(os.path.abspath(__file__))+'/server/dataset/pc_dataloader.pth')
			except:
				typer.secho("\t➢   Errot while deleting the file\n", fg=typer.colors.RED, bold=True)


	if dpm:
		delete = typer.confirm("Are you sure you want to delete pre-trained VAE model?")
		if delete:
			typer.secho("\t➢   Deleting pre-trained VAE model!\n", fg=typer.colors.YELLOW, bold=True)
			try:
				os.remove(os.path.dirname(os.path.abspath(__file__))+'/core/position_clustering/utils/pc_model_vae.pth')
			except:
				typer.secho("\t➢   Errot while deleting the file\n", fg=typer.colors.RED, bold=True)



	dataloader_kwargs = {'num_workers': num_workers, 'pin_memory': True} if device is 'cuda' else {}
	dataloader = ClusteringDatasetLoader(
							   batch_size=batch_size, 
							   generate_fake_samples=generate_fake_samples,
							   plotting_kwargs=plot_method, 
							   dataloader_kwargs=dataloader_kwargs
							   ) # build a dataloader object if there is no one, otherwise it'll load the saved object

	

	# train vae model if there is no pre-trained one, otherwise it'll load the saved model
	pc_model = position_clustering_trainer(data=dataloader(), device=device, latent_dim=latent_dim, epoch=epoch)		
	latent = pc_model(data=dataloader().dataset.data) # get the latent space of dataset



	typer.secho("\n________VAE model state dict________\n", fg=typer.colors.MAGENTA, bold=True)
	for param_tensor in pc_model.vae_model.state_dict():
		print("\t➢  ",param_tensor, "\t\t", pc_model.vae_model.state_dict()[param_tensor].size())
	typer.secho(f"\n________the model________\n", fg=typer.colors.MAGENTA, bold=True)
	typer.secho(f"{pc_model.vae_model}", fg=typer.colors.RESET, bold=True)
	
	# print("\n________VAE model optimizer state dict________\n")
	# for var_name in pc_model.optimizer.state_dict():
	# 	print(var_name, "\t", pc_model.optimizer.state_dict()[var_name])
	typer.secho(f"\n________the optimizer________\n", fg=typer.colors.MAGENTA, bold=True)
	typer.secho(f"{pc_model.optimizer}", fg=typer.colors.RESET, bold=True)

	typer.secho("\n________VAE model last epoch saved________\n", fg=typer.colors.MAGENTA, bold=True)
	typer.secho(f"\t➢   current check point epoch : {pc_model.epoch}", fg=typer.colors.RESET, bold=True)

	typer.secho("\n________VAE model last training loss saved________\n", fg=typer.colors.MAGENTA, bold=True)
	typer.secho("\t➢   current check point loss : {:.6f}".format(pc_model.loss), fg=typer.colors.RESET, bold=True)
	pc_model.plot_loss() # plot training loss

	

	typer.secho("\n________testing VAE model________\n", fg=typer.colors.MAGENTA, bold=True)
	sample_zero = dataloader().dataset.data[0]
	sample_zero_latent = pc_model(sample_zero)
	sample_zero_recons_decode_m = pc_model.decode(sample_zero_latent).data.numpy()
	sample_zero_recons_recons_m, mu, log_variance = pc_model.recons(sample_zero)
	typer.secho(f"\t➢   sample 0 of dataset : {sample_zero}", fg=typer.colors.RESET, bold=True)
	typer.secho(f"\t➢   getting the latent space of sample 0 : {sample_zero_latent}", fg=typer.colors.RESET, bold=True)
	typer.secho(f"\t➢   reconstructing the sample 0 from latent space using decode method : {sample_zero_recons_decode_m}", fg=typer.colors.RESET, bold=True)
	typer.secho(f"\t➢   reconstructing the sample 0 from latent space using recons method : {sample_zero_recons_recons_m.data.numpy()}", fg=typer.colors.RESET, bold=True)
	typer.secho(f"\t➢   mu : {mu.data.numpy()}", fg=typer.colors.RESET, bold=True) # mu is equals to the latent space cause we are not in training mode, in this case reparam method return mu
	typer.secho(f"\t➢   log variance : {log_variance.data.numpy()}", fg=typer.colors.RESET, bold=True)


	if cluster_on_latent:
		typer.secho("\n________Clustering on latent space of VAE model________\n", fg=typer.colors.MAGENTA, bold=True)
		cluster_ = labels(data=latent, data_type='latent', cluster_method=cluster_method)
		typer.secho("\n________latent space of VAE information________\n", fg=typer.colors.MAGENTA, bold=True)

	if not cluster_on_latent:
		typer.secho("\n________Clustering on pc_features raw dataset________\n", fg=typer.colors.MAGENTA, bold=True)
		cluster_ = labels(data=dataloader().dataset.get_raw(), data_type='raw', cluster_method=cluster_method)
		typer.secho("\n________pc_features raw data information during clustering________\n", fg=typer.colors.MAGENTA, bold=True)

	typer.secho(f"{cluster_.dataset_info()}\n", fg=typer.colors.RESET, bold=True) # dataset information during clustering
	cluster_.set() # export a csv of dataset with their labels
	cluster_.plot(method=plot_method) # plot the clustered data
	cluster_sample_label = cluster_[0] # get the cluster number for 0th sample of the dataset
	typer.secho("\n________credit information________\n", fg=typer.colors.MAGENTA, bold=True)
	typer.secho(f"\t➢   position for 0th sample of dataset is : {cluster_.get_position(cluster=cluster_sample_label)}\n", fg=typer.colors.RESET, bold=True)
	



@app.command()
def classify_positions(csv_path: Path = typer.Option(labeled_csv_path, help="Path to labeled pc_features csv dataset.", 
				   	   exists=True, file_okay=True, dir_okay=False, writable=False, readable=True, resolve_path=True),
					   input_data_csv_path: Path = typer.Option(csv_input_data_for_classification, help="Path to input data csv for classification.", 
				   	   exists=True, file_okay=True, dir_okay=False, writable=False, readable=True, resolve_path=True),
					   ddo: bool = typer.Option(False, "--ddo", help="Force deletion with confirmation for dataloader objects."),
					   dpm: bool = typer.Option(False, "--dpm", help="Force deletion with confirmation for pre-trained classifier model."),
					   epoch: int = typer.Option(200, help="Number of epoch for training classifier.", min=100, max=300),
					   batch_size: int = typer.Option(64, help="Number of batch size for training classifier.", min=16, max=256),
					   device: str = typer.Option('cpu', help="Training device. cpu or cuda"),
					   num_workers: int = typer.Option(4, help="Number of workers for pytroch dataloader object.", min=4),
				   ):

	
	if device != 'cuda' and device != 'cpu':
		typer.secho("Please specify a correct device.", fg=typer.colors.RED, bold=True)
		sys.exit(1)


	if ddo:
		delete = typer.confirm("Are you sure you want to delete dataloader objects?")
		if delete:
			typer.secho("\t➢   Deleting dataloader objects!\n", fg=typer.colors.YELLOW, bold=True)
			try:
				os.remove(os.path.dirname(os.path.abspath(__file__))+f'/server/dataset/pc_features_labeled_testing_tensors-{data_type}-DATALOADER.pth')
				os.remove(os.path.dirname(os.path.abspath(__file__))+f'/server/dataset/pc_features_labeled_training_tensors-{data_type}-DATALOADER.pth')
			except:
				typer.secho("\t➢   Errot while deleting the file\n", fg=typer.colors.RED, bold=True)

	if dpm:
		delete = typer.confirm("Are you sure you want to delete pre-trained classifier model?")
		if delete:
			typer.secho("\t➢   Deleting pre-trained classifier model!\n", fg=typer.colors.YELLOW, bold=True)
			try:
				os.remove(os.path.dirname(os.path.abspath(__file__))+f'/core/position_classification/utils/pc_model_classifier-{data_type}.pth')
			except:
				typer.secho("\t➢   Errot while deleting the file\n", fg=typer.colors.RED, bold=True)


	dataloader_kwargs = {'num_workers': num_workers, 'pin_memory': True} if device is 'cuda' else {}
	# build a dataloader objects for training and testing data if there is no one, otherwise it'll load the saved objects
	dataloader = ClassificationDatasetLoader(csv_path=csv_path, batch_size=batch_size, dataloader_kwargs=dataloader_kwargs)
	pc_model = position_classification_trainer(device=device, epoch=epoch, data_type=data_type) # train and test classifier model if there is no pre-trained one
	pc_model(data=dataloader()) # dataloader()[0] is training pipeline and dataloader()[1] is testing pipeline
	classifier_ = predictor(device=device, data_type=data_type) # it'll load the saved model and classify input data using the pre-trained one
	

	typer.secho("\n________classifier model state dict________\n", fg=typer.colors.MAGENTA, bold=True)
	for param_tensor in classifier_.model.state_dict():
		print("\t➢  ",param_tensor, "\t\t", classifier_.model.state_dict()[param_tensor].size())
	typer.secho(f"\n________the model________\n", fg=typer.colors.MAGENTA, bold=True)
	typer.secho(f"{classifier_.model}", fg=typer.colors.RESET, bold=True)
	
	# print("\n________classifier model optimizer state dict________\n")
	# for var_name in classifier_.optimizer.state_dict():
	# 	print(var_name, "\t", classifier_.optimizer.state_dict()[var_name])
	typer.secho(f"\n________the optimizer________\n", fg=typer.colors.MAGENTA, bold=True)
	typer.secho(f"{classifier_.optimizer}", fg=typer.colors.RESET, bold=True)

	typer.secho("\n________classifier model last epoch saved________\n", fg=typer.colors.MAGENTA, bold=True)
	typer.secho(f"\t➢   current check point epoch : {classifier_.epoch}", fg=typer.colors.RESET, bold=True)

	typer.secho("\n________classifier model last training loss saved________\n", fg=typer.colors.MAGENTA, bold=True)
	typer.secho("\t➢   current check point training loss : {:.6f}".format(classifier_.training_loss), fg=typer.colors.RESET, bold=True)

	typer.secho("\n________classifier model best training accuracy saved________\n", fg=typer.colors.MAGENTA, bold=True)
	typer.secho("\t➢   current check point training best accuracy : {:.6f}".format(classifier_.training_best_accuracy), fg=typer.colors.RESET, bold=True)
	


	# classify the input data using pre-trained classifier model
	# input data can be either a valid csv_path or a numpyndarray (online training only)
	# contains rollcall_score, class_activity, discipline and total_quizzes_avg as features
	classifier_(input_data_csv_path)