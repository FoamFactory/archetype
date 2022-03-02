[![](docs/assets/logo.png)](https://github.com/FoamFactory/archetype)

# Archetype
An easy solution for avatar services.

# API
You can view the API for Archtype [here](https://app.swaggerhub.com/apis-docs/FoamFactory/Archetype/1.0.0)

# Installation
The easiest way to install archetype is from source through github:
```bash
git clone git@github.com:FoamFactory/archetype
```

# Setup
Archetype is run via a [docker](https://www.docker.com/) container. As such, you're going to need docker engine and `docker-compose` installed. Installation instructions:
  - [Docker engine](https://docs.docker.com/engine/install/)
  - [docker-compose](https://docs.docker.com/compose/install/)

Once you have docker installed, you will need to edit `docker-compose.yml` to include the host(s) which you wish to allow access to the avatar service _from_. This is a comma-separated list of IP addresses. If you only want access to be available from the docker host machine, you can likely use the default. If this doesn't work (you get errors saying that your host is not in the allowed list), you may need to run the following command on your docker host to determine its in-network IP:
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

# Installation and Maintenance Service
FoamFactory offers the following installation and maintenance services for archetype, so you don't even have to do the above:
- Install on one of our shared hosts, with maintenance: **$3/month** (billed annually)
- Install on a linode instance, **$150** (one-time), plus [linode fees](https://www.linode.com/pricing/)

If you're interested in having us perform the setup for you, please fill out the following [query form](https://forms.gle/FfJtdic2dz2md5bu8) and we'll respond as soon as possible.

# Building
If you want to build from source, you can clone this project from Github and follow these steps to build. You will need
docker installed and working (see above) to run the MySQL database. You will additionally need a Rust toolchain installed,
typically using `rustup`: [Installing Rust](https://www.rust-lang.org/tools/install). As of this writing, the minimum
`rustc` version required is `1.60.0-nightly`.

1.Start the MySQL database using docker (you may want to change the defaults in `.env`):
```bash
docker-compose up db
```
2. Specify your database connection url:
```bash
export DATABASE_URL=mysql://archetype:<password>@0.0.0.0:3307/archetype_production
```
3. Use `cargo` to build and run:
```bash
cargo run
```