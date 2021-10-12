

import operator
import torch
from torch.autograd import Variable
import matplotlib.pyplot as plt
plt.style.use('ggplot')



# NOTE - 200 iterations through the dataloader means iterate through 200 minibatch samples with 64 batch of input data in each



def TrainEvalMLP(model, device, e, train_iter, valid_iter, criterion):
	# =============
	# training loop
	# =============
	running_train_loss = 0.
	for idx, sample in enumerate(train_iter): # len(train_iter) iterations
		images, labels = sample
		images = Variable(images.float().to(device))
		images = images.view(images.size(0), -1) # flatten the image
		output = model(images) # feeding images to the model
		winners = output.argmax(dim=1) # calculate the most prob of predictions, return images.size(0) indices of most prob chars in each row
		labels_long_tensor = labels.nonzero(as_tuple=True) # all nonezero values with their coressponding indices
		corrects = (winners == labels_long_tensor[1]) # list of corrects - labels_long_tensor[0] is batch indices and labels_long_tensor[1] is indices of nonzero values
		train_acc = 100*corrects.sum().float()/float(labels.size(0)) # the ratio of number of correct predictions to the total number of input samples
		train_loss = criterion(output, labels.argmax(dim=1)) # calculate the loss between output and labels
		running_train_loss += train_loss.item()
		# *************************
		model.train(images, labels) # run backpropagation algorithm to tune the weights at the end of the iteration - minibatch gradient
		# *************************
		if idx % 20 == 0: # log every 20 mini-batch
			print("[TRAINING LOG]")
			print('\t☕️ [epoch ⇀  %d, sample/mini-batch ⇀  %d, batch size ⇀  %d] \n\t\t ↳  loss: %.3f - acc: %.3f' % (e + 1, idx + 1, images.size(0), running_train_loss/20, train_acc))
			running_train_loss = 0.
	# ===============
	# validating loop
	# ===============
	running_valid_loss = 0.
	for idx, sample in enumerate(valid_iter): # len(valid_iter) iterations
		images, labels = sample
		images = Variable(images.float().to(device))
		images = images.view(images.size(0), -1) # flatten the image
		output = model(images) # feeding images to the model
		winners = output.argmax(dim=1) # calculate the most prob of predictions, return images.size(0) indices of most prob chars in each row
		labels_long_tensor = labels.nonzero(as_tuple=True) # all nonezero values with their coressponding indices
		corrects = (winners == labels_long_tensor[1]) # list of corrects - labels_long_tensor[0] is batch indices and labels_long_tensor[1] is indices of nonzero values
		valid_acc = 100*corrects.sum().float()/float(labels.size(0)) # the ratio of number of correct predictions to the total number of input samples
		valid_loss = criterion(output, labels.argmax(dim=1)) # calculate the loss between output and labels
		running_valid_loss += valid_loss.item()
		if idx % 20 == 0: # log every 20 mini-batch
			print("\n[VALIDATING LOG]")
			print('\t☕️ [epoch ⇀  %d, sample/mini-batch ⇀  %d, batch size ⇀  %d] \n\t\t ↳  loss: %.3f - acc: %.3f' % (e + 1, idx + 1, images.size(0), running_valid_loss/20, valid_acc))
			running_valid_loss = 0.
	# NOTE : to return total loss and acc of all iterations in an epoch you have to sum all loss and accuracy values 
	# 	 	at the end of each iteration then divide each of them by the length of the loader
	# return the last loss and last acc of both loaders at the end of iteration in an epoch
	return train_loss, train_acc, valid_loss, valid_acc




