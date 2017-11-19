#!/usr/bin/env sh

set -ev

[ ${TRAVIS_BRANCH} = master ]
[ ${TRAVIS_PULL_REQUEST} = false ]

git clone --branch gh-pages git@github.com:${TRAVIS_REPO_SLUG} docs
cd docs

ls | grep --invert-match "index.html" | xargs --verbose rm -r
mv -f ../target/doc/* .

git add -vA
git commit -vm "Automatic Travis doc build"
git push origin gh-pages
