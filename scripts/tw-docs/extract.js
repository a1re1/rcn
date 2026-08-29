(() => {
  const main = document.querySelector('main') || document.body;
  const h1 = main.querySelector('h1');
  const title = h1 ? h1.textContent.trim() : null;
  let desc = null;
  if (h1) { let e = h1.nextElementSibling; while (e && !desc) { if (e.tagName === 'P') desc = e.textContent.trim(); e = e.nextElementSibling; } }
  const rows = [...main.querySelectorAll('table tbody tr')].map(r => [...r.children].map(c => c.textContent.trim()));
  const examples = [];
  const all = [...main.querySelectorAll('h2, h3, h4, p, pre')];
  let inExamples = false, cur = null;
  for (const el of all) {
    if (el.tagName === 'H2') { inExamples = /^Examples$/i.test(el.textContent.trim()); if (!inExamples) cur = null; continue; }
    if (!inExamples) continue;
    if (el.tagName === 'H3') { cur = { title: el.textContent.trim(), prose: [], code: [] }; examples.push(cur); continue; }
    if (!cur) continue;
    if (el.tagName === 'P') cur.prose.push(el.textContent.trim());
    if (el.tagName === 'PRE') cur.code.push(el.textContent);
  }
  return JSON.stringify({ url: location.pathname, title, desc, rows, examples });
})()
