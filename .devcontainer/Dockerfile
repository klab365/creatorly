FROM mcr.microsoft.com/devcontainers/rust:1

USER vscode

# Install rust version and components
RUN rustup update 1.80
RUN rustup default 1.80
RUN rustup component add rustfmt
RUN rustup component add clippy

# Install cargo apps for development
