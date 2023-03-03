const cpy_mod = import("cpy");
const { lessLoader } = require("esbuild-plugin-less");
const { WasmPlugin } = require("@finos/perspective-esbuild-plugin/wasm");
const { WorkerPlugin } = require("@finos/perspective-esbuild-plugin/worker");
const { AMDLoader } = require("@finos/perspective-esbuild-plugin/amd");
const { UMDLoader } = require("@finos/perspective-esbuild-plugin/umd");
const { ReplacePlugin } = require("@finos/perspective-esbuild-plugin/replace");
const { build } = require("@finos/perspective-esbuild-plugin/build");
const path = require("path");

const NBEXTENSION_PATH = path.resolve(
    __dirname,
    "..",
    "..",
    "python",
    "perspective",
    "perspective",
    "nbextension",
    "static"
);

const THEMES_BUILD = {
    entryPoints: ["src/less/index.less"],
    plugins: [WasmPlugin(false), lessLoader()],
    outdir: "dist/css",
};

const TEST_BUILD = {
    entryPoints: ["src/js/psp_widget.js"],
    define: {
        global: "window",
    },
    plugins: [WasmPlugin(true), WorkerPlugin({ inline: true }), UMDLoader()],
    globalName: "PerspectiveLumino",
    format: "cjs",
    loader: {
        ".html": "text",
        ".ttf": "file",
        ".css": "text",
    },
    outfile: "dist/umd/lumino.js",
};

const LAB_BUILD = {
    entryPoints: ["src/js/index.js"],
    define: {
        global: "window",
    },
    plugins: [
        lessLoader(),
        WasmPlugin(true), 
        WorkerPlugin({ inline: true }),

        // See note in `rust/perspective-viewer/build.js`
        ReplacePlugin(/["']perspective_viewer_bg\.wasm["']/, "undefined"),
    ],
    external: ["@jupyter*", "@lumino*"],
    format: "esm",
    loader: {
        ".css": "text",
        ".html": "text",
        ".ttf": "file",
    },
    outfile: "dist/umd/perspective-jupyterlab.js",
};

const NB_BUILDS = [
    {
        entryPoints: ["src/js/notebook/extension.js"],
        define: {
            global: "window",
        },
        plugins: [WasmPlugin(true), WorkerPlugin(true), AMDLoader([])],
        loader: {
            ".ttf": "file",
            ".css": "text",
        },
        external: ["@jupyter*", "@lumino*"],
        format: "cjs",
        outfile: path.join(NBEXTENSION_PATH, "extension.js"),
    },
    {
        entryPoints: ["src/js/notebook/index.js"],
        define: {
            global: "window",
        },
        plugins: [
            WasmPlugin(true),
            WorkerPlugin(true),
            AMDLoader([`@jupyter-widgets/base`]),
        ],
        external: ["@jupyter*"],
        format: "cjs",
        loader: {
            ".ttf": "file",
        },
        outfile: path.join(NBEXTENSION_PATH, "index.js"),
    },
];

const { BuildCss } = require("@prospective.co/procss/target/cjs/procss.js");
const fs = require("fs");

function add(builder, path, path2) {
    builder.add(
        path,
        fs.readFileSync(require.resolve(path2 || path)).toString()
    );
}

const PROD_BUILD = [LAB_BUILD, ...NB_BUILDS];
const BUILD = process.argv.some((x) => x == "--test") ? TEST_BUILD : PROD_BUILD;

async function build_all() {
    const { default: cpy } = await cpy_mod;
    fs.mkdirSync("dist/css", { recursive: true });
    const builder3 = new BuildCss("");

    add(builder3, "@finos/perspective-viewer/dist/css/themes.css");
    add(builder3, "./index.less", "./src/less/index.less");
    fs.writeFileSync(
        "dist/css/perspective-jupyterlab.css",
        builder3.compile().get("index.css")
    );

    await build(THEMES_BUILD);
    await Promise.all(BUILD.map(build)).catch(() => process.exit(1));
    cpy(["dist/css/*"], "dist/umd");
    cpy(["src/less/*"], "dist/less");
}

build_all();
