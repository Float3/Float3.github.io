#!/usr/bin/env bash

rm -rf content/piano/debug content/piano/tuningplayground/
rm -rf tuningplayground/www tuningplayground/www-dev

cd tuningplayground

echo "building master"
./build.sh prod
rm ./www/163.bootstrap.js.LICENSE.txt | true
mv ./www ./stable

echo "building dev"
./build.sh dev
rm ./www/chords.json ./www/chord.txt | true
mv ./www ../content/piano/debug
mv ./stable ../content/piano/tuningplayground/
mv ../content/piano/tuningplayground/chords.* ../content/piano/

cd ../textprocessing
./build.sh prod
mv ./www/ ../content/textprocessing/wasm

cd ../ts 
pnpm install
pnpm exec tsc

cd ..
./scripts/collect_links.sh
./scripts/media.sh

pnpm install
npx quartz build