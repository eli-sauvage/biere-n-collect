FROM node:20-bookworm as builder

WORKDIR /app

COPY front/package*.json .

RUN npm i 

RUN ls node_modules/.bin
RUN echo $PATH

COPY front/ .

RUN npm run build

FROM nginx:1-alpine

COPY --from=builder /app/dist /usr/share/nginx/html

COPY nginx.conf /etc/nginx/nginx.conf
