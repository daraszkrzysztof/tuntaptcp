#!/usr/bin/env bash

cargo b --release

sudo setcap cap_net_admin=eip target/release/tuntaptcp
target/release/tuntaptcp &
pid=$!
sudo ip addr add 192.168.0.1/24 dev mytun
sudo ip link set up dev mytun

trap "exit" INT TERM
trap "kill 0" EXIT
wait $pid