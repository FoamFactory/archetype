# Docker Image Maintenance
This is a note for maintainers of the docker image [jwir3/archetype_web](https://hub.docker.com/r/jwir3/archetype_web/tags).
You probably don't need to worry about this.

In order to build the docker image, we need to set up a "dummy" `.env` file with some basic environment variables in it.
These can be overridden by the user when the download/run the image, but the build will fail without _something_ there.

**Note**: We should fix this so that it doesn't just _terminate the build_ if a `.env` file is not found.

Once you've made code changes that constitute the update(s) you want to make to the image in question, make sure that
a git tag is in place and that you're building off of that tag:
```
git checkout <tag_name>
```

Next,
```
docker-compose -f docker-compose.yml --env-file <dummy_env_file or test env_file> build web_service
```

Next, verify that the service is working as you expect (unless you built with a dummy `.env` file, in which case you
likely will not be able to check).

Next, tag the image for docker:
```
docker image tag archetype_web_service:latest jwir3/archetype_web:X.Y.Z
docker image tag archetype_web_service:latest jwir3/archetype_web:latest
```

Finally, push the image to docker hub:
```
docker image push jwir3/archetype_web:X.Y.Z
docker image push jwir3/archetype_web:latest
```
# Docker Image Update on Shared Host
To update the docker image that's being used on the archetype shared host, login to the host as you normally would. Then,
run the following:
```
docker-compose pull
docker-compose down
docker-compose up -d
```