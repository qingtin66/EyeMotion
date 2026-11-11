import { readFileSync } from 'node:fs';
import { join } from 'node:path';

const root = process.cwd();
const read = (p) => readFileSync(join(root, p), 'utf8');

const cargoVersion = read('Cargo.toml').match(/\[workspace\.package\][\s\S]*?version\s*=\s*"([^"]+)"/)?.[1];
const pkgVersion = JSON.parse(read('package.json')).version;
const confVersion = JSON.parse(read('src-tauri/tauri.conf.json')).version;

const versions = { 'Cargo.toml': cargoVersion, 'package.json': pkgVersion, 'tauri.conf.json': confVersion };
const unique = new Set(Object.values(versions));

if (unique.size !== 1 || [...unique][0] == null) {
    console.error('❌ 版本不一致:', versions);
    process.exit(1);
}
console.log('✅ 版本一致:', [...unique][0]);
