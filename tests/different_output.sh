#!/bin/bash

Chance=$((RANDOM % 3))

if [ $Chance -eq 0 ]
then
    echo -e " Ahh! Ahh!\n" \
        "We come from the land of the ice and snow\n" \
        "From the midnight sun where the hot springs flow\n" \
        "- \"Immigrant Song\" by Led Zeppelin"
    exit 0
elif [ $Chance -eq 1 ]
then
    echo -e " The hammer of the gods\n" \
        "Will drive our ships to new lands\n" \
        "To fight the horde, sing and cry\n" \
        "Valhalla, I am coming\n" \
        "- \"Immigrant Song\" by Led Zeppelin"
    exit 1
else
    echo -e " On we sweep with threshing oar\n" \
        "Our only goal will be the western shore\n" \
        "- \"Immigrant Song\" by Led Zeppelin"
    exit 2
fi
