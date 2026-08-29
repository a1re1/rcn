"""Emit a Rust `TwPage` skeleton from a scraped docs JSON: reference rows and
examples (title, prose, snippet) filled in; `demo:` left as a TODO node.
Usage: gen_page.py <slug> <SECTION_VARIANT> [CONST_NAME]"""
import json, sys, os, re
slug, section = sys.argv[1], sys.argv[2]
name = sys.argv[3] if len(sys.argv) > 3 else slug.replace('-', '_').upper()
S = os.environ["TW_DOCS_DIR"]
d = json.load(open(f"{S}/{slug}.json"))
def rs(s): return '"' + s.replace('\\', '\\\\').replace('"', '\\"') + '"'
def rawstr(s):
    return 'r#"' + s + '"#' if '"#' not in s else rs(s)
title = ' / '.join(w.capitalize() for w in d['title'].split(' / ')) if '/' in d['title'] else d['title'][:1].upper() + d['title'][1:]
out = []
out.append(f"/// <https://tailwindcss.com/docs/{slug}>")
out.append(f"pub static {name}: TwPage = TwPage {{")
out.append(f"    slug: {rs(slug)},")
out.append(f"    title: {rs(title)},")
out.append(f"    section: TwSection::{section},")
out.append(f"    description: {rs(d['desc'] or '')},")
out.append("    reference: &[")
for row in d['rows']:
    if len(row) >= 2: out.append(f"        ({rs(row[0])}, {rs(row[1])}),")
out.append("    ],")
out.append("    examples: &[")
for e in d['examples']:
    snippet = '\n'.join(c.rstrip() for c in e['code']) if e['code'] else ''
    # docs snippets collapse newlines; restore one element per line
    snippet = re.sub(r'>\s*<', '>\n<', snippet)
    out.append("        TwExample {")
    out.append(f"            title: {rs(e['title'])},")
    out.append("            prose: &[")
    for p in e['prose']: out.append(f"                {rs(p)},")
    out.append("            ],")
    out.append(f"            snippet: {rawstr(snippet)},")
    out.append("            demo: TODO_DEMO,")
    out.append("        },")
out.append("    ],")
out.append("};")
print('\n'.join(out))
