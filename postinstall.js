import { execSync } from 'node:child_process';
import { rmSync } from 'node:fs';

const root = import.meta.dirname;
const options = {
  'recursive': true,
};

execSync('npm run build');
rmSync(`${root}/bindings`, options);
rmSync(`${root}/target`, options);
rmSync(`${root}/whisper.cpp`, options);
rmSync(`${root}/node_modules`, options);
