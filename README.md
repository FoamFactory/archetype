[![](docs/assets/logo.png)](https://github.com/FoamFactory/archetype)

# Archetype
An easy solution for avatar services.

## Features
  - **Fast**: We're built on Rust with a MySQL backend, so archetype is faster than light (ok, no, not really faster than light...nothing can be [faster than light](https://en.wikipedia.org/wiki/Special_relativity))
  - **Safe**: Again, being built on Rust, we're type- and memory-safe.
  - **Straightforward API**: The API to communicate with Archetype is easy to understand and use for anyone that's familiar with REST.
  - **Lightweight**: Archetype is easy to set up and can be forgotten about once setup. Just run our Docker containers on your server, and start using avatars.

# API
You can view the API for Archetype [here](https://app.swaggerhub.com/apis-docs/FoamFactory/Archetype)

# Installation and Maintenance Service
The installation instructions below aren't complicated, but we make it even easier for you by providing a service to
install and maintain an Archetype instance for a really low cost.

The following services are offered through FoamFactory:
- Install on one of our shared hosts, with maintenance: **$3/month** (billed annually)
- Install on a linode instance, **$150** (one-time), plus [linode fees](https://www.linode.com/pricing/)

If you're interested in having us perform the setup for you, please fill out the following
[query form](https://forms.gle/FfJtdic2dz2md5bu8), and we'll respond as soon as possible.

# Installation
The easiest way to install archetype is from source through github:
```bash
git clone git@github.com:FoamFactory/archetype
```

# Setup
## From Source
Archetype is run via a [docker](https://www.docker.com/) container. As such, you're going to need docker engine and `docker-compose` installed. Installation instructions:
  - [Docker engine](https://docs.docker.com/engine/install/)
  - [docker-compose](https://docs.docker.com/compose/install/)

Once you have docker installed, you will need to download this repository, edit `docker-compose.yml` to include the
host(s) which you wish to allow access to the avatar service _from_. This is a comma-separated list of IP addresses.
If you only want access to be available from the docker host machine, you can likely use the default. If this doesn't
work (you get errors saying that your host is not in the allowed list), you may need to run the following command on
your docker host to determine its in-network IP:
```bash
# Get the docker network up and running
docker-compose up -d

# Query to find your IP address. Place the result of this command into the ALLOWED_HOSTS variable in docker-compose.yml
docker network inspect archetype_archetype_net -f '{{range .IPAM.Config}}{{.Gateway}}{{end}}'

# Bring the network down again
docker-compose down
```

Then, to get the service running, simply run:
```bash
docker-compose up -d
```

## From Docker Hub
You can also create your own `docker-compose.yml` file and have it download the appropriate images from Docker Hub. You
will likely want to create a `.env` file first, and populate the following values:
```bash
# You can use anything you want here, but we recommend staging or production
ARCHETYPE_MODE=production

# You can put any regex here to allow from whatever IP addresses you desire. This
# is currently setup for allowing from localhost only.
ARCHETYPE_ALLOWED_HOSTS=127.0.0.1

# The name of the database you want to use. Again, you can name it anything, but we recommend just leaving this
# alone.
MYSQL_DATABASE="archetype_${ARCHETYPE_MODE}"

# The name of the user that will be accessing the database on a regular basis.
MYSQL_USER=archetype

# The password for the user that will be accessing the database on a regular basis.
MYSQL_PASSWORD=somepassword

# The root password you want to set for the MySQL database instance.
MYSQL_ROOT_PASSWORD=someotherpassword

# The name of the host for the mysql database. Typically, this can be left as 'db' if using
# docker-compose. If the database is on a separate host, then you'll want to make this
# the host of the actual database in question.
MYSQL_HOST="db"

# The port on which the mysql server is running. Typically, this can be left as '3306' if
# using docker-compose (or the standard mysql setup).
MYSQL_PORT="3306"
```

Once this is complete, write your `docker-compose.yml` file. A typical `docker-compose.yml` file looks something like:

```yaml
version: "3.9"
services:
  db:
    image: "mysql:5.7"
    container_name: archetype_db
    restart: unless-stopped
    volumes:
      - db_data:/var/lib/mysql
    networks:
      - archetype_net
    env_file:
      - .env.sample
    ports:
      # Use port 3307 on the host machine so as not to interfere with another mysql server already running
      # You can safely disable this if you don't want to access the MySQL container outside of archetype
      - "3307:3306"
    healthcheck:
      test: mysqladmin ping -h 127.0.0.1 -u $$MYSQL_USER --password=$$MYSQL_PASSWORD

  web_service:
    image: "jwir3/archetype_web:latest"
    env_file:
      - .env.sample
    environment:
      - ARCHETYPE_MODE
      - ARCHETYPE_ALLOWED_HOSTS=172\.([0-9]{1,3})\.([0-9]{1,3})\.([0-9]{1,3})
      - MYSQL_USER
      - MYSQL_PASSWORD
      - MYSQL_DATABASE="${MYSQL_DATABASE}_${ARCHETYPE_MODE}"
      - DB_URL="mysql://$MYSQL_USER:$MYSQL_PASSWORD@db/$MYSQL_DATABASE"
    depends_on:
      db:
        condition: service_healthy
    ports:
      - "8000:8000"
    networks:
      - archetype_net
networks:
  archetype_net:
    driver: bridge
volumes:
  db_data:
```

# Building
If you want to build from source, you can clone this project from Github and follow these steps to build. You will need
docker installed and working (see above) to run the MySQL database. You will additionally need a Rust toolchain installed,
typically using `rustup`: [Installing Rust](https://www.rust-lang.org/tools/install). As of this writing, the minimum
`rustc` version required is `1.60.0-nightly`.

## Linux
1. Point the `docker-compose.yml` file to the linux variant: `ln -s docker-compose.linux.yml docker-compose.yml`
2. Copy the `.env.sample` file to `.env.test`: `cp .env.sample .env.test`.
3. Start the MySQL database using docker (you may want to change the defaults in `.env.test`):
```bash
docker-compose up db
```
4. Use `cargo` to build and run:
```bash
cargo run
```

## MacOS
1. Install MySQL using homebrew: `brew install mysql`
2. Point the `docker-compose.yml` file to the macos variant: `ln -s docker-compose.mac.yml docker-compose.yml`
3.  Copy the `.env.sample` file to `.env.test`: `cp .env.sample .env.test`.
4. Start the MySQL database using docker (you may want to change the defaults in `.env.test`):
```bash
docker-compose up db
```
5. Use `cargo` to build and run:
```bash
source .env.test && cargo run
```
