#!/usr/bin/env bash
docker run --rm -it -v $(pwd)/build/output:/speculos/apps --publish 41000:41000 --publish 5001:5001 --publish 4001:4001 \
    speculos-runner --display headless --vnc-port 41000 --api-port 5001 --apdu-port 4001 apps/app_s.elf