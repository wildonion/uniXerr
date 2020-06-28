



# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██

'''


import uvicorn
import typer
import subprocess
import multiprocessing
from controller import app
from server.api import api



@app.command()
def deploy(build: bool = typer.Option(False, "--build", help="Building for production."),
		   streamer: bool = typer.Option(False, "--kafka", help="Streamer processor for online training.")
		   ):
	typer.echo("\n________Production process________\n")

	if build:
		typer.echo("\t---building with docker\n")
		# TODO : build api.py inside server folder using docker and traefik
		# ...


	if streamer:
		typer.echo("\t---streaming over kafka for online training\n")
		# TODO : work on streamer.py inside server folder
		# ...



@app.command()
def develop(workers: int = typer.Option(4, help="Number of workers.", min=4)):

	typer.echo("\n________Running in development________\n")
	workers = number_of_workers() if workers != 4 else workers
	uvicorn.run('app:api', host="127.0.0.1", port=8000, reload=True, workers=workers)


if __name__ == "__main__":
	app()

















