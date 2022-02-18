# syntax=docker/dockerfile:1
#FROM python:3.7-alpine
#WORKDIR /code
#ENV FLASK_APP=app.py
#ENV FLASK_RUN_HOST=0.0.0.0
#RUN apk add --no-cache gcc musl-dev linux-headers
#COPY requirements.txt requirements.txt
#RUN pip install -r requirements.txt
#EXPOSE 5000
#COPY . .
#CMD ["flask", "run"]

FROM rustlang/rust:nightly

WORKDIR /usr/src/archetype
COPY . .

RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get install -y sqlite3 libsqlite3-dev
RUN apt-get install -y lynx

RUN cargo install --path .
RUN cargo install diesel_cli
RUN diesel migration run

CMD ["archetype"]