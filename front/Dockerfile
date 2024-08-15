FROM node:20-bookworm AS builder

WORKDIR /app

COPY front/package*.json .

RUN npm i 

ARG VITE_SITE_URL
ENV VITE_SITE_URL=${VITE_SITE_URL}

ARG VITE_API_URL
ENV VITE_API_URL=${VITE_API_URL}

COPY front/ .

RUN npm run build

FROM nginx:1-alpine

COPY --from=builder /app/dist /usr/share/nginx/html

COPY nginx.conf /etc/nginx/nginx.conf
