#!/usr/bin/env sh

echo "Location: ${HTTP_REDIRECT_TO-}"
echo "Foo: test"
echo ""

if test "${HTTP_DOCUMENT-}"
then
      echo "${HTTP_DOCUMENT-}"
fi