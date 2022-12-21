


# coding: utf-8

'''
	Codded By : 
 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██
-------------------------------------------------------------------------------------------------
| 				Persian Database Classification using MLP and CNN
|-------------------------------------------------------------------------------------------------
|
|
|
|
| USAGE : _______training_______ 
|			python trainer.py --network mlp --batch-size 32 --num-workers 4 --epochs 200 --learning-rate 0.001 --device cpu
|
|
|
|


'''



import numpy as np
import operator
import time
import os, sys
import argparse
import matplotlib.pyplot as plt
import torch
from torch.utils.data import DataLoader
from torchvision import transforms
import torch.optim as optim
from dataset import PersianAlphabetDataset
from utils import ToTensor, Normalize, UnNormalize, CalMeanStd0, TrainEvalCNN, TrainEvalMLP, PlotStat, GetSample
from model import CNN, MLP




# ------------ argument options
# ------------------------------
parser = argparse.ArgumentParser(description='Persian Database Classification Trainer')
parser.add_argument('--network', action='store', type=str, help='mlp or cnn', required=True)
parser.add_argument('--batch-size', action='store', type=int, help='The number of batch size', required=True)
parser.add_argument('--num-workers', action='store', type=int, help='The number of workers for dataloader object', required=True)
parser.add_argument('--epochs', action='store', type=int, help='The number of epochs', required=True)
parser.add_argument('--learning-rate', action='store', type=float, help='Learning rate value', required=True)
parser.add_argument('--device', action='store', type=str, help='The device to attach the torch model to', required=True)
args = parser.parse_args()





# ------------ setup device and optimizer
# ----------------------------------------------
cuda = torch.cuda.is_available() if args.device == 'cuda' else None
device = torch.device("cuda" if cuda else "cpu")
torch.backends.cudnn.benchmark = True
optimizer = None





# ------------ check path for pre-trained models
# --------------------------------------------------------
if os.path.exists('utils/cnn.pth') or os.path.exists('utils/mlp.pth'):
	print("\t✅ found existing pre-trained models, start bot.py server for prediction\n")
	sys.exit(1)






