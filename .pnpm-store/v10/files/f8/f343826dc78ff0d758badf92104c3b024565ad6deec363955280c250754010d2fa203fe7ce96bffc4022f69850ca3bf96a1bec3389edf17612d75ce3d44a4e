import type { PluginObj } from '@babel/core'
import { createVisitor } from './visitor'

export interface InspectorPluginOptions {
  /** override process.cwd() */
  cwd?: string;
  /** patterns to exclude matched files */
  excludes?: (string | RegExp)[];
}

export function InspectorBabelPlugin(babel: unknown, options?: InspectorPluginOptions): PluginObj {
  return {
    name: 'babel-plugin-react-dev-locator',

    visitor: createVisitor({
      cwd: options?.cwd,
      excludes: [
        'node_modules/',
        ...options?.excludes ?? [],
      ],
    }),
  }
}
