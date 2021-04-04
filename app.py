



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
					 _______________________________________________
					|	 SOME DOCKER EXAMPLE COMMANDS           |
					|containers are reachable by their service names|
					 -----------------------------------------------
                  	sudo docker network connect <NETWORK_NAME> <CONTAINER/IMAGE_NAME/ID>
			sudo docker network create -o com.docker.network.bridge.enable_icc=true -d bridge <NETWORK_NAME>
			sudo docker network ls
			sudo docker network inspect -f '{{range .Containers}}{{.Name}} {{end}}' <NETWORK_NAME>
			-------------------------------------------------------------------------------------------------------------
			sudo docker-compose -f docker-compose.yml build --no-cache
			sudo docker-compose up -d --force-recreate
			sudo docker-compose -f docker-compose.yml logs -f
			sudo docker-compose run -u aranobi <SERVICE_NAME_IN_DOCKER_COMPOSE> bash
			sudo docker-compose restart <SERVICE_NAME_IN_DOCKER_COMPOSE>
			sudo docker-compose down -v
			sudo docker-compose -f docker-compose.yml up --build
			sudo docker-compose exec db psql --username=uniXerr --dbname=uniXerr < uniXerr.sql
			-------------------------------------------------------------------------------------------------------------
			sudo docker save $(sudo docker images -q) -o docker-utils/uniXerr.tar
			sudo docker load -i -o docker-utils/uniXerr.tar
			sudo docker ps
			sudo docker exec <CONTAINER/IMAGE_NAME/ID>_A ping <CONTAINER/IMAGE_NAME/ID>_B -c2
			sudo docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' <CONTAINER_ID>
			sudo docker inspect -f '{{index .Options "com.docker.network.bridge.enable_icc"}}' <NETWORK_NAME>
			sudo docker build -t uniXerr .
			sudo docker run -it uniXerr /bin/bash
			sudo docker run -d -it -p 8586:8586 uniXerr --network=<NETWORK_NAME>
			sudo docker images
			sudo docker volume ls
			sudo docker volume inspect <CHOOSE_ONE_FROM_ABOVE_COMMAND>
			sudo docker commit <CONTAINER/IMAGE_NAME/ID> <NEW_IMAGE_NAME>
			sudo docker stop <CONTAINER/IMAGE_NAME/ID>
			sudo docker rmi -f <CONTAINER/IMAGE_NAME/ID>
			sudo docker image prune -a
			sudo docker system prune -a
			sudo docker rmi -f $(sudo docker images -a -q)
			sudo docker rmi -f $(sudo docker images -f "dangling=true" -q)
			sudo docker rm -f $(sudo docker ps -aq)
			sudo docker login --username=wildonion --password="password"
			sudo docker commit <CONTAINER/IMAGE_NAME/ID> uniXerr
			sudo docker cp /home/wildonion/uniXerr/  e4d47a395d07:/home/wildonion/
			sudo docker cp 4ba0d2853dd2:/opt/uniXerr/migrations /home/mehran/utils/
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

















