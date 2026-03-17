import { relative } from 'path'
import type { PluginPass } from '@babel/core'
import type {
  JSXAttribute,
  JSXElement,
  JSXIdentifier,
  JSXMemberExpression,
  JSXNamespacedName,
  JSXOpeningElement,
  Node as NodeType,
} from '@babel/types'
import {
  // isJSXAttribute,
  // isJSXIdentifier,
  // isJSXNamespacedName,
  isJSXText,
  // isNumericLiteral,
  // isStringLiteral,
} from '@babel/types';
import {
  jsxAttribute,
  jsxIdentifier,
  stringLiteral,
  // @ts-expect-error import from deep path for reduce load files
} from '@babel/types/lib/builders/generated'
import type { NodePath, Visitor } from '@babel/traverse'
import { threeNames } from './utils'


export const createVisitor = ({ cwd = process.cwd(), excludes }: {
  cwd?: string;
  excludes?: (string | RegExp)[];
}): Visitor<PluginPass> => {
  const isExclude = excludes?.length
    ? memo((filePath: string): boolean => pathMatch(filePath, excludes))
    : () => false

  const pathRelative = memo((filePath: string): string => relative(
    cwd,
    filePath,
  ))

  const visitor: Visitor<PluginPass> = {
    JSXOpeningElement: {
      enter(path, state: PluginPass) {
        const filePath = state?.file?.opts?.filename
        if (!filePath)
          return
        if (isExclude(filePath))
          return

        const relativePath = pathRelative(filePath)
        if (path.node.name.type !== 'JSXIdentifier')
          return
        const jsxName = path.node.name.name;
        if (/^[A-Z]/.test(jsxName))
          return
        if (threeNames.some(n => n === jsxName))
          return
        doJSXOpeningElement(
          path.node,
          {
            relativePath,
            cwd,
            path
          },
        )
      },
    },
  }

  return visitor
}


/**
 * simple path match method, only use string and regex
 */
export const pathMatch = (filePath: string, matches?: (string | RegExp)[]): boolean => {
  if (!matches?.length)
    return false

  return matches.some((match) => {
    if (typeof match === 'string') {
      return filePath.includes(match)
    }
    else if (match instanceof RegExp) {
      return match.test(filePath)
    }
    // default is do not filter when match is illegal, so return true
    return true
  })
}


type ElementTypes = JSXOpeningElement['name']['type']

const doJSXPathName: NodeHandler<JSXOpeningElement['name']> = (name) => {
  const visitors: { [key in ElementTypes]: NodeHandler } = {
    JSXIdentifier: doJSXIdentifierName,
    JSXMemberExpression: doJSXMemberExpressionName,
    JSXNamespacedName: doJSXNamespacedNameName,
  }

  return visitors[name.type](name)
}

export const doJSXOpeningElement: NodeHandler<
  JSXOpeningElement,
  { relativePath: string; cwd: string; path: NodePath<JSXOpeningElement> }
