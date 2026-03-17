import path from 'path';
import { parse, } from '@babel/parser';
import generate from '@babel/generator';
import traverse from '@babel/traverse';
import { pathMatch, doJSXOpeningElement, } from './visitor';
/**
 * inject line, column, relative-path to JSX html data attribute in source code
 *
 * @type webpack.loader.Loader
 * ref: https://astexplorer.net  +  @babel/parser
 */
export const transform = ({ rootPath = process.cwd(), filePath, sourceCode, options, }) => {
    /**
     * example:
     * rootPath: /home/xxx/project
     * filePath: /home/xxx/project/src/ooo/xxx.js
     * relativePath: src/ooo/xxx.js
     */
    const relativePath = path.relative(rootPath, filePath);
    const isSkip = pathMatch(filePath, options?.excludes);
    if (isSkip) {
        return sourceCode;
    }
    const ast = parse(sourceCode, {
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
    traverse(ast, {
        JSXOpeningElement: {
            enter(path) {
                doJSXOpeningElement(path.node, { relativePath, cwd: rootPath, path });
            },
        },
    });
    const { code, } = generate(ast, {
        decoratorsBeforeExport: true,
    });
    return code;
};
