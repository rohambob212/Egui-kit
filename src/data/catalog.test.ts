import test from 'node:test';
import assert from 'node:assert/strict';
import { WIDGET_CATALOG } from './catalog';
import { countByStatus, groupWidgetsByPart } from '../utils/catalogLogic';

test('WIDGET_CATALOG contains exactly 80 unique gpui-kit components', () => {
  assert.equal(WIDGET_CATALOG.length, 80);
  const slugs = new Set(WIDGET_CATALOG.map((w) => w.slug));
  assert.equal(slugs.size, 80);
});

test('WIDGET_CATALOG assigns exactly 10 components to each of the 8 parts', () => {
  const grouped = groupWidgetsByPart(WIDGET_CATALOG);
  for (let p = 1; p <= 8; p += 1) {
    assert.equal(grouped.get(p as 1)?.length, 10, `Part ${p} should have 10 widgets`);
  }
});

test('Parts 1 to 6 have all 60 components marked ready and Parts 7-8 planned', () => {
  const counts = countByStatus(WIDGET_CATALOG);
  assert.deepEqual(counts, { ready: 60, planned: 20, total: 80 });
  const readyParts = WIDGET_CATALOG.filter((w) => w.part >= 1 && w.part <= 6);
  assert.equal(readyParts.length, 60);
  assert.ok(readyParts.every((w) => w.status === 'ready'));
});
