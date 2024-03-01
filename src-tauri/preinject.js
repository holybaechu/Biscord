window.VCDP = {};

function addPatch(p) {
    const { patches, ...globals } = p;

    for (const patch of patches) {
        if (!Array.isArray(patch.replacement)) patch.replacement = [patch.replacement];
        for (const r of patch.replacement) {
            if (typeof r.replace === "string") r.replace = r.replace.replaceAll("$self", "VCDP");
        }

        patch.plugin = "Vesktop";
        Vencord.Plugins.patches.push(patch);
    }

    Object.assign(VCDP, globals);
}

function isJson(str) {
    let json
    try {
        json = JSON.parse(str);
    } catch (e) {
        return false;
    }
    return json;
}

function waitForElement(selector) {
    return new Promise((resolve) => {
        const interval = setInterval(() => {
            const element = document.querySelector(selector);
            if (element) {
                clearInterval(interval);
                resolve(element);
            }
        }, 0);
    });
}

;(async () => {
    while (!window.__TAURI__) await new Promise(r => setTimeout(r, 0));

    const { invoke, http } = window.__TAURI__;

    invoke("get_mods_css").then(css => {
        const style = document.createElement("style");
        style.innerHTML = css;
        document.head.append(style);
    })

    const originalFetch = window.fetch;
    window.fetch = async (url, options) => {
        if (url.toString().match(/https?:\/\/(?:[a-z]+\.)?(?:discord\.com|discordapp\.com)(?:\/.*)?/g)) {
            if (url.toString().match(/\/api\/v.*\/(science|track)/g)) {
                return;
            }
            originalFetch(url, options);
        }

        if (options?.body) {
            const bodyContent = isJson(String(options.body)) ? http.Body.json(options.body) : typeof options.body === 'string' ? http.Body.text(options.body) : http.Body.bytes(options.body)
            options.body = bodyContent
        }

        if (options?.headers) {
            if (options.headers instanceof Headers) {
                const headers = {}
                for (const [k, v] of options.headers.entries()) {
                    headers[key] = value
                }
                options.headers = headers
            }
        }

        const response = await http.fetch(url, {
            responseType: 2,
            ...options
        })

        response.json = async () => JSON.parse(response.data)
        response.text = async () => response.data

        response.headers = new Headers(response.headers)

        return response
    }

    addPatch({
        patches: [
            {
                find: ".wordmarkWindows",
                replacement: [
                    {
                        match: /case \i\.\i\.WINDOWS:/,
                        replace: 'case "WEB":'
                    },
                    ...["close", "minimize", "maximize"].map(op => ({
                        match: new RegExp(String.raw`\i\.\i\.${op}\b`),
                        replace: `window.__TAURI__.invoke("${op}")`
                    }))
                ]
            }
        ]
    });

    // Title bar drag
    await waitForElement("#app-mount div > [aria-label]");
    document.querySelector("#app-mount div > [aria-label]").parentElement.setAttribute("data-tauri-drag-region", true)

    invoke("loaded")

    console.log("Loaded Biscord!")
})();