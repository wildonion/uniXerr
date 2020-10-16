
# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██


'''

from cassandra.io.libevreactor import LibevConnection
from cassandra.cqlengine.management import sync_table
from cassandra.cqlengine import connection
from cassandra import ConsistencyLevel
from cassandra.cluster import Cluster
from dotenv import load_dotenv, find_dotenv
from .schema import User
import os
import typer


load_dotenv(find_dotenv())
HOSTS  = os.getenv("HOST")
PORT = os.getenv("PORT")
KEYSPACE = os.getenv("KEYSPACE")
USERNAME = os.getenv("USERNAME")
PASSWORD = os.getenv("PASSWORD")
REPLICATION = os.getenv("REPLICATION")
os.environ['CQLENG_ALLOW_SCHEMA_MANAGEMENT'] = '1'


class init:
	def __new__(cls, *args, **kwargs):
		cls.__cluster = Cluster(HOSTS, port=PORT) # db hosts and port to start clusters
		cls.__cluster.connection_class = LibevConnection # use libev event loop 
		cls.__session = cls.__cluster.connect() # connect to hosts - cls is server.db.init
		cls.__session.execute(f"CREATE KEYSPACE IF NOT EXISTS {KEYSPACE} WITH REPLICATION = {REPLICATION};")
		cls.__session.execute(f'USE {KEYSPACE}')
		connection.register_connection('DML', session=cls.__session) # DML cluster for sync_table ops
		return super(init, cls).__new__(cls, *args, **kwargs)

  
	def __init__(self):
		sync_table(User) # create users_info table if not exitscass
		typer.secho("➲   [Database initialized successfully]", fg=typer.colors.WHITE, bold=True)


	def query(self, q, v):
		stmt = self.__session.prepare(q)
		return self.__session.execute_async(stmt, v) # return a future object
