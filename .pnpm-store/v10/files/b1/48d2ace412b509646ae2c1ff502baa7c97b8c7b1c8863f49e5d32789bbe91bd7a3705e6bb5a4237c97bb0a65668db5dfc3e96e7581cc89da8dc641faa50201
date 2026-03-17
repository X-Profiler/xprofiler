import { type ParserPlugin, type ParserOptions } from '@babel/parser';
export interface TransformOptions {
    /** patterns to exclude matched files */
    excludes?: (string | RegExp)[];
    babelPlugins?: ParserPlugin[];
    babelOptions?: ParserOptions;
}
/**
 * inject line, column, relative-path to JSX html data attribute in source code
 *
 * @type webpack.loader.Loader
 * ref: https://astexplorer.net  +  @babel/parser
 */
export declare const transform: ({ rootPath, filePath, sourceCode, options, }: {
    /**
     * path for project root path
     * @default process.cwd()
     */
    rootPath?: string | undefined;
    /**
     * path of source file
     * MUST keep consistent relative or absolute with `rootPath`
     */
    filePath: string;
    sourceCode: string;
    options?: TransformOptions | undefined;
}) => string;
