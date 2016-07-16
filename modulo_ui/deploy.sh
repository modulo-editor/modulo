#!/bin/bash

set -o errexit -o nounset

if [ "$TRAVIS_PULL_REQUEST" != "false" -o "$TRAVIS_BRANCH" != "master" ]
then
  echo "This commit was made against the $TRAVIS_BRANCH and not the master or this is a pull request! No deploy!"
  exit 0
fi

cargo doc

rev=$(git rev-parse --short HEAD)

cd target/doc

git init
git config user.name "Connor Brewster"
git config user.email "connor.brewster@eagles.oc.edu"

git remote add upstream "https://$GH_TOKEN@github.com/modulo-editor/modulo.git"
git fetch upstream
git reset upstream/gh-pages

touch .

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -qf upstream HEAD:gh-pages
