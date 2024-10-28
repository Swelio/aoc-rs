#!/usr/bin/env sh
set -eu

USERNAME="${USERNAME:-"${_REMOTE_USER}"}"

/usr/bin/install --directory \
    --owner $USERNAME \
    --group $USERNAME \
    --mode 0700 \
    /home/${USERNAME}/.gnupg
