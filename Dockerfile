# Build stage: Rust + Trunk for WASM (Rust 1.85+ required for trunk's edition2024 deps)
FROM rust:latest AS build
WORKDIR /app

# Install trunk and wasm target
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

# Copy project files
COPY . .

# Build (trunk outputs to dist/)
RUN trunk build --release

# Runtime stage: nginx to serve static files
FROM nginx:alpine
RUN apk add --no-cache gettext
WORKDIR /usr/share/nginx/html

# Copy built assets from build stage
COPY --from=build /app/dist .

# Nginx config template: PORT is substituted at runtime (Render sets PORT=10000)
COPY nginx.conf.template /etc/nginx/conf.d/default.conf.template
RUN rm -f /etc/nginx/conf.d/default.conf

# Startup: substitute PORT into nginx config, then start nginx
COPY start.sh /start.sh
RUN chmod +x /start.sh

EXPOSE 8080
CMD ["/start.sh"]
