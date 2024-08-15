FROM node:20-bookworm as builder

WORKDIR /app

COPY package*.json .

RUN npm i

COPY . .

RUN --mount=type=cache,target=/app/lhavrais-pay/target \
    npm run build


FROM nginx:1-alpine

COPY --from=builder /app/dist /usr/share/nginx/html

COPY nginx.conf /etc/nginx/nginx.conf

COPY certs/dummy-certs /etc/letsencrypt/dummy-certs
