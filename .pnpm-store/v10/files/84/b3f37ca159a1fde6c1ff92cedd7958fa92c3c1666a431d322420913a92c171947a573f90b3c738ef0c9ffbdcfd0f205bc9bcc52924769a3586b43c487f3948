import path from 'path'
import {
  parse,
  type ParserPlugin,
  type ParserOptions,
} from '@babel/parser'
import generate from '@babel/generator'
import traverse from '@babel/traverse'
import type {
  Node,
  JSXOpeningElement,
} from '@babel/types'
import {
  pathMatch,
  doJSXOpeningElement,
} from './visitor'


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
export const transform = ({
  rootPath = process.cwd(),
  filePath,
  sourceCode,
  options,
}: {
  /**
   * path for project root path
   * @default process.cwd()
   */
  rootPath?: string;
  /**
   * path of source file
   * MUST keep consistent relative or absolute with `rootPath`
   */
  filePath: string;
  sourceCode: string;
  options?: TransformOptions;
}) => {
  /**
   * example:
   * rootPath: /home/xxx/project
   * filePath: /home/xxx/project/src/ooo/xxx.js
   * relativePath: src/ooo/xxx.js
   */
  const relativePath = path.relative(rootPath, filePath)

  const isSkip = pathMatch(filePath, options?.excludes)
  if (isSkip) {
    return sourceCode
  }

  const ast: Node = parse(sourceCode, {
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
  })


  /**
   * astexplorer + @babel/parser
   * https://astexplorer.net
   */
  traverse(ast, {
    JSXOpeningElement: {
      enter(path) {
        doJSXOpeningElement(
          path.node as JSXOpeningElement,
          { relativePath, cwd: rootPath, path },
        )
      },
    },
  })

  const {
    code,
  } = generate(ast, {
    decoratorsBeforeExport: true,
  })

  return code
}
