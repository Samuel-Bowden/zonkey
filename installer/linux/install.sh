#!/bin/bash

# Installer for Zonkey on Linux machines
# Please run this script inside the installer directory

echo "Installing Zonkey"

cp zonkey-browser.desktop ~/.local/share/applications/zonkey-browser.desktop
check1=$?
sudo chmod +x zonkey
check2=$?
sudo cp zonkey /usr/local/bin/zonkey
check3=$?

check=$((check1 + check2 + check3))

if [ "$check" -eq 0 ]; then
    echo "Install Successful. You can use 'zonkey' to start programming, or find the shortcut to open the browser in your home menu."
else
    echo "Install Failed. Please check the failed commands."
fi
