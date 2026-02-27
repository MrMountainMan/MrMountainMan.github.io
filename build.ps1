dx bundle --release --out-dir docs
rm docs/assets -r
rm docs/404.html
rm docs/index.html
mv docs/public/* docs
cp docs/index.html docs/404.html