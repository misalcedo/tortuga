#!/usr/bin/env sh

echo "Status: 302"
echo "Content-Type: text/html"
echo "Location: ${HTTP_REDIRECT_TO-}"
echo ""

if test "${HTTP_DOCUMENT-}"
then
      echo "${HTTP_DOCUMENT-}"
fi