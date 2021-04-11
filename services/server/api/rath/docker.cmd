



==============
first building
==============
✅ sudo docker stop $(sudo docker ps -a -q) && sudo docker-compose down -v && sudo docker system prune -af --volumes
✅ sudo docker-compose -f docker-compose.yml build --no-cache && sudo docker-compose up -d --force-recreate
✅ sudo docker-compose -f docker-compose.yml logs




=================
on changing build
=================
✅ sudo docker stop $(sudo docker ps -a -q)                               --------------------------------> stop all running containers
✅ sudo docker-compose -f docker-compose.yml build --no-cache             --------------------------------> update images, omit the --no-cache if you want to use cache building
✅ sudo docker-compose down -v && sudo docker-compose up --force-recreate --------------------------------> remove and rebuild all containers, you will lose the old ones data
✅ sudo docker-compose up -d --force-recreate                             --------------------------------> omit the --force-recreate if you don't want to recreate all the containers
✅ sudo docker-compose -f docker-compose.yml logs                         --------------------------------> see the docker containers logs
✅ sudo docker-compose run -u rath web bash                               --------------------------------> accessing bash shell of we service




========================================
saving and loading images on other hosts
========================================
✅ sudo docker save $(sudo docker images -q) | gzip > rath.tar.gz
✅ sudo docker load -i -o rath.tar
