FROM nginx:1.24.0 as local


RUN apt-get update && apt-get install logrotate -y
COPY /docker/nginx/nginx.conf /etc/nginx/nginx.conf
COPY /docker/nginx/logrotate.conf /etc/logrotate.conf

CMD ["nginx", "-g", "daemon off;"]
