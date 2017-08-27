#!/usr/bin/env sh

set -ev

[ ${TRAVIS_BRANCH} = master ]
[ ${TRAVIS_PULL_REQUEST} = false ]

echo "<meta http-equiv=\"refresh\" content=\"0; url=nafi_lexer/index.html\">
<a href=\"nafi_lexer/index.html\">Redirect</a>" > target/doc/index.html

git clone --branch gh-pages git@github.com:${TRAVIS_REPO_SLUG} docs

cd docs
shopt -s extglob
rm -rf !(.git/)
shopt -u extglob
mv ../target/doc/* .
git add -A
git commit -m "Automatic Travis doc build"
git push origin gh-pages
