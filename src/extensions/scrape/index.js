export function create_scraper() {
  Deno.core.ops.create_scraper();
}

export function load_document(url, path) {
  Deno.core.ops.load_document(url, path);
}

export function get_element(selectors) {
  return Deno.core.ops.get_element(selectors);
}

export function get_element_with_attr(selectors, attr) {
  return Deno.core.ops.get_element_with_attr(selectors, attr);
}