FROM mariadb:latest

ENV MYSQL_ROOT_PASSWORD=root
ENV MYSQL_DATABASE=hryak
ENV MYSQL_USER=klewy
ENV MYSQL_PASSWORD=root

COPY hryak.sql /docker-entrypoint-initdb.d/

EXPOSE 3306
