# Stage 1: Build the React app
FROM node:22-alpine AS builder

WORKDIR /app
COPY package*.json ./
RUN npm install --frozen-lockfile
COPY . .

# Pass build environment variables to Vite
ARG VITE_NATS_URL
ENV VITE_NATS_URL=${VITE_NATS_URL}

# Build the production version of the app
RUN npm run build

# Stage 2: Serve the app using nginx
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
