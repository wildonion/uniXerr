



# USAGE : pm2 start bot.py


# https://docs.aiogram.dev/en/latest/quick_start.html

# TOKEN : 1488597268:AAHlrJReS2da55dC4CsaRn_zTFHQbI9eqTM
# TODO - choose picture for the bot
# TODO - send every printing message to the user



import torch
import os, sys
from PIL import Image
import numpy as np





model_command = "mlp" # TODO - get this value from the bot
input_image = None # TODO - get this value from the bot



cnn_pre_trained_model_path = "utils/cnn.pth"
mlp_pre_trained_model_path = "utils/mlp.pth"


# ------------ load pre-trained model for predictions 
# --------------------------------------------------------
if model_command == "mlp":
	if mlp_pre_trained_model_path and os.path.exists(mlp_pre_trained_model_path):
		print(f"\n‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗found existing mlp pre-trained model‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗\n")
		try:
			checkpoint = torch.load(mlp_pre_trained_model_path)
			print(f"\t✅ loaded pre-trained model from {mlp_pre_trained_model_path}\n")
		except IOError:
			print(f"\t❌ can't load pre-trained model from : {mlp_pre_trained_model_path}\n")

		# TODO - load mlp model
		# start predicting using mlp model
		# remember to greyscale input image and resize them to between 60 and 90 pixels
	else:
		print(f"\t❌ can't find the {model_command} model path!")


elif model_command == "cnn":
	if cnn_pre_trained_model_path and os.path.exists(cnn_pre_trained_model_path):
		print(f"\n‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗found existing cnn pre-trained model‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗\n")
		try:
			checkpoint = torch.load(cnn_pre_trained_model_path)
			print(f"\t✅ loaded pre-trained model from {cnn_pre_trained_model_path}\n")
		except IOError:
			print(f"\t❌ can't load pre-trained model from : {cnn_pre_trained_model_path}\n")

		# TODO - load mlp model
		# start predicting using mlp model
		# remember to greyscale input image and resize them to between 60 and 90 pixels
	else:
		print(f"\t❌ can't find the {model_command} model path!")	


else:
	pass