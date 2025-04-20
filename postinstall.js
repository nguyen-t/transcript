const { execSync } = require('node:child_process');
const { rmSync } = require('node:fs');

const root = import.meta.dirname;
const options = {
  'recursive': true,
};

execSync('npx @napi-rs/cli build --platform --release');
rmSync(`${root}/bindings`, options);
rmSync(`${root}/target`, options);
rmSync(`${root}/whisper.cpp`, options);
