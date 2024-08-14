docker run -it --rm \
-v ./certs:/etc/letsencrypt \
-v ./nginx.conf:/etc/nginx/nginx.conf \
-v ./site:/var/www/html \
certbot/certbot certonly --webroot -w /var/www/html -d eli-sauvage.eu