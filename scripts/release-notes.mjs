import { appendFileSync, existsSync, readFileSync } from 'node:fs';

const fallback = 'See the assets to download this version and install.';
const tag = process.env.GITHUB_REF_NAME ?? '';
const version = tag.replace(/^v/, '');
let body = fallback;

if (version && existsSync('CHANGELOG.md')) {
    const lines = readFileSync('CHANGELOG.md', 'utf8').split(/\r?\n/);
    const start = lines.findIndex((line) => line.startsWith(`## [${version}]`));
    if (start !== -1) {
        const section = [];
        for (let index = start + 1; index < lines.length; index += 1) {
            if (lines[index].startsWith('## [')) break;
            section.push(lines[index]);
        }
        const notes = section.join('\n').trim();
        if (notes) body = notes;
    }
}

const envFile = process.env.GITHUB_ENV;
if (envFile) {
    appendFileSync(envFile, `RELEASE_BODY<<EOF\n${body}\nEOF\n`);
} else {
    console.log(body);
}
