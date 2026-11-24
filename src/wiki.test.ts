import assert from 'node:assert/strict';
import { test } from 'node:test';
import { pageLinks, pageName } from './wiki.ts';

test('lists unique targets in reading order without treating malformed markup as links', () => {
  assert.deepEqual(pageLinks('[[ 日記 ]] [[予定]] [[日記]] [[]] [[改\n行]] [[未閉じ [[次]]'), ['日記', '予定', '次']);
});

test('normalizes valid page names without folding case', () => {
  assert.equal(pageName('  Plan  '), 'Plan');
  assert.equal(pageName(' '), null);
  assert.equal(pageName('a[b]'), null);
  assert.equal(pageName('a\nb'), null);
});
