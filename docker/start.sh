#!/bin/sh
# Skills Hub 合并镜像启动脚本
# 同时启动后端服务和 nginx

set -e

echo "Starting Skills Hub..."

# 执行数据库迁移
echo "Running database migrations..."
MIGRATIONS_DIR="/app/migrations"

if [ -d "$MIGRATIONS_DIR" ] && [ -n "$DATABASE_URL" ]; then
    # 等待数据库就绪
    echo "Waiting for database..."
    max_attempts=30
    attempt=0
    while [ $attempt -lt $max_attempts ]; do
        if psql "$DATABASE_URL" -c "SELECT 1" > /dev/null 2>&1; then
            echo "Database is ready"
            break
        fi
        attempt=$((attempt + 1))
        echo "Waiting for database... ($attempt/$max_attempts)"
        sleep 1
    done

    if [ $attempt -eq $max_attempts ]; then
        echo "Warning: Database not ready after $max_attempts seconds"
    fi

    # 创建迁移记录表（幂等）
    psql "$DATABASE_URL" -c "CREATE TABLE IF NOT EXISTS _migrations (id SERIAL PRIMARY KEY, name VARCHAR(255) UNIQUE NOT NULL, executed_at TIMESTAMPTZ DEFAULT NOW());" > /dev/null 2>&1

    # 执行未执行的迁移文件（按文件名顺序）
    for migration_file in $(ls $MIGRATIONS_DIR/*.sql 2>/dev/null | sort); do
        filename=$(basename "$migration_file")

        # 检查是否已执行
        already_run=$(psql "$DATABASE_URL" -t -c "SELECT COUNT(*) FROM _migrations WHERE name = '$filename';" 2>/dev/null | tr -d ' ')

        if [ "$already_run" = "0" ] || [ -z "$already_run" ]; then
            echo "Applying migration: $filename"
            if psql "$DATABASE_URL" -f "$migration_file" > /dev/null 2>&1; then
                psql "$DATABASE_URL" -c "INSERT INTO _migrations (name) VALUES ('$filename') ON CONFLICT DO NOTHING;" > /dev/null 2>&1
                echo "Migration $filename completed"
            else
                echo "Warning: Migration $filename failed or partially applied"
            fi
        else
            echo "Skipping migration (already applied): $filename"
        fi
    done
    echo "Migrations check completed"
else
    echo "Warning: Migrations directory not found or DATABASE_URL not set, skipping migrations"
fi

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