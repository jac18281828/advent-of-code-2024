FROM ghcr.io/jac18281828/rust:latest

ARG PROJECT=advent-of-code-2024
WORKDIR /workspaces/${PROJECT}

USER rust
ENV USER=rust
ENV PATH=/home/${USER}/.cargo/bin:${PATH}::/usr/local/go/bin
# source $HOME/.cargo/env

COPY --chown=rust:rust . .
