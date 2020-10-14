
# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██


'''

from cassandra.cluster import Cluster
from dotenv import load_dotenv, find_dotenv
import os
import typer


load_dotenv(find_dotenv())
HOSTS  = os.getenv("HOST").split(",")
PORT = os.getenv("PORT")
KEY_SPACE = os.getenv("KEY_SPACE")
USERNAME = os.getenv("USERNAME")
PASSWORD = os.getenv("PASSWORD")


class init:
	def __new__(cls):
		cluster = Cluster(HOSTS, port=PORT) # db hosts and port to start clusters
		cls.session = cluster.connect() # connect to hosts - cls is server.db.init so cls.session means self.session for init class
		return super(init, cls).__new__(cls)

  
	def __init__(self):
		self.session.execute(f'USE {KEY_SPACE}')
		typer.secho("➲   [Database initialized successfully]", fg=typer.colors.WHITE, bold=True)