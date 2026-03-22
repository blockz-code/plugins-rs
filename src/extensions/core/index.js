globalThis.window = {
  config : () => {
    return Deno.core.ops.config();
  }
};

export const dirs = {
  audio_dir : () => {
    return Deno.core.ops.audio_dir().data;
  },
  cache_dir : () => {
    return Deno.core.ops.cache_dir().data;
  },
  config_dir : () => {
    return Deno.core.ops.config_dir().data;
  },
  config_local_dir : () => {
    return Deno.core.ops.config_local_dir().data;
  },
  data_dir : () => {
    return Deno.core.ops.data_dir().data;
  },
  data_local_dir : () => {
    return Deno.core.ops.data_local_dir().data;
  },
  desktop_dir : () => {
    return Deno.core.ops.desktop_dir().data;
  },
  document_dir : () => {
    return Deno.core.ops.document_dir().data;
  },
  download_dir : () => {
    return Deno.core.ops.download_dir().data;
  },
  home_dir : () => {
    return Deno.core.ops.home_dir().data;
  },
  picture_dir : () => {
    return Deno.core.ops.picture_dir().data;
  },
  video_dir : () => {
    return Deno.core.ops.video_dir().data;
  }
}


export const utils = {
  which : (file_path) => {
    return Deno.core.ops.js_which(file_path).data;
  },
  log : (url, level, message) => {
    Deno.core.ops.op_internal_log(url, level, message);
  },
}


export const ids = {
  nid : (value) => {
    return Deno.core.ops.nid(value).data;
  },
  nid_custom : (value, alphabet) => {
    return Deno.core.ops.nid(value, alphabet).data;
  },
  nid_safe : (value) => {
    return Deno.core.ops.nid(value).data;
  },
  uuid : () => {
    return Deno.core.ops.uid().data;
  }
}