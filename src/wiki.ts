export function pageName(raw: string): string | null {
  const name = raw.replace(/^\p{White_Space}+|\p{White_Space}+$/gu, '');
  return name && !/[\[\]\r\n\u2028\u2029]/u.test(name) ? name : null;
}

export function pageLinks(body: string): string[] {
  const links = new Set<string>();
  for (const match of body.matchAll(/\[\[([^\[\]\r\n\u2028\u2029]+)\]\]/gu)) {
    const name = pageName(match[1]);
    if (name !== null) links.add(name);
  }
  return [...links];
}
