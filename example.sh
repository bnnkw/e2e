#!/bin/bash

stop() {
  docker stop e2e-apache e2e-selenium
  docker network rm e2e-net
}

trap stop EXIT

cargo build --release

docker network create e2e-net
docker run -d --name e2e-apache --network e2e-net --rm httpd:2.4
docker run -d --name e2e-selenium --network e2e-net -p 4444:4444 --rm --shm-size="2g" selenium/standalone-edge:4.32.0-20250505

echo "Waiting for WebDriver..."
until curl -sf http://localhost:4444/status | grep -q '"ready": true'; do
  sleep 1
done

./target/release/e2e -f example-e2e.yaml run
