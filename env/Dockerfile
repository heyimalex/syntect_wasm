FROM rust:1.32.0-stretch

# Install wasmpack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install node/npm
RUN curl -sL https://deb.nodesource.com/setup_11.x | bash - \
  && apt-get install -y nodejs

# Install emsdk
RUN apt-get update && apt-get install -y --no-install-recommends cmake default-jre
ADD eminstall.sh /root/eminstall.sh
RUN chmod +x /root/eminstall.sh && /root/eminstall.sh && rm /root/eminstall.sh
RUn touch /root/.bashrc && echo 'source /emsdk/emsdk_env.sh' >> /root/.bashrc