def TrainEvalCNN(model, device, e, train_iter, valid_iter, optimizer, criterion):
	# =============
	# training loop
	# =============
	model.train()
	running_train_loss = 0.
	for idx, sample in enumerate(train_iter): # len(train_iter) iterations
		images, labels = sample
		images = Variable(images.float().to(device))
		optimizer.zero_grad() # set all previous calculated gradients to zero
		output = model(images) # feeding images to the model
		winners = output.argmax(dim=1) # calculate the most prob of predictions, return images.size(0) indices of most prob chars in each row
		labels_long_tensor = labels.nonzero(as_tuple=True) # all nonezero values with their coressponding indices
		corrects = (winners == labels_long_tensor[1]) # list of corrects - labels_long_tensor[0] is batch indices and labels_long_tensor[1] is indices of nonzero values
		train_acc = 100*corrects.sum().float()/float(labels.size(0)) # the ratio of number of correct predictions to the total number of input samples
		train_loss = criterion(output, labels.argmax(dim=1)) # calculate the loss between output and labels
		train_loss.backward() # calculate the gradient using computational graph for all weights
		optimizer.step() # update weights and other parameters like biases
		running_train_loss += train_loss.item()
		if idx % 20 == 0: # log every 20 mini-batch
			print("[TRAINING LOG]")
			print('\t☕️ [epoch ⇀  %d, sample/mini-batch ⇀  %d, batch size ⇀  %d] \n\t\t ↳  loss: %.3f - acc: %.3f' % (e + 1, idx + 1, images.size(0), running_train_loss/20, train_acc))
			running_train_loss = 0.
	# ===============
	# validating loop
	# ===============
	model.eval()
	running_valid_loss = 0.
	for idx, sample in enumerate(valid_iter): # len(valid_iter) iterations
		images, labels = sample
		images = Variable(images.float().to(device))
		output = model(images) # feeding images to the model
		winners = output.argmax(dim=1) # calculate the most prob of predictions, return images.size(0) indices of most prob chars in each row
		labels_long_tensor = labels.nonzero(as_tuple=True) # all nonezero values with their coressponding indices
		corrects = (winners == labels_long_tensor[1]) # list of corrects - labels_long_tensor[0] is batch indices and labels_long_tensor[1] is indices of nonzero values
		valid_acc = 100*corrects.sum().float()/float(labels.size(0)) # the ratio of number of correct predictions to the total number of input samples
		valid_loss = criterion(output, labels.argmax(dim=1)) # calculate the loss between output and labels
		running_valid_loss += valid_loss.item()
		if idx % 20 == 0: # log every 20 mini-batch
			print("[VALIDATING LOG]")
			print('\t☕️ [epoch ⇀  %d, sample/mini-batch ⇀  %d, batch size ⇀  %d] \n\t\t ↳  loss: %.3f - acc: %.3f' % (e + 1, idx + 1, images.size(0), running_valid_loss/20, valid_acc))
			running_valid_loss = 0.
	# NOTE : to return total loss and acc of all iterations in an epoch you have to sum all loss and accuracy values 
	# 	 	at the end of each iteration then divide each of them by the length of the loader
	# return the last loss and last acc of both loaders at the end of iteration in an epoch
	return train_loss, train_acc, valid_loss, valid_acc



def PlotStat(history):
		acc_fig_path = os.path.dirname(os.path.abspath(__file__))+f'/utils/acc.png'
		loss_fig_path = os.path.dirname(os.path.abspath(__file__))+f'/utils/loss.png'
		
		plt.figure()
		plt.plot(history['train_acc'], label="train")
		plt.plot(history['valid_acc'], label="valid")
		plt.title('Model accuracy')
		plt.ylabel('accuracy')
		plt.xlabel('epoch')
		plt.legend(['train', 'valid'])
		plt.savefig(acc_fig_path)
		print(f"\t➢   plots saved at {acc_fig_path}\n")

		plt.figure()
		plt.plot(history['train_loss'], label="train")
		plt.plot(history['valid_loss'], label="valid")
		plt.title('Model loss')
		plt.ylabel('accuracy')
		plt.xlabel('loss')
		plt.legend(['train', 'valid'])
		plt.savefig(loss_fig_path)
		print(f"\t➢   plots saved at {loss_fig_path}\n")



def GetSample(dataloader, device):
	batch_index = torch.randint(len(dataloader), (1,), device=device)[0]
	for batch_ndx, sample in enumerate(dataloader): # total training data = len(dataloader) * inputs.size(0)
		if batch_ndx == batch_index:
			return sample # sample is a mini-batch (a pack of batch No. data) list with two elements : inputs and labels - cause dataloader object split the dataset into small batches
