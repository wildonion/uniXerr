



import torch
import torch.nn as nn
import torch.nn.functional as F


__all__ = ['MLP', 'CNN']

class MLP(nn.Module):
	def __init__(self, input_neurons, output_neurons, learning_rate):
		"""
				☕️ https://www.deeplearningwizard.com/deep_learning/boosting_models_pytorch/forwardpropagation_backpropagation_gradientdescent/

								 ⚠️ image batch size : (batch, C, H, W) ---flattened---> (batch, C*H*W) ⚠️
						    ⚠️ you can usually prevent over-fitting if you keep your number of neurons below ⚠️
		"""
		super(MLP, self).__init__()
		self.learning_rate = learning_rate

		self.fc1  = nn.Linear(input_neurons, 512) # flattened image to 512 neurons in first hidden layer - weight shape : (512 X input_neurons)
		self.fc2  = nn.Linear(512, 256) # 512 neurons in first hidden to 256 neurons in second hidden layer - weight shape : (256 X 512)
		self.fc3  = nn.Linear(256, output_neurons) # 256 neurons in second hidden to "n" neurons in output layer - weight shape : (output_neurons X 256)
		self.relu = nn.ReLU()


	def relu_prime(self, y):
		# y[y<=0] = 0
		# y[y>0] = 1
		# return y
		return torch.where(y <= 0, 0, 1)


	def forward(self, batch):
		"""
			pytorch multiply inputs by the transpose of initialized weights in order to do the multiplication process correctly
			because the initialized weights are the transpose of actual weights means their size are (output_neurons X input_neurons)
			in order to do the multiplication process correctly between inputs and weights we have to transpose
			the weights to (input_neurons X output_neurons) and pytorch do this automatically in forwarding process.


			❗️we can't apply relu on last layer(except hidden layers) with any loss functions because of its derivative nature problem:
				☕️ https://stats.stackexchange.com/questions/166595/how-to-apply-cross-entropy-on-rectified-linear-units

			❗️we shouldn't apply softmax on last layer when we're using cross entropy loss because the loss itself apply softmax on the logits:
				☕️ https://discuss.pytorch.org/t/multi-class-cross-entropy-loss-and-softmax-in-pytorch/24920/3

		"""

		self.y1 = self.fc1(batch) # y1 = batch * w1.T
		self.y2 = F.dropout(self.relu(self.y1), p=0.5, training=self.training) # y2 = relu(y1) - active only on training
		self.y3 = self.fc2(self.y2) # y3 = y2 * w2.T
		self.y4 = F.dropout(self.relu(self.y3), p=0.5, training=self.training) # y4 = relu(y3) - active only on training
		self.y5 = self.fc3(self.y4) # y5 = y4 * w3.T
		return self.y5



	def backward(self, batch, y5, actual):
		"""
								=============================
								  COMPUTING GRADIENT FOR w3
								  	   BATCH SIZE : 32
								=============================

			📌 dC/dw3 = dC/dy5 * dy5/dw3  => derivative of cross entropy loss w.r.t w3

					  ⚠️ derivative of CE loss w.r.t output passed from a softmax layer is logits - labels ⚠️ 
				
				☕️ https://www.adeveloperdiary.com/data-science/deep-learning/neural-network-with-softmax-in-python/

		"""

		self.dC_dy5   = y5 - actual # output - actual => derivative of cross entropy loss w.r.t output - size : (32 X output_neurons)
		self.dy5_dw3  = self.y4 # size : (32 X 256)
		self.dC_w3    = torch.matmul(torch.t(self.dC_dy5.float()), self.dy5_dw3.float()) # size : (32 X output_neurons).transpose * (32 X 256) = (output_neurons X 32) * (32 X 256) = (output_neurons X 256) -> w3 size

		"""
								=============================
								  COMPUTING GRADIENT FOR w2
								  	   BATCH SIZE : 32
								=============================

			📌 dC/dw2 = dC/dy5 * dy5/dy4 * dy4/dy3 * dy3/dw2 => derivative of cross entropy loss w.r.t w2

		"""

		self.dy5_dy4  = self.fc3.weight # w3 - size : (output_neurons X 256) -> this is the transpose size of actual w3 size
		self.y5_delta = torch.matmul(self.dC_dy5.float(), self.dy5_dy4.float()) # size : (32 X output_neurons) * (output_neurons X 256) = (32 X 256)
		self.dy4_dy3  = self.relu_prime(self.y4) # dy4/dy3 = relu'(y4) because relu(y3) = y4 then relu'(relu(y3)) = relu'(y4) - size : (32 X 256)
		self.dy3_dw2  = self.y2 # size : (32 X 512)
		self.y3_delta = torch.matmul(torch.t(self.dy3_dw2.float()), self.dy4_dy3.float()) # size : (32 X 512).transpose * (32 X 256) = (512 X 32) * (32 X 256) = (512 X 256)
		self.dC_dw2   = torch.matmul(self.y5_delta, torch.t(self.y3_delta)) # size : (32 X 256) * (512 X 256).transpose = (32 X 256) * (256 X 512) = (32 X 512) -> w2 size

		"""
								=============================
								  COMPUTING GRADIENT FOR w1
									   BATCH SIZE : 32
								=============================

			📌 dC/dw1 = dC/dy5 * dy5/dy4 * dy4/dy3 * dy3/dy2 * dy2/dy1 * dy1/dw1 => derivative of cross entropy loss w.r.t w1

		"""

		self.dy3_dy2  = self.fc2.weight # w2 - size : (256 X 512) -> this is the transpose size of actual w2 size
		self.dy2_dy1  = self.relu_prime(self.y2) # dy2/dy1 = relu'(y2) because relu(y1) = y2 then relu'(relu(y1)) = relu'(y2) - size : (32 X 512)
		self.dy1_dw1  = batch # size : (32 X input_neurons)
		self.y3_delta = torch.matmul(self.dy4_dy3.float(), self.dy3_dy2.float()) # size : (32 X 256) * (256 X 512) = (32 X 512) 
		self.y1_delta = torch.matmul(torch.t(self.dy1_dw1.float()), self.dy2_dy1.float()) # size : (32 X input_neurons).transpose * (32 X 512) = (input_neurons X 32) * (32 X 512) = (input_neurons X 512)
		self.dC_w1    = torch.matmul(torch.t(self.y5_delta), self.y3_delta) # (32 X 256).transpose * (32 X 512) = (256 X 32) * (32 X 512) = (256 X 512)
		self.dC_w1    = torch.matmul(self.dC_w1, torch.t(self.y1_delta)) # size : (256 X 512) * (input_neurons X 512).transpose = (32 X 512) * (512 X input_neurons) = (32 X input_neurons) -> w1 size

		"""
									=======================
										UPDATING WEIGHTS
										Δw = α * ∂Eⱼ/∂wᵢ
									=======================

			w1 = w1 - lr*dC/dw1
			w2 = w2 - lr*dC/dw2
			w3 = w3 - lr*dC/dw3

		"""
		with torch.no_grad():
			# ========= DEBUGGING =========
			print(self.dC_w1.shape)
			print(self.dC_w2.shape)
			print(self.dC_w3.shape)
			# =============================
			self.fc1.weight -= self.learning_rate * self.dC_w1 # size : (512 X input_neurons)
			self.fc2.weight -= self.learning_rate * self.dC_w2 # size : (256 X 512)
			self.fc3.weight -= self.learning_rate * self.dC_w3 # size : (output_neurons X 256)


	def train(self, x, y):
		output = self.forward(x)
		# ========= DEBUGGING =========
		print(output)
		print(y)
		print(y.shape)
		print(output.shape)
		# =============================
		self.backward(x, output, y)




