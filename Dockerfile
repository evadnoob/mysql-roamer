FROM debian:jessie
MAINTAINER david boon <david.boon@gmail.com>

RUN apt-key adv --keyserver pgp.mit.edu --recv-keys 5072E1F5
RUN echo 'deb http://repo.mysql.com/apt/debian/ jessie mysql-5.7' > /etc/apt/sources.list.d/mysql.list
RUN apt-get update
 RUN DEBIAN_FRONTEND=noninteractive apt-get install -y --force-yes --no-install-recommends \
     mysql-server \
     less \
     vim-nox \
     libssl-dev && \
   DEBIAN_FRONTEND=noninteractive apt-get autoremove -y  

ADD target/debug /opt/mysql_roamer
CMD ["bash"]


