# 构建阶段
FROM node:20-alpine AS builder

# 配置 Alpine 阿里云镜像
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

# 配置 npm 淘宝镜像
RUN npm config set registry https://registry.npmmirror.com

WORKDIR /app

# 复制依赖文件
COPY package.json ./

# 安装依赖
RUN npm install

# 复制源代码
COPY . .

# 构建
RUN npm run build

# 运行阶段
FROM nginx:alpine

# 配置 Alpine 阿里云镜像
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

# 复制构建产物
COPY --from=builder /app/dist /usr/share/nginx/html

# 复制 nginx 配置
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]