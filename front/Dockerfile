FROM node:16-slim as build-stage

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# installing packages
RUN npm install
RUN npm run build

# nginxでルーティングの準備
FROM nginx:alpine as production-stage
COPY --from=build-stage /usr/src/app/dist /usr/share/nginx/html
COPY --from=build-stage /usr/src/app/nginx/ /etc/nginx/conf.d/

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 8080

# Run the web service on container startup.
CMD ["nginx", "-g", "daemon off;"]
