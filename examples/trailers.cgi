#!/usr/bin/env sh

echo "Status: ${HTTP_STATUS-204}"
echo "Content-Type: text/html"
echo "X-CGI-Test: missing"
echo ""
