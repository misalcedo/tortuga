#!/usr/bin/env sh

echo "Location: ${HTTP_REDIRECT_TO-}"

if test "${HTTP_DOCUMENT-}"
then
  echo "Status: 302"
  echo "Content-Type: text/html"
  echo ""
  echo "${HTTP_DOCUMENT-}"
else
  echo ""
fi