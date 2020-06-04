



# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██



 --------------------------------------------
|        Variational Autoencoder model
| -------------------------------------------
| dimensionality reduction process
| using variational autoencoder
| to cluster the latent space.
|
|

'''


import numpy as np
import os
import sys
import pandas as pd
import matplotlib.pyplot as plt
import torch
from torch import nn
from torch.autograd import Variable


class VAE(nn.Module):
	def __init__(self, pc_features, latent_dim):
		'''
		reference : 
			https://towardsdatascience.com/reparameterization-trick-126062cfd3c3 
			https://www.jeremyjordan.me/variational-autoencoders/
			https://chrisorm.github.io/VAE-pyt.html


			[?] in probability theory and statistics, the multivariate normal distribution, 
				multivariate Gaussian distribution, or joint normal distribution is a 
				generalization of the one-dimensional normal distribution to higher dimensions.
		
							mu = 𝜇 = mean vector 
							std = 𝜎 = standard deviation vector
							z = latent space vector of batch of sample
							ε ~ normal distribution(0,1) - (gaussian) 
							
							| |\'                       /| |
							| | \'                     / | |
							| |  \'                   /  | |
							|x|   | std + mu * ε = z |   |x̂|
							| |  /                   \'  | |
							| | /                     \' | |
							| |/                       \'| |


			[?] each data point in a VAE would get mapped to mean and log_variance vectors 
				which would define the multivariate normal distribution around that input data point.
			
			[?] a point is sampled from this distribution and is returned as the latent variable.
			
			[?] this latent variable is fed to the decoder to produce the output.

			[?] self.en_mu is mean and self.en_std is the 
				diagonal covariance matrix = variances
				the positive square root of these variances = std

			[?] rather than directly outputting values for the latent state as we would in a standard autoencoder, 
				the encoder model of a VAE will output parameters describing a distribution for each dimension 
				in the latent space. Since we're assuming that our prior follows a normal distribution, 
				we'll output two vectors describing the mean and variance of the latent state distributions. 
				if we were to build a true multivariate Gaussian model, we'd need to define a covariance matrix 
				describing how each of the dimensions are correlated. However, we'll make a simplifying assumption 
				that our covariance matrix only has nonzero values on the diagonal, allowing us to describe this 
				information in a simple vector.
	
		'''
		super(VAE, self).__init__()
		self.pc_features = pc_features
		self.latent_dim = latent_dim

		self.en1 = nn.Linear(self.pc_features, 200)
		self.en_mu = nn.Linear(200, self.latent_dim) # mean
		self.en_std = nn.Linear(200, self.latent_dim) # log_variance or log_sigma or the diagonal covariance matrix (variances)

		self.de1 = nn.Linear(self.latent_dim, 200) # from batch_size * latent_dim to batch_size * 200
		self.de2 = nn.Linear(200, self.pc_features) # from batch_size * 200 to batch_size * pc_features
		self.relu = nn.ReLU()
		self.sigmoid = nn.Sigmoid() # we use sigmoid because the bce loss accept input in range [0, 1]

	def encode(self, x):
		'''
			return the mu and std of the latent space distribution (gaussian) for a given batch
		'''
		h1 = self.relu(self.en1(x))
		return self.en_mu(h1), self.en_std(h1)


	def decode(self, z):
		'''
			decode a batch of latent variables (space) from mu and std
		'''                        
		h2 = self.relu(self.de1(z))
		return self.sigmoid(self.de2(h2))


	def reparam(self, mu, log_variance):
		'''
			eparameterisation trick to sample z values and learn a distribution for the latent space
			we need to consider that the sampling node inside is stochastic in nature. 
			We can compute the gradients of the sampling node with respect to the 
			mean and log-variance vectors (both the mean and log-variance vectors are used in the sampling layer)

			by taking the logarithm of the variance, we force the network to have the output range 
			of the natural numbers rather than just positive values (variances would only have positive values). 
			This allows for smoother representations for the latent space.

			this essentially means sampling latent vector from a distribution defined by its mean and std.

			we randomly sample ε from a unit Gaussian, and then shift the randomly sampled ε by 
			the latent distribution's mean μ and scale it by the latent distribution's variance σ.
			with this reparameterization, we can now optimize the parameters of the distribution 
			while still maintaining the ability to randomly sample from that distribution.
		'''
		if self.training:
			'''
			    in order to deal with the fact that the network may learn negative values for σ, 
			    we'll typically have the network learn log(σ) and exponentiate this value to 
			    get the latent distribution's variance.
				
				calculate during training : std + mu * ε = z

				log_variance = log_sigma = the diagonal covariance matrix = variances ~ std = the positive square root of these variances
			        it makes sense that:
			    log_variance.exp() = sigma = std
			        mathematically it is all equivalent.

			'''
			std = log_variance.mul(0.5).exp_() # smoothing log_variance by 0.5
			eps = Variable(std.data.new(std.size()).normal_())
			return eps.mul(std).add_(mu)
		else:
			return mu


	def forward(self, x):
		'''
			encode a batch of sample then decode them to comapre

			-1 represents the number of batch.
		'''
		mu, log_variance = self.encode(x.view(-1, self.pc_features))
		z = self.reparam(mu, log_variance)
		return self.decode(z), mu, log_variance


	def loss(self, reconstruction, x, mu, log_variance):
		'''
			the parameters of a VAE are trained via two loss functions: 
			a reconstruction loss that forces the decoded samples to match the initial inputs, 
			and a regularization loss that helps learn well-formed latent spaces 
			and reduce overfitting to the training data.
			
			sigma = std = log_variance.exp()
			log_variance = log_sigma = the diagonal covariance matrix = variances ~ std = the positive square root of these variances
			KLD loss : 0.5 * sum(1 + log(sigma^2) - mu^2 - sigma^2)

			we will scale the KLD loss by same number of elements as in reconstruction : batch size * features

			because our input is binary data and we use Linear layer for that we have to use the bce loss
			as our reconstruction loss and our input must be in range [0, 1]
			so we fixed this by passing it through sigmoid activation function.
		'''
		bce = nn.BCELoss() # we choose bce as our reconstruction loss because of our input data which is binary data
		bce_loss = bce(reconstruction.double(), x.view(-1, self.pc_features).double())
		KLD = -0.5 * torch.sum(1 + log_variance - mu.pow(2) - log_variance.exp())
		KLD /= x.view(-1, self.pc_features).data.shape[0] * self.pc_features
		return bce_loss + KLD


	def get_latent_z(self, x):
		mu, log_variance = self.encode(x.view(-1, self.pc_features))
		return self.reparam(mu, log_variance)