import { relative } from 'path';
import { 
// isJSXAttribute,
// isJSXIdentifier,
// isJSXNamespacedName,
isJSXText,
// isNumericLiteral,
// isStringLiteral,
 } from '@babel/types';
import { jsxAttribute, jsxIdentifier, stringLiteral,
// @ts-expect-error import from deep path for reduce load files
 } from '@babel/types/lib/builders/generated';
import { threeNames } from './utils';
export const createVisitor = ({ cwd = process.cwd(), excludes }) => {
    const isExclude = excludes?.length
        ? memo((filePath) => pathMatch(filePath, excludes))
        : () => false;
    const pathRelative = memo((filePath) => relative(cwd, filePath));
    const visitor = {
        JSXOpeningElement: {
            enter(path, state) {
                const filePath = state?.file?.opts?.filename;
                if (!filePath)
                    return;
                if (isExclude(filePath))
                    return;
                const relativePath = pathRelative(filePath);
                if (path.node.name.type !== 'JSXIdentifier')
                    return;
                const jsxName = path.node.name.name;
                if (/^[A-Z]/.test(jsxName))
                    return;
                if (threeNames.some(n => n === jsxName))
                    return;
                doJSXOpeningElement(path.node, {
                    relativePath,
                    cwd,
                    path
                });
            },
        },
    };
    return visitor;
};
/**
 * simple path match method, only use string and regex
 */
export const pathMatch = (filePath, matches) => {
    if (!matches?.length)
        return false;
    return matches.some((match) => {
        if (typeof match === 'string') {
            return filePath.includes(match);
        }
        else if (match instanceof RegExp) {
            return match.test(filePath);
        }
        // default is do not filter when match is illegal, so return true
        return true;
    });
};
const doJSXPathName = (name) => {
    const visitors = {
        JSXIdentifier: doJSXIdentifierName,
        JSXMemberExpression: doJSXMemberExpressionName,
        JSXNamespacedName: doJSXNamespacedNameName,
    };
    return visitors[name.type](name);
};
export const doJSXOpeningElement = (node, option) => {
    const { stop } = doJSXPathName(node.name);
    if (stop)
        return { stop };
    const { relativePath, path } = option;
    const jsxNode = path.parent;
    const line = jsxNode.loc?.start.line;
    const column = jsxNode.loc?.start.column;
    const endLine = jsxNode.loc?.end.line;
    const endColumn = jsxNode.loc?.end.column;
    const startLineAttr = isNil(line)
        ? null
        : jsxAttribute(jsxIdentifier('trae-inspector-start-line'), stringLiteral(line.toString()));
    const startColumnAttr = isNil(column)
        ? null
        : jsxAttribute(jsxIdentifier('trae-inspector-start-column'), stringLiteral(column.toString()));
    const endLineAttr = isNil(endLine)
        ? null
        : jsxAttribute(jsxIdentifier('trae-inspector-end-line'), stringLiteral(endLine.toString()));
    const endColumnAttr = isNil(endColumn)
        ? null
        : jsxAttribute(jsxIdentifier('trae-inspector-end-column'), stringLiteral(endColumn.toString()));
    const relativePathAttr = jsxAttribute(jsxIdentifier('trae-inspector-file-path'), stringLiteral(relativePath));
    // 查找非 props 传入的，静态定义的节点
    const staticProps = {};
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
    const isChildrenTextOnly = isTextOnlyChildren(jsxNode);
    let text = '';
    if (isChildrenTextOnly) {
        let startLine, startColumn, endLine, endColumn;
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
                text += child.value;
            }
        });
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
    const staticAttr = jsxAttribute(jsxIdentifier('trae-inspector-static-props'), stringLiteral(encodeURIComponent(JSON.stringify(staticProps))));
    const attributes = [startLineAttr, startColumnAttr, endLineAttr, endColumnAttr, relativePathAttr, staticAttr];
    // Make sure that there are exist together
    if (attributes.every(Boolean)) {
        node.attributes.unshift(...attributes);
    }
    return { result: node };
};
const doJSXIdentifierName = (name) => {
    if (name.name.endsWith('Fragment')) {
        return { stop: true };
    }
    return { stop: false };
};
const doJSXMemberExpressionName = (name) => {
    const { stop } = doJSXIdentifierName(name.property);
    return { stop };
};
const doJSXNamespacedNameName = (name) => {
    const { stop } = doJSXIdentifierName(name.name);
    return { stop };
};
function isTextOnlyChildren(node) {
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
const isNil = (value) => value === null || value === undefined;
const memo = (handler) => {
    const cache = new Map();
    return (arg) => {
        if (cache.has(arg)) {
            return cache.get(arg);
        }
        const result = handler(arg);
        cache.set(arg, result);
        return result;
    };
};