# ------------ start training and evaluating on training and testing data respectively
# ----------------------------------------------------------------------------------------------
else:
	print(f"\n‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗found no existing pre-trained model, start training‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗\n")




	# 					   ------------------------------------------
	# --------------------- calculating mean and std of training data
	# 					   ------------------------------------------
	# 
	print(f"\t✅ calculating mean and std of training data for normalization\n")
	# we do not have to pass the ToTensor an Normalize transforms to 
	# PersianAlphabetDataset cause Dataloader will turn images of dataset
	# into tensor and since we're calculating the mean and std there is no
	# need to pass Normalize in here!
	# -----------------------------------------------------------------------
	cal_mean_std_iter = DataLoader(PersianAlphabetDataset(csv_files=['dataset/train_x.csv', 'dataset/train_y.csv']), batch_size=args.batch_size)
	mean, std = CalMeanStd0(cal_mean_std_iter) # you have to pass a dataloader object





	# 					   ------------------
	# --------------------- building dataset
	# 					   ------------------
	# 
	print(f"\t✅ building dataset pipeline from CSV files\n")
	# normalize image using calculated mean and std per channel
	# generally mean and std is a list of per channel values
	# in our case one value for std and mean cause we have one channel
	# --------------------------------------------------------------------
	transform = transforms.Compose([ToTensor(), Normalize(mean=mean, std=std)])
	training_transformed = PersianAlphabetDataset(csv_files=['dataset/train_x.csv', 'dataset/train_y.csv'], transform=transform)
	valid_transformed  = PersianAlphabetDataset(csv_files=['dataset/test_x.csv', 'dataset/test_y.csv'], transform=transform)




	# 						---------------------------
	# --------------------- building dataloader objects
	# 						---------------------------
	# 
	print(f"\t✅ building dataloader objects from training and valid data pipelines\n")
	# -----------------------------------------------------	
	train_iter = DataLoader(training_transformed, batch_size=args.batch_size, shuffle=True, num_workers=args.num_workers)
	valid_iter  = DataLoader(valid_transformed, batch_size=args.batch_size, shuffle=True, num_workers=args.num_workers)

	

	
	# 						-----------------
	# --------------------- plotting a sample
	#						-----------------
	# 
	print(f"\t✅ plotting a sample from training dataloader object\n")	
	# if we have 50 pack of data with 20 data in each pack 
	# then we have 1000 total data and a mini-batch 
	# is one iteration through that 50 pack inside the dataloader
	# ----------------------------------------------------------------
	mini_batch = GetSample(train_iter, device)
	mini_batch_inputs = mini_batch[0]
	mini_batch_labels = mini_batch[1]
	plt.figure()
	plt.imshow(mini_batch_inputs[0].permute(1, 2, 0).numpy()) # plot the first image of the mini-batch with args.batch_size images in it as a numpy image - (H, W , C)
	plt.show()


	

	# 						--------------
	# --------------------- building model
	# 						--------------
	# 
	print(f"\t✅ building {args.network} model\n")
	# ---------------------------------------------
	if args.network == "mlp":
		net = MLP(input_neurons=mini_batch_inputs.shape[2]**2, output_neurons=mini_batch_labels.shape[1], learning_rate=args.learning_rate)
	elif args.network == "cnn":
		net = CNN(input_channels=mini_batch_inputs.shape[1], output_neurons=mini_batch_labels.shape[1])
		optimizer = optim.Adam(net.parameters(), args.learning_rate)
	else:
		print("[❌] Network Not Supported!")
		sys.exit(1)




	# 						-------------------------------
	# --------------------- training and evaluating process
	# 						-------------------------------
	# 
	print(f"\t✅ start training and evaluating process\n")
	# -----------------------------------------------------------
	valid_loss_min = np.Inf
	criterion = torch.nn.CrossEntropyLoss()
	start_time = time.time()
	history = {"train_loss": [], "valid_loss": [], "train_acc": [], "valid_acc": []}


	for e in range(args.epochs):
		loggings = TrainEvalCNN(net.to(device), device, e, train_iter, valid_iter, optimizer=optimizer, criterion=criterion) if optimizer else TrainEvalMLP(net.to(device), device, e, train_iter, valid_iter, criterion=criterion)
		train_loss, train_acc, valid_loss, valid_acc = loggings[0], loggings[1], loggings[2], loggings[3]
		history["train_loss"].append(train_loss)
		history["train_acc"].append(train_acc)
		history["valid_loss"].append(valid_loss)
		history["valid_acc"].append(valid_acc)
		if valid_loss <= valid_loss_min:
			print(f'\t⚠️ validation loss decreased ({valid_loss_min:.6f} ☛ {val_loss:.6f})')
			print(f'\t📸 model snapshot saved')
			torch.save(net.state_dict(), f'utils/{args.network}.pth')
			valid_loss_min = val_loss

	
	end_time = time.time()
	total_time = end_time - start_time
	index_train_acc, value_train_acc = max(enumerate(history["train_acc"]), key=operator.itemgetter(1))
	index_val_acc, value_val_acc = max(enumerate(history["valid_acc"]), key=operator.itemgetter(1))
	print("➲ Best training accuracy was {} at epoch {}".format(value_train_acc, index_train_acc+1))
	print("➲ Best valid accuracy was {} at epoch {}".format(value_val_acc, index_val_acc+1))




	# 						----------------------------
	# --------------------- plotting statistical results
	# 						----------------------------
	# 
	print(f"\t⌛ finished training and evaluating process in {total_time} ms\n")
	print(f"\t📊 plotting history\n")
	# -----------------------------------------------------------
	PlotStat(history)
