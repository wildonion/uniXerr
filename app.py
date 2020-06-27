



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
from controller import app
from server.api import api


@app.command()
def deploy(build: bool = typer.Option(False, "--build", help="Building for production.")):
	typer.echo("\n________Production process________\n")
	if build:
		typer.echo("\t---building\n")


@app.command()
def develop(workers: int = typer.Option(4, help="Number of workers", min=4),
			asgi_server: str = typer.Option('gunicorn', help="ASGI server. uvicorn or gunicorn")):

	typer.echo("\n________Running in development________\n")
	
	if asgi_server == 'uvicorn':
		uvicorn.run('app:api', host="127.0.0.1", port=8000, reload=True, workers=4)
	
	else: # linux only
		cmd = "gunicorn app:api -w 4 -k uvicorn.workers.UvicornWorker"
		subprocess.run([cmd], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True)




if __name__ == "__main__":
	app()

















