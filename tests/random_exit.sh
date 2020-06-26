#!/bin/bash

Chance=$(($RANDOM % 4))

if [ $Chance -eq 0 ]
then
    exit 0
else
    exit $(($RANDOM % 8))
fi
