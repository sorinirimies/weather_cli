#!/bin/bash

# Test script for Weather Man TUI functionality

set -e

echo "🧪 Testing Weather Man TUI Charts"
echo "=================================="

# Build the project first
echo "🔨 Building project..."
cargo build --release

# Test 1: Current weather with charts (should show automatically)
echo ""
echo "📋 Test 1: Current weather with automatic charts"
echo "This should show weather info then transition to charts automatically"
echo "Press 'q' to exit charts when they appear"
echo ""
read -p "Press Enter to start test 1..."

./target/release/weather_man

echo ""
echo "✅ Test 1 completed"

# Test 2: Charts disabled
echo ""
echo "📋 Test 2: Weather with charts disabled"
echo "This should only show text output without charts"
echo ""
read -p "Press Enter to start test 2..."

./target/release/weather_man --no-charts

echo ""
echo "✅ Test 2 completed"

# Test 3: Daily forecast with charts
echo ""
echo "📋 Test 3: Daily forecast with charts"
echo "This should show daily forecast then charts"
echo ""
read -p "Press Enter to start test 3..."

./target/release/weather_man --mode daily

echo ""
echo "✅ Test 3 completed"

# Test 4: Interactive charts mode directly
echo ""
echo "📋 Test 4: Direct charts mode"
echo "This should go directly to charts"
echo ""
read -p "Press Enter to start test 4..."

./target/release/weather_man --mode charts

echo ""
echo "✅ All tests completed!"
echo ""
echo "🎯 Expected behavior:"
echo "- Tests 1, 3, 4 should show interactive charts with 5 tabs"
echo "- Charts should have: Hourly Temp, Hourly Precip, Daily Temp, Daily Precip, Calendar"
echo "- Use arrow keys/1-5 to switch tabs, 'q' to exit"
echo "- Test 2 should only show text output"