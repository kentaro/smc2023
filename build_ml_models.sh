set -eu

cd wasm-standalone-builder/

docker-compose build wasm-standalone-builder
docker-compose run wasm-standalone-builder

cp build/*.wasm ../wasm
cd ..
