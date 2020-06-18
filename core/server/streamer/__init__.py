

# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██


 -----------------------------------------------------------------------------------
| features are sent weekly to the uniXerr server for prediction over kafka streaming
| the model is always training, this means that the features are sent from the app 
| constantly at every moment to the server at any time for training VAE, 
| but the clustering is done weekly using VAE model.
| __________________________________________________________________________________
|
|
|
| https://aroussi.com/post/live-plotting-with-matplotlib-and-python
| https://medium.com/analytics-vidhya/data-streams-and-online-machine-learning-in-python-a382e9e8d06a
| https://www.confluent.io/blog/improving-stream-data-quality-with-protobuf-schema-validation/
| https://medium.com/data-rocks/protobuf-as-an-encoding-format-for-apache-kafka-cad4709a668d
|
|

'''


from ._pipeline import pipeline
import os

class DatasetStreamer():
	def __init__(self, plotting_kwargs):
		pass

	def __call__(self):
		pass