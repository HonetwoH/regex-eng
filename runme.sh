#!/bin/sh

printf "Please look into these issues: \n"
grep -r -C 10 "TODO" src/
printf "\nPlease look into these issues: \n"
