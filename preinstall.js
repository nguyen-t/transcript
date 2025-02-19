import { execSync } from 'node:child_process';

const WHISPER_CPP = 'https://github.com/ggerganov/whisper.cpp';
const WHISPER_DIR = 'whisper.cpp';
const BUILD_DIR = `${WHISPER_DIR}/build`;
const CMAKE_OPTIONS = [
  '',
  'CMAKE_POSITION_INDEPENDENT_CODE=ON',
  'BUILD_SHARED_LIBS=OFF',
  'GGML_CCACHE=OFF',
  'GGML_OPENMP=OFF',
  ...process.argv.slice(2),
].join(' -D').trim();

try {
  execSync(`git clone ${WHISPER_CPP}`, {
    'stdio': ['pipe', 'pipe', 'ignore'],
  });
} catch (error) {
}
try {
  execSync(`cmake ${WHISPER_DIR} -B ${BUILD_DIR} ${CMAKE_OPTIONS}`);
  execSync(`cmake --build ${BUILD_DIR} --config Release `);
} catch (error) {
  console.log(error);
}
