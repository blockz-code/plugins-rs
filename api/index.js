class Plugins {

    store = Object.create({});

    /**
     * ### get all loaded plugins
     * @returns usize
     */
    countPlugins() {
        return Object.keys(this.store).length;
    }
    
    /**
     * ### get registred plugin
     * @param string name 
     * @returns object
     */
    loadPlugin(name) {
        return this.store[name];
    }
    
    /**
     * ### register a plugin
     * @param string name 
     * @param {*} object 
     */
    registerPlugin(name, object) {
        this.store[name] = object;
    }

}

globalThis.Plugins = new Plugins();