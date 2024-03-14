#!/bin/bash

echo "Creating User ${UID:-1000}"
useradd -m -u "${UID:-1000}" hoard || exit 1

chown -R hoard /download
mkdir /.cache && chown -R hoard /.cache
chown -R hoard /data

su hoard -c /hoard
