#!/bin/sh



# 📌 CMD and ENTRYPOINT directive execute the command first and then stop the container with exit code 0 (no error)
# 📌 containers are reachable by their service names



# ==============
# first building
# ==============
# ✅ sudo docker stop $(sudo docker ps -a -q) && sudo docker-compose down -v && sudo docker system prune -af --volumes
# ✅ sudo docker-compose -f docker-compose.yml build --no-cache && sudo docker-compose up -d --force-recreate
# ✅ sudo docker-compose -f docker-compose.yml logs -f




# =================
# on changing build
# =================
# ✅ sudo docker stop $(sudo docker ps -a -q)                               --------------------------------> stop all running containers
# ✅ sudo docker-compose -f docker-compose.yml build --no-cache             --------------------------------> update images, omit the --no-cache if you want to use cache building
# ✅ sudo docker-compose down -v && sudo docker-compose up --force-recreate --------------------------------> remove and rebuild all containers, you will lose the old ones data
# ✅ sudo docker-compose up -d --force-recreate                             --------------------------------> omit the --force-recreate if you don't want to recreate all the containers
# ✅ sudo docker-compose -f docker-compose.yml logs                         --------------------------------> see the docker containers logs
# ✅ sudo docker-compose run -u <SERVICE_NAME> web bash                     --------------------------------> accessing bash shell of a service
# ✅ sudo docker-compose exec db psql --username=uniXerr --dbname=uniXerr   --------------------------------> accessing inside the db shell




# ========================================
# saving and loading images on other hosts
# ========================================
# ✅ sudo docker save $(sudo docker images -a -q) | gzip > uniXerr.tar.gz
# ✅ sudo docker load -i -o uniXerr.tar




# sudo docker network connect <NETWORK_NAME> <CONTAINER/IMAGE_NAME/ID>
# sudo docker network create -o com.docker.network.bridge.enable_icc=true -d bridge <NETWORK_NAME>
# sudo docker network ls
# sudo docker network inspect -f '{{range .Containers}}{{.Name}} {{end}}' <NETWORK_NAME>
# -------------------------------------------------------------------------------------------------------------
# sudo docker-compose -f docker-compose.yml build --no-cache
# sudo docker-compose up -d --force-recreate
# sudo docker-compose -f docker-compose.yml logs -f
# sudo docker-compose run -u uniXerr <SERVICE_NAME_IN_DOCKER_COMPOSE> bash
# sudo docker-compose restart <SERVICE_NAME_IN_DOCKER_COMPOSE>
# sudo docker-compose down -v
# sudo docker-compose -f docker-compose.yml up --build
# sudo docker-compose exec db psql --username=uniXerr --dbname=uniXerr < uniXerr.sql
# -------------------------------------------------------------------------------------------------------------
# sudo docker save $(sudo docker images -q) -o docker-utils/uniXerr.tar
# sudo docker load -i -o docker-utils/uniXerr.tar
# sudo docker save $(sudo docker images -q) | gzip > uniXerr.tar.gz
# sudo docker load -i uniXerr.tar.gz
# sudo docker ps
# sudo docker exec <CONTAINER/IMAGE_NAME/ID>_A ping <CONTAINER/IMAGE_NAME/ID>_B -c2
# sudo docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' <CONTAINER_ID>
# sudo docker inspect -f '{{index .Options "com.docker.network.bridge.enable_icc"}}' <NETWORK_NAME>
# sudo docker build -t uniXerr .
# sudo docker run -it uniXerr /bin/bash
# sudo docker run -d -it -p 8586:8586 uniXerr --network=<NETWORK_NAME>
# sudo docker images
# sudo docker volume ls
# sudo docker volume inspect <CHOOSE_ONE_FROM_ABOVE_COMMAND>
# sudo docker commit <CONTAINER/IMAGE_NAME/ID> <NEW_IMAGE_NAME>
# sudo docker stop <CONTAINER/IMAGE_NAME/ID>
# sudo docker rmi -f <CONTAINER/IMAGE_NAME/ID>
# sudo docker image prune -a
# sudo docker system prune -a
# sudo docker stop $(docker ps -a -q)
# sudo docker rmi -f $(sudo docker images -a -q)
# sudo docker rmi -f $(sudo docker images -f "dangling=true" -q)
# sudo docker rm -f $(sudo docker ps -aq)
# sudo docker login --username=wildonion --password="password"
# sudo docker commit <CONTAINER/IMAGE_NAME/ID> uniXerr
# sudo docker cp /home/wildonion/uniXerr/  e4d47a395d07:/home/wildonion/
# sudo docker cp 4ba0d2853dd2:/opt/uniXerr/migrations /home/wildonion/utils





echo "⌛ Stopping and Removing all containers..."
sudo docker stop $(sudo docker ps -a -q) && sudo docker-compose down -v
echo "⌛ Pruning system..."
sudo docker system prune -af --volumes
echo "⌛ Building images with --no-cache enabled..."
sudo docker-compose -f docker-compose.yml build --no-cache
echo "⌛ Running all containers from built images with --force-recreate enabled..."
sudo docker-compose up -d --force-recreate
echo "⌛ Following containers log..."
sudo docker-compose -f docker-compose.yml logs -f





