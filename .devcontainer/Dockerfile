FROM mcr.microsoft.com/devcontainers/rust:1

USER vscode

RUN sudo apt update
RUN sudo apt upgrade -y

# Install rust version and components
RUN rustup update 1.77.1
RUN rustup default 1.77.1

# Install cargo apps for development
RUN cargo install just