> = (node, option) => {
  const { stop } = doJSXPathName(node.name)
  if (stop)
    return { stop }

  const { relativePath, path } = option
  
  const jsxNode = path.parent as JSXElement;

  const line = jsxNode.loc?.start.line
  const column = jsxNode.loc?.start.column
  const endLine = jsxNode.loc?.end.line;
  const endColumn = jsxNode.loc?.end.column;

  const startLineAttr: JSXAttribute | null = isNil(line)
    ? null
    : jsxAttribute(
      jsxIdentifier('trae-inspector-start-line'),
      stringLiteral(line.toString()),
    )

  const startColumnAttr: JSXAttribute | null = isNil(column)
    ? null
    : jsxAttribute(
      jsxIdentifier('trae-inspector-start-column'),
      stringLiteral(column.toString()),
    )

  const endLineAttr: JSXAttribute | null = isNil(endLine)
    ? null
    : jsxAttribute(
      jsxIdentifier('trae-inspector-end-line'),
      stringLiteral(endLine.toString()),
    )

  const endColumnAttr: JSXAttribute | null = isNil(endColumn)
    ? null
    : jsxAttribute(
      jsxIdentifier('trae-inspector-end-column'),
      stringLiteral(endColumn.toString()),
    )

  const relativePathAttr: JSXAttribute = jsxAttribute(
    jsxIdentifier('trae-inspector-file-path'),
    stringLiteral(relativePath),
  )

  // 查找非 props 传入的，静态定义的节点
  const staticProps: Record<string, string | boolean> = {};
  // node.attributes.forEach(attr => {
  //   if (isJSXAttribute(attr)) {
  //     let key = '';
  //     if (isJSXIdentifier(attr.name)) {
  //       key = attr.name.name;
  //     } else if (isJSXNamespacedName(attr.name)) {
  //       key = attr.name.namespace.name + ':' + attr.name.name.name;
  //     } else {
  //       return;
  //     }
  //     // <div disabled /> 布尔属性
  //     if (attr.value === null) {
  //       staticProps[key] = true;
  //     }
  //     // <div className="xxx" />
  //     else if (isStringLiteral(attr.value) || isNumericLiteral(attr.value)) {
  //       staticProps[key] = attr.value.value;
  //     }
  //     // 其他类型（表达式等）不处理
  //   }
  //   // JSXSpreadAttribute 跳过
  // });
  // 查找当前这个 JSXElement 的子节点是否是纯文本，是的话就把文本内容作为静态属性
  const isChildrenTextOnly = isTextOnlyChildren(jsxNode)
  let text = ''
  if (isChildrenTextOnly) {
    let startLine: undefined | number, startColumn: undefined | number, endLine: undefined | number, endColumn: undefined | number;
    jsxNode.children.forEach((child, index) => {
      if (isJSXText(child)) {
        if (index === 0) {
          startLine = child.loc?.start.line;
          startColumn = child.loc?.start.column;
        }

        if (index === jsxNode.children.length - 1) {
          endLine = child.loc?.end.line;
          endColumn = child.loc?.end.column;
        }
        text += child.value
      }
    })
    staticProps.text = text.trim();
    if (startLine && startColumn) {
      staticProps.textStartLine = startLine.toString();
      staticProps.textStartColumn = startColumn.toString();
    }
    if (endLine && endColumn) {
      staticProps.textEndLine = endLine.toString();
      staticProps.textEndColumn = endColumn.toString();
    }
  }

  staticProps.cwd = option.cwd;

  const staticAttr: JSXAttribute = jsxAttribute(
    jsxIdentifier('trae-inspector-static-props'),
    stringLiteral(encodeURIComponent(JSON.stringify(staticProps))),
  )

  const attributes = [startLineAttr, startColumnAttr, endLineAttr, endColumnAttr, relativePathAttr, staticAttr] as JSXAttribute[]

  // Make sure that there are exist together
  if (attributes.every(Boolean)) {
    node.attributes.unshift(...attributes)
  }

  return { result: node }
}

type NodeHandler<Node extends NodeType = any, Option = void> =
  (node: Node, option: Option) => {
    /**
     * stop processing flag
     */
    stop?: boolean;

    /**
     * throw error
     */
    error?: any;

    /**
     * node after processing
     */
    result?: Node;
  }

const doJSXIdentifierName: NodeHandler<JSXIdentifier> = (name) => {
  if (name.name.endsWith('Fragment')) {
    return { stop: true }
  }
  return { stop: false }
}

const doJSXMemberExpressionName: NodeHandler<JSXMemberExpression> = (name) => {
  const { stop } = doJSXIdentifierName(name.property)
  return { stop }
}

const doJSXNamespacedNameName: NodeHandler<JSXNamespacedName> = (name) => {
  const { stop } = doJSXIdentifierName(name.name)
  return { stop }
}

function isTextOnlyChildren(node: JSXElement) {
  // 只允许 JSXText，且不能全是空白
  return node.children.length > 0 && node.children.every(child => {
    if (isJSXText(child)) {
      // 过滤掉全空白的文本节点
      return child.value.trim().length > 0;
    }
    // 只要有不是 JSXText 的子节点就不是 string only
    return false;
  });
}

const isNil = (value: unknown): value is null | undefined => value === null || value === undefined

const memo = <Params, Result>(handler: (params: Params) => Result): typeof handler => {
  const cache = new Map<Params, Result>()
  return (arg) => {
    if (cache.has(arg)) {
      return cache.get(arg)!
    }
    const result = handler(arg)
    cache.set(arg, result)
    return result
  }
}
