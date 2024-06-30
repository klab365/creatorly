FROM mcr.microsoft.com/devcontainers/rust:1

USER vscode

RUN sudo apt update
RUN sudo apt upgrade -y

# Install rust version and components
RUN rustup update 1.79
RUN rustup default 1.79
RUN rustup component add rustfmt

# Install cargo apps for development
RUN cargo install just
