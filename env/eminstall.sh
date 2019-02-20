#/bin/sh -e

# Emscripten install script. I prefer adding a script and running it to
# fitting everything into a single run command.
git clone https://github.com/emscripten-core/emsdk.git /emsdk
cd /emsdk
./emsdk update
./emsdk install latest
./emsdk activate latest
