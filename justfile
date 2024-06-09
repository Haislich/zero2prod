set shell := ["powershell.exe", "-c"]
DB_USER:="postgres"
DB_PASSWORD:="password"
DB_NAME:="newsletter"
DB_PORT:="5432"
DB_HOST:="localhost"
DATABASE_URL:="postgres://postgres:password@localhost:5432/newsletter"

init_db:
  # Creating / starting the docker image
  docker run  -e POSTGRES_USER={{DB_USER}}  -e POSTGRES_PASSWORD={{DB_PASSWORD}} -e POSTGRES_DB={{DB_NAME}} -p {{DB_PORT}}:5432 -d postgres postgres -N 1000
  # Creating the database
  sqlx database create --database-url {{DATABASE_URL}}
migrate:
  # Start the container
  docker start 908b8bd9a489
  # Migrating
  sqlx migrate run --database-url {{DATABASE_URL}} 
  