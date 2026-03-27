//import * as format_js from "jsr:@ymd-h/format-js";

import do_action from "./dos/do_action.ts";


globalThis.Plugins.registerPlugin("base", {
    name: "Base Test Plugin",
    exec: function () {
        return Date.now();
    },
    demo: CustomApi,
    do_action
});