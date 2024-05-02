#!/bin/bash

# Check if the user already exists
if id hoard &>/dev/null; then
    echo "User hoard already exists."
else
    # Create the user
    echo "Creating User ${UID:-1000}"
    useradd -m -u "${UID:-1000}" hoard || exit 1
fi

# Perform other setup tasks
chown -R hoard /download
mkdir -p /.cache && chown -R hoard /.cache
chown -R hoard /data

# Start the application as the user
su hoard -c /hoard