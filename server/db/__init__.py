
# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██



https://stackoverflow.com/questions/24949676/difference-between-partition-key-composite-key-and-clustering-key-in-cassandra


Explanations:

    A primary key uniquely identifies a row, composed of partition key(s) [and optional clustering keys(or columns)]
    A composite key is a key formed from multiple columns.
    A partition key is the primary lookup to find a set of rows, i.e. a partition. The hash value of Partition key is used to determine the specific node in a cluster to store the data
    A clustering key is the part of the primary key that isn't the partition key (and defines the ordering within a partition or responsible node and it's replicas).

Examples:

    PRIMARY KEY (a): The partition key is a.
    PRIMARY KEY (a, b): The partition key is a, the clustering key is b.
    PRIMARY KEY ((a, b)): The composite partition key is (a, b).
    PRIMARY KEY (a, b, c): The partition key is a, the composite clustering key is (b, c).
    PRIMARY KEY ((a, b), c): The composite partition key is (a, b), the clustering key is c.
    PRIMARY KEY ((a, b), c, d): The composite partition key is (a, b), the composite clustering key is (c, d).


- Compound Primary Key: The clustering keys are optional in a Primary Key. If they aren't mentioned, it's a simple primary key. If clustering keys are mentioned, it's a Compound primary key.
- Composite Partition Key: Using just one column as a partition key, might result in wide row issues (depends on use case/data modeling). Hence the partition key is sometimes specified as a combination of more than one column.
- All select and insert queries must specifie a partition key or a set of these keys, if the key doesn't exist it'll insert otherwise i'll update
- Cassandra is based on column family architecture, means that you can insert a row filled just a few columns only and not all of them and according to this architecture cassandra can store 2 billions cells (rows X column key X column value)


'''

from cassandra.io.libevreactor import LibevConnection
from cassandra.cqlengine.management import sync_table
from cassandra.cqlengine import connection
from cassandra.cluster import Cluster
from cassandra.policies import DCAwareRoundRobinPolicy
from cassandra.auth import PlainTextAuthProvider
from dotenv import load_dotenv, find_dotenv
from .schema import User, Position
import os
import typer


load_dotenv(find_dotenv())
USERNAME = os.getenv("USERNAME")
PASSWORD = os.getenv("PASSWORD")
AUTH_PROVIDER = PlainTextAuthProvider(USERNAME, PASSWORD)
CLOUD_CONFIG = {'secure_connect_bundle': os.path.dirname(os.path.abspath(__file__))+'/secure-connect-unixerr.zip'}
os.environ['CQLENG_ALLOW_SCHEMA_MANAGEMENT'] = '1'

class init:
	def __new__(cls, *args, **kwargs):
		try:
			cls.__cluster = Cluster(auth_provider=AUTH_PROVIDER, cloud=CLOUD_CONFIG, load_balancing_policy=DCAwareRoundRobinPolicy(local_dc="dc-1")) # db hosts and port to start clusters
			cls.__cluster.connection_class = LibevConnection # use libev event loop 
			cls.__session = cls.__cluster.connect() # connect to hosts - cls is server.db.init
			connection.register_connection('DML', session=cls.__session) # DML cluster for sync_table ops
			return super(init, cls).__new__(cls, *args, **kwargs)
		except Exception as e:
			typer.secho(f"➲   [Could not initialize database] ::: {e}", fg=typer.colors.RED, bold=True)

  
	def __init__(self):
		sync_table(User) # create users_info table if not exits
		sync_table(Position) # create positions table if not exits
		typer.secho("➲   [Database initialized successfully]", fg=typer.colors.WHITE, bold=True)


	def close(self):
		self.__cluster.shutdown()
		typer.secho(f"➲   [Database shutdown successfully]", fg=typer.colors.WHITE, bold=True)


	def query(self, q, v):
		stmt = self.__session.prepare(q)
		return self.__session.execute_async(stmt, v) # return a future object