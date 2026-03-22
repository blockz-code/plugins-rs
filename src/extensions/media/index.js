
export function instance() {
  Deno.core.ops.instance();
}

export function install(cb) {
  return Deno.core.ops.install(cb);
}
