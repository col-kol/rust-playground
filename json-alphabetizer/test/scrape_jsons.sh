#!/bin/bash
set -eo pipefail

while read line; do 
  
  if [[ -n "$line" ]]; then
    url="$line"
    wget "$url"
  fi

done <urls.txt
