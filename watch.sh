#!/bin/sh
while sleep 0.1; do ls ./*.md | entr -d ./build.sh; done
