#!/bin/sh
find . -name \*.md -type f -exec pandoc header-includes.yaml -B includes/header.html -A includes/footer.html -o {}.html {} \;
find . -depth -name '*.md.html' -execdir bash -c 'mv -i -f "$1" "${1//md.html/html}"' bash {} \;
pandoc header-includes.yaml --metadata title="Dylan Falconer's Website" --variable title="" -B includes/header.html -A includes/footer.html -o index.html index.md
