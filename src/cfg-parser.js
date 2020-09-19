import fs from 'fs';

export function escapeStringsIfNeeded(filePath) {
    const content = fs.readFileSync(filePath, 'utf8');
    const contentEscaped = content.replace(/(?<=^[^[][^=]*=)(.*)$/gmu, match => {
        if (!match.includes('"') && !match.includes(';')) return match;
        if (match[0] === '"' && match[match.length - 1] === '"') return match;
        return `"${match}"`;
    });
    fs.writeFileSync(filePath, contentEscaped);
}
