#!/bin/bash
set -e

echo $(/usr/local/bin/scs key generate-node-key --base-path $BASE_PATH)

echo "初始配置设置成功！"