



# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██

'''

import os
import sys

# typer doesn't know the path of imported module in where it's imported like in here
# to solve this we have to give typer the full path of a module that we want to import it in its main file(app.py).
# it's like typer has its own namespace for running functions decorated with app.command()
# this file will import in typer main file and you can see the bytecode of this file in __pycache__ folder.
configfile = 'controller.py'
path = sys.path.append(os.path.dirname(os.path.expanduser(configfile)))

import uvicorn
import typer
import multiprocessing
from controller import app
from server import DatasetStreamer
from server import api



@app.command()
def deploy(build: bool = typer.Option(False, "--build", help="Building for production."),
		   streamer: bool = typer.Option(False, "--kafka", help="Streamer processor for online training.")
		   ):
	typer.secho("\n________Production process________\n", fg=typer.colors.MAGENTA, bold=True)

	if build:
		typer.secho("\t➢   building with docker\n", fg=typer.colors.RESET, bold=True)
		# TODO : build the app.py using https://typer.tiangolo.com/tutorial/package/ and docker or github actions for publishing python packages
		# TODO : secure api.py inside server folder using https://fastapi.tiangolo.com/tutorial/security/ and build it using docker(file), traefik and other DevOps tools (https://fastapi.tiangolo.com/deployment/)
		# TODO : after building, complete README.md guide for installing uniXerr using pip and docker containers(docker-compose)
		# ... 


	if streamer:
		typer.secho("\t➢   streaming over kafka for online training\n", fg=typer.colors.RESET, bold=True)
		# TODO : work on streamer.py inside server folder
		# ...



@app.command()
def develop(workers: int = typer.Option(multiprocessing.cpu_count(), help="Number of workers.", min=4)):

	typer.secho("\n________Running in development________\n", fg=typer.colors.MAGENTA, bold=True)
	uvicorn.run('app:api', host="api.unixerr.com", port=8000, reload=True, workers=workers, lifespan="on")

if __name__ == "__main__":
	app()

















