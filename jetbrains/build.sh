#!/bin/sh
set -e
cd "$(dirname "$0")"
rm -f warm-burnout-theme.jar
zip -r warm-burnout-theme.jar \
  META-INF/ \
  "Warm Burnout Dark.theme.json" \
  "Warm Burnout Light.theme.json" \
  "Warm Burnout Islands Dark.theme.json" \
  "Warm Burnout Islands Light.theme.json" \
  "Warm Burnout Dark.icls" \
  "Warm Burnout Light.icls"
echo "Built: jetbrains/warm-burnout-theme.jar"
