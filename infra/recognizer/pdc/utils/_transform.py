

# codes for Normalize and UnNormalize :
# https://discuss.pytorch.org/t/simple-way-to-inverse-transform-normalization/4821/3

import torch



class ToTensor:
	def __call__(self, image):
		# image = image.transpose((2, 0, 1)) # torch image: C X H X W
		return torch.from_numpy(image)
			   

class UnNormalize:
	def __init__(self, mean, std):
		self.mean = mean
		self.std = std

	def __call__(self, image):
		for t, m, s in zip(image, self.mean, self.std):
			t.mul_(s).add_(m) # image = ((image * std) + mean)
			t = t.float().mul(255)
		return image


class Normalize: # standardize
	def __init__(self, mean, std):
		self.mean = mean
		self.std = std

	def __call__(self, image): # image size : (C, H, W)
		for t, m, s in zip(image, self.mean, self.std):
			t = t.float().div(255)
			t.sub_(m).div_(s) # image = (image - mean) / std
		return image
