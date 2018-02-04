#!/usr/bin/env bash

for i in {1..1000}; do
  echo -e "some.metric|s|$i" | nc -w 0 -u 0.0.0.0 55123
done
