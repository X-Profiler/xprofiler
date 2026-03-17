"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.transform = void 0;
const path_1 = __importDefault(require("path"));
const parser_1 = require("@babel/parser");
const generator_1 = __importDefault(require("@babel/generator"));
const traverse_1 = __importDefault(require("@babel/traverse"));
const visitor_1 = require("./visitor");
/**
 * inject line, column, relative-path to JSX html data attribute in source code
 *
 * @type webpack.loader.Loader
 * ref: https://astexplorer.net  +  @babel/parser
 */
const transform = ({ rootPath = process.cwd(), filePath, sourceCode, options, }) => {
    /**
     * example:
     * rootPath: /home/xxx/project
     * filePath: /home/xxx/project/src/ooo/xxx.js
     * relativePath: src/ooo/xxx.js
     */
    const relativePath = path_1.default.relative(rootPath, filePath);
    const isSkip = (0, visitor_1.pathMatch)(filePath, options?.excludes);
    if (isSkip) {
        return sourceCode;
    }
    const ast = (0, parser_1.parse)(sourceCode, {
        sourceType: 'module',
        allowUndeclaredExports: true,
        allowImportExportEverywhere: true,
        plugins: [
            'typescript',
            'jsx',
            'decorators-legacy',
            'classProperties',
            ...options?.babelPlugins ?? [],
        ],
        ...options?.babelOptions,
    });
    /**
     * astexplorer + @babel/parser
     * https://astexplorer.net
     */
    (0, traverse_1.default)(ast, {
        JSXOpeningElement: {
            enter(path) {
                (0, visitor_1.doJSXOpeningElement)(path.node, { relativePath, cwd: rootPath, path });
            },
        },
    });
    const { code, } = (0, generator_1.default)(ast, {
        decoratorsBeforeExport: true,
    });
    return code;
};
exports.transform = transform;
