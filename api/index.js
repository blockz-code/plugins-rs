globalThis.window.storePlugins = {};
globalThis.window.countPlugins = () => Object.keys(window.storePlugins).length;
globalThis.window.loadPlugin   = (name) => window.storePlugins[name];
globalThis.window.registerPlugin = (name, object) => window.storePlugins[name] = object;