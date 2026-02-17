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
WORKDIR /usr/share/nginx/html

# Copy built assets from build stage
COPY --from=build /app/dist .

# Custom nginx config: listen on 8080 (Render PORT)
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
