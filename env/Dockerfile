FROM rust:1.32.0-stretch

# Install wasmpack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install node/npm
RUN curl -sL https://deb.nodesource.com/setup_11.x | bash - \
  && apt-get install -y nodejs
