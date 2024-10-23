wasm-pack build --target web  --out-dir out
rm out/package.json
cp -r out/* pkg/
sudo rm -r out
