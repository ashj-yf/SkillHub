#!/bin/sh
# Skills Hub 合并镜像启动脚本
# 同时启动后端服务和 nginx

set -e

echo "Starting Skills Hub..."

# 启动后端服务（后台运行）
echo "Starting backend service..."
./skillhub-backend &
BACKEND_PID=$!

# 等待后端启动
sleep 2

# 检查后端是否正常运行
if ! kill -0 $BACKEND_PID 2>/dev/null; then
    echo "Failed to start backend service"
    exit 1
fi

echo "Backend started (PID: $BACKEND_PID)"

# 启动 nginx（前台运行）
echo "Starting nginx..."
nginx -g "daemon off;" &
NGINX_PID=$!

# 等待 nginx 启动
sleep 1

# 检查 nginx 是否正常运行
if ! kill -0 $NGINX_PID 2>/dev/null; then
    echo "Failed to start nginx"
    kill $BACKEND_PID 2>/dev/null || true
    exit 1
fi

echo "Nginx started (PID: $NGINX_PID)"
echo "Skills Hub is ready on port 80"

# 信号处理 - 优雅关闭
trap 'echo "Shutting down..."; kill $BACKEND_PID $NGINX_PID 2>/dev/null; exit 0' SIGTERM SIGINT

# 等待任一进程退出
wait -n $BACKEND_PID $NGINX_PID

# 如果任一进程退出，关闭另一个
kill $BACKEND_PID $NGINX_PID 2>/dev/null || true

echo "Skills Hub stopped"