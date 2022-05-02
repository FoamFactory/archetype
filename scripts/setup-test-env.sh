#!/bin/bash
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
env_file="${parent_path}/../.env.test"

# Start the mysql server locally using docker
docker-compose --env-file "${env_file}" up -d db

sleep 1s

# Load environment variables
#set -- devEnv
. "${env_file}"

# Run any pending migrations
if !!diesel migration pending --database-url="${DATABASE_URL}" > /dev/null 2>&1; then
  echo "Diesel migration pending. Running migration..."
  diesel migration run --database-url="${DATABASE_URL}"
else
  echo "No diesel migration pending."
fi