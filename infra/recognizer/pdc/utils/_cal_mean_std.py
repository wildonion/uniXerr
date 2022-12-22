



# codes from :

# https://discuss.pytorch.org/t/computing-the-mean-and-std-of-dataset/34949/2
# https://stackoverflow.com/questions/60485362/how-to-normalize-images-in-pytorch


import torch

							# ========================
							#  SOLUTION 1
							# ========================

def CalMeanStd1(dataloader):
	mean = 0.
	std = 0.
	nb_samples = 0.
	for data in dataloader:
		batch_samples = data[0].size(0)
		data = data[0].view(batch_samples, data[0].size(1), -1)
		mean += data.mean(2).sum(0)
		std += data.std(2).sum(0)
		nb_samples += batch_samples
	mean /= nb_samples
	std /= nb_samples
	return mean, std
	

							# ========================
							#  SOLUTION 2
							# ========================

def CalMeanStd0(dataloader):
	cnt = 0
	fst_moment = torch.empty(3)
	snd_moment = torch.empty(3)
	for _, sample in enumerate(dataloader):
		images, _ = sample
		images = images/255
		b, c, h, w = images.shape
		nb_pixels = b * h * w # total pixels
		sum_ = torch.sum(images, dim=(0, 2, 3))
		sum_of_square = torch.sum(images**2, dim=(0, 2, 3))
		fst_moment = (cnt * fst_moment + sum_) / (cnt + nb_pixels)
		snd_moment = (cnt * snd_moment + sum_of_square) / (cnt + nb_pixels)
		cnt += nb_pixels
	mean = fst_moment
	std  = torch.sqrt(snd_moment - fst_moment ** 2) 
	return mean, std