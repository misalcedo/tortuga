#!/usr/bin/env sh

echo "Status: 302"
echo "Location: ${HTTP_REDIRECT_TO-}"
echo ""

if test "${HTTP_DOCUMENT-}"
then
      echo "${HTTP_DOCUMENT-}"
fi