#!/usr/bin/env bash
docker run --rm -it -v $(pwd)/build/output:/speculos/apps --publish 41000:41000 --publish 5001:5001 --publish 4001:4001 --entrypoint /bin/bash speculos
./speculos.py --display headless --vnc-port 41000 --api-port 5001 --apdu-port 4001 -d apps/app_s.elf & ./tools/debug.sh apps/app_s.elf
echo 'FF010000190680000a55c741e9c9c47a6028800000008000000000000000' | LEDGER_PROXY_ADDRESS=127.0.0.1 LEDGER_PROXY_PORT=4001 ledgerctl send -