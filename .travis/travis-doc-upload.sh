#!/usr/bin/env sh

set -ev

[ ${TRAVIS_BRANCH} = master ]
[ ${TRAVIS_PULL_REQUEST} = false ]

echo "<meta http-equiv=refresh content=0; url=nafi_lexer/index.html>" > target/doc/index.html

mkdir -p ~/.ssh
openssl aes-256-cbc \
  -K ${encrypted_6c6066c61167_key} \
  -iv ${encrypted_6c6066c61167_iv} \
  -in .travis/travis_id_rsa.enc \
  -out ~/.ssh/id_rsa \
  -d
chmod 600 ~/.ssh/id_rsa

git clone --branch gh-pages git@github.com:${TRAVIS_REPO_SLUG} docs

cd docs
mv ../target/doc .
git add -A
git commit -m "Automatic Travis doc build"
git push origin gh-pages
