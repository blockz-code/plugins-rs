export namespace dirs {
    function audio_dir(): any;
    function cache_dir(): any;
    function config_dir(): any;
    function config_local_dir(): any;
    function data_dir(): any;
    function data_local_dir(): any;
    function desktop_dir(): any;
    function document_dir(): any;
    function download_dir(): any;
    function home_dir(): any;
    function picture_dir(): any;
    function video_dir(): any;
}
export namespace utils {
    function which(file_path: any): any;
    function log(url: any, level: any, message: any): void;
}
export namespace ids {
    function nid(value: any): any;
    function nid_custom(value: any, alphabet: any): any;
    function nid_safe(value: any): any;
    function uuid(): any;
}
