#!/bin/sh -e

# 1.
# Requires plugin https://github.com/heroku/heroku-builds
# Install via `heroku plugins:install heroku-builds`

# 2.
# Set heroku environment variables by running following script:
# private/update-environment-variables-on-heroku.mjs

heroku builds:create -a factorio-mods-localization