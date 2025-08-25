#!/bin/bash

# 构建 Web Page Example
# 此脚本使用 wasm-pack 将 Rust 代码编译为 WebAssembly

set -e

echo "🚀 开始构建 Ruled Router Web Demo..."

# 检查 wasm-pack 是否已安装
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ 错误: wasm-pack 未安装"
    echo "请运行以下命令安装 wasm-pack:"
    echo "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# 进入 web-page-example 目录
cd "$(dirname "$0")"

echo "📦 使用 wasm-pack 构建 WebAssembly..."

# 使用 wasm-pack 构建，目标为 web
wasm-pack build --target web --out-dir pkg --dev

echo "✅ 构建完成!"
echo ""
echo "🌐 启动开发服务器:"
echo ""
echo "方法 1 - 使用 Python (推荐):"
echo "  python3 -m http.server 8000"
echo ""
echo "方法 2 - 使用 Node.js:"
echo "  npx http-server -p 8000 -c-1"
echo ""
echo "方法 3 - 使用 Rust miniserve:"
echo "  cargo install miniserve"
echo "  miniserve . -p 8000"
echo ""
echo "然后在浏览器中打开: http://localhost:8000"
echo ""
echo "🎯 功能说明:"
echo "  ✅ 路由解析和格式化"
echo "  ✅ DOM 事件监听"
echo "  ✅ History API 导航"
echo "  ✅ 查询参数支持"
echo "  ✅ 浏览器前进/后退支持"
