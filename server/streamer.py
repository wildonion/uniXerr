


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
| constantly at every moment to the server at any time for training.
| __________________________________________________________________________________
|
|
| https://github.com/kkroening/ffmpeg-python
| https://kafka-python.readthedocs.io/en/master/usage.html
| https://aroussi.com/post/live-plotting-with-matplotlib-and-python
| https://medium.com/analytics-vidhya/data-streams-and-online-machine-learning-in-python-a382e9e8d06a
| https://www.confluent.io/blog/improving-stream-data-quality-with-protobuf-schema-validation/
| https://medium.com/data-rocks/protobuf-as-an-encoding-format-for-apache-kafka-cad4709a668d
| https://www.kai-waehner.de/blog/2018/02/13/machine-learning-trends-of-2018-combined-with-apache-kafka-ecosystem/?sfw=pass1593361971
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