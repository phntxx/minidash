#!/bin/sh
set -eu

# This script checks if the default needed files are
# available at the specified location (defaults specified via env-vars)
# If they're not, it copies the default files there.

if [[ ! -f $CONFIG_FILE ]]; then
  __DIRNAME="$(dirname $CONFIG_FILE)"
  mkdir -p $__DIRNAME
  mv defaults/config.yml $CONFIG_FILE
else
  rm -rf defaults/config.yml
fi

if [[ ! -f $TEMPLATE_FILE ]]; then
  __DIRNAME="$(dirname $TEMPLATE_FILE)"
  mkdir -p $__DIRNAME
  mv defaults/template.hbs $TEMPLATE_FILE
else
  rm -rf defaults/template.hbs
fi

if [[ ! -f $STATIC_PATH ]]; then
  mkdir -p "$STATIC_PATH"
  mv defaults/static/* $STATIC_PATH
else
  rm -rf defaults/static
fi

if [[ ! "$(ls -A defaults)" ]]; then
  rm -rf defaults
fi

"$@"
