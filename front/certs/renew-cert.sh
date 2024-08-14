docker run -it --rm \
-v ./certs:/etc/letsencrypt \
-v ./nginx.conf:/etc/nginx/nginx.conf \
certbot/certbot renew