import { rm, rmSync } from 'node:fs';

const root = import.meta.dirname;
const options = {
  'recursive': true,
};

rmSync(`${root}/bindings`, options);
rmSync(`${root}/target`, options);
rmSync(`${root}/node_modules`, options);
rmSync(`${root}/whisper.cpp`, options);
