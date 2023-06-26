#!/bin/bash

# Uninstaller for Zonkey on Linux machines

echo "Uninstalling Zonkey"

rm ~/.local/share/applications/zonkey-browser.desktop
check1=$?
sudo rm /usr/local/bin/zonkey
check2=$?
rm -rf ~/.local/share/zonkey/
check3=$?

check=$((check1 + check2 + check3))

if [ "$check" -eq 0 ]; then
    echo "Uninstall Successful."
else
    echo "Uninstall Failed. Please check the failed commands."
fi
