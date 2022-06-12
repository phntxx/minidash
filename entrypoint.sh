#!/bin/sh

# This script checks if the default needed files are
# available at the specified location (defaults specified via env-vars)
# If they're not, it copies the default files there.

if [[ ! -f $CONFIG_FILE ]]; then
  cp defaults/template.hbs $CONFIG_FILE
fi

if [[ ! -f $TEMPLATE_FILE ]]; then
  cp defaults/config.yml $TEMPLATE_FILE
fi

./minidash