class CNN(nn.Module):
	"""
										    ⚠️ image batch size : (batch, C, H, W) ⚠️

		Example:
			>>> c1 = nn.Conv2d(3, 32, 5, 2, 1) # 3 input channels, 32 output channels, 5 kernel size, 2 is stride and 1 is padding 
			>>> c1.weight.shape
			torch.Size([32, 3, 5, 5]) # creates 32 random filters each of with size 3X5X5
	"""
	def __init__(self, input_channels, output_neurons):
		super(CNN, self).__init__()
		self.conv1     = nn.Conv2d(input_channels, 16, kernel_size=5, stride=2, padding=1) # outout image size is half of the input image size 
		self.conv2     = nn.Conv2d(16, 32, kernel_size=5, stride=2, padding=1) # outout image size is half of the input image size


		"""	
				☕️ https://discuss.pytorch.org/t/linear-layer-input-neurons-number-calculation-after-conv2d/28659/6
							
							⚠️ C * H * W is the number of input neurons for fc1 layer which is the flattened image batch ⚠️
		"""
		input_neurons_for_fc1 = self.conv2.shape[1]*self.conv2.shape[2]*self.conv2.shape[3]


		self.fc1       = nn.Linear(input_neurons_for_fc1, 200)
		self.fc2       = nn.Linear(200, 128)
		self.fc3       = nn.Linear(128, output_neurons)
		self.relu  	   = nn.ReLU()
		self.maxpool2d = nn.MaxPool2d(2, 2)


	def forward(self, batch):
		h1 = self.relu(self.maxpool2d(self.conv1(batch)))
		h1 = F.dropout(h1, p=0.5, training=self.training)

		h2 = self.relu(self.maxpool2d(self.conv2(h1), 2))
		h2 = F.dropout(h2, p=0.5, training=self.training)
		h2 = h2.view(-1, input_neurons_for_fc1)

		h3 = self.relu(self.fc1(h2))
		h3 = F.dropout(h3, p=0.5, training=self.training)

		h4 = self.relu(self.fc2(h3))
		h4 = F.dropout(h4, p=0.5, training=self.training)

		h5 = self.fc2(h4)
		return h4








