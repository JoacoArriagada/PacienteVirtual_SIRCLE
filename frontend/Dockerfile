FROM node:24-bookworm-slim AS compilacion
WORKDIR /app
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci --no-audit --no-fund
COPY frontend/index.html frontend/tsconfig.json ./
COPY frontend/scripts/build.mjs ./scripts/build.mjs
COPY frontend/src ./src
COPY frontend/recursos/semana5 ./recursos/semana5
RUN npm run build

FROM nginx:stable-alpine
COPY frontend/servidor.conf /etc/nginx/conf.d/default.conf
COPY --from=compilacion /app/dist/ /srv/sircle/
EXPOSE 8080
