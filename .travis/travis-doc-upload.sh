#!/usr/bin/env sh

set -ev

[ ${TRAVIS_BRANCH} = master ]
[ ${TRAVIS_PULL_REQUEST} = false ]

git clone --branch gh-pages git@github.com:${TRAVIS_REPO_SLUG} docs
cd docs

find -maxdepth 1 -not -name "index.html" -not -name ".git" | xargs -verbose rm -vr
mv -f ../target/doc/* .

git add -vA
git commit -vm "Automatic Travis doc build"
git push origin gh-pages
