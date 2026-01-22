#!/bin/bash
set -e

# Change to the script directory
cd "$(dirname "$0")"

echo "🐍 Setting up Python Virtual Environment..."
if [ ! -d ".venv" ]; then
    python3 -m venv .venv
fi

source .venv/bin/activate

echo "📦 Installing build dependencies (maturin)..."
pip install maturin

echo "🔨 Building and installing dm2xcod..."
# Navigate to project root to run maturin
cd ../..
maturin develop
# Go back to python_demo directory
cd examples/python_demo

echo "🚀 Running demonstration..."
python demonstration.py
