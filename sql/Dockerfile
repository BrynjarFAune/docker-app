# syntax=docker/dockerfile:1

# use the latest base image mysql:latest
FROM mysql:latest

# set environment variables
ENV MYSQL_DATABASE mydb

# copy custom scripts or configuration files from your host to the container
COPY ./scripts/ /docker-entrypoint-initdb.d/
