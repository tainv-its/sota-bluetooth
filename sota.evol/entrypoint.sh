#!/bin/sh

cp /etc/local.d/-63ecac6a.rsa.pub /etc/apk/keys/
apk update
apk add /etc/local.d/sota-bluetooth-0.0.5-r0.apk

cp /etc/local.d/bluetooth.conf.conf /etc/bluetooth/main.conf
rc-service bluetooth restart

bluetoothctl power yes
bluetoothctl --agent NoInputNoOutput