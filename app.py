



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
def deploy(build: bool = typer.Option(False, "--build", help="Build from docker-compose.yml file."),
		   streamer: bool = typer.Option(False, "--kafka", help="Streamer processor for online training.")
		   ):
	typer.secho("\n________Production process________\n", fg=typer.colors.MAGENTA, bold=True)

	if build:
		typer.secho("\t➢   building with docker for production\n", fg=typer.colors.RESET, bold=True)
		# TODO : update README.md with docker-compose commands after building containers
                # TODO : secure api.py inside server folder using https://fastapi.tiangolo.com/tutorial/security/ and build it using docker-compose.yml, traefik and other DevOps tools (https://fastapi.tiangolo.com/deployment/) 
                '''
                         ____________________________
                         SOME DOCKER EXAMPLE COMMANDS 
                         ----------------------------
                  sudo docker-compose inspect <SERVICE_NAME_IN_DOCKER_COMPOSE>
                  sudo docker-compose -f docker-compose.yml up -d --build
                  sudo docker-compose run <SERVICE_NAME_IN_DOCKER_COMPOSE> bash
                  sudo docker-compose restart <SERVICE_NAME_IN_DOCKER_COMPOSE>
                  sudo docker volume inspect inobi_media_volume
                  sudo docker-compose -f docker-compose.yml logs -f
                  sudo docker-compose down -v
                  sudo docker-compose -f docker-compose.yml up --build
                  sudo docker-compose exec db psql --username=inobi --dbname=inobi < inobi.sql
                  sudo docker-compose exec db psql --username=traccar --dbname=traccar < traccar.sql
                  sudo docker build -t inobi .
                  sudo docker run -it inobi /bin/bash
                  sudo docker run -d -it -p 8586:8586 inobi
                  sudo docker images
                  sudo docker volume ls
                  sudo docker commit [CONTAINER_ID] [new_image_name]
                  sudo docker stop <CONTAINER/IMAGE_NAME/ID>
                  sudo docker rmi -f <CONTAINER/IMAGE_NAME/ID>
                  sudo docker image prune -a
                  sudo docker rmi -f $(sudo docker images -f "dangling=true" -q)
                  sudo docker rm -f $(sudo docker ps -aq)
                  sudo docker ps
                  sudo docker login --username=wildonion --password="password"
                  sudo docker commit <CONTAINER_ID> inobi
                  sudo docker cp /home/mehran/inobi/  e4d47a395d07:/home/aranobi/
               '''
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

















