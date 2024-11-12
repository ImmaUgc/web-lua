import weblua, { execute_lua } from './pkg/weblua.js';

class LuaScript extends HTMLElement {
    constructor() {
        super();
        this.lua_code = this.innerHTML.trim();
        this.remove();
    }

    connectedCallback() {
        const src = this.getAttribute('src');

        if (src) {
            fetch(src)
            .then(response => response.text())
            .then(lua_code => {
                this.lua_code = lua_code;
                this.init();
            })
            .catch(err => console.error('Erro ao carregar o arquivo Lua:', err));
        } else {
            this.init();
        }
    }

    async init() {
        await weblua();
        execute_lua(this.lua_code);
    }
}

customElements.define("lua-script", LuaScript);