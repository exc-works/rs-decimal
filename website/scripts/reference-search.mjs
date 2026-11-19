// Sort the exact API path ahead of a short-name match, then localized results.
export function searchReference(items, query, labels) {
  const term = query.trim().toLowerCase();
  if (!term) return items;
  function rank(item) {
    const id = item.id.toLowerCase();
    if (id === term) return 0;
    if (id.split(/::|\./).at(-1) === term) return 1;
    if (id.includes(term)) return 2;
    if (labels[item.operation]?.toLowerCase().includes(term)) return 3;
    if (item.task?.toLowerCase().includes(term)) return 3;
    if (item.summary.toLowerCase().includes(term) || labels[item.group].toLowerCase().includes(term)) return 4;
    return Infinity;
  }
  return items.map(item => ({ item, rank: rank(item) }))
    .filter(hit => Number.isFinite(hit.rank))
    .sort((a, b) => a.rank - b.rank)
    .map(hit => hit.item);
}
