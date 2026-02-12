#!/bin/bash

# 定义变量
PROBING_LIST_CMD="probing list"
PROBING_CONFIG_CMD="probing"
SERVER_ADDRESS="10.107.204.71"
START_PORT=12347

# 执行 probing list 命令并提取进程 PID
output=$($PROBING_LIST_CMD)
pids=$(echo "$output" | awk '/Processes with injected probes:/ {flag=1; next} flag {print $1}' | tr -d ':')

# 检查是否提取到 PID
if [[ -z "$pids" ]]; then
    echo "No processes with injected probes found."
    exit 1
fi

# 遍历每个 PID 并运行 probing config 命令
port=$START_PORT
for pid in $pids; do
    echo "Configuring probing for PID $pid with port $port"
    $PROBING_CONFIG_CMD -t $pid config "probing.server.address='$SERVER_ADDRESS:$port'"
    port=$((port + 1))
done

echo "Probing configuration completed."
