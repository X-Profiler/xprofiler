"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.doJSXOpeningElement = exports.pathMatch = exports.createVisitor = void 0;
const path_1 = require("path");
const types_1 = require("@babel/types");
const generated_1 = require("@babel/types/lib/builders/generated");
const utils_1 = require("./utils");
const createVisitor = ({ cwd = process.cwd(), excludes }) => {
    const isExclude = excludes?.length
        ? memo((filePath) => (0, exports.pathMatch)(filePath, excludes))
        : () => false;
    const pathRelative = memo((filePath) => (0, path_1.relative)(cwd, filePath));
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
                if (utils_1.threeNames.some(n => n === jsxName))
                    return;
                (0, exports.doJSXOpeningElement)(path.node, {
                    relativePath,
                    cwd,
                    path
                });
            },
        },
    };
    return visitor;
};
exports.createVisitor = createVisitor;
/**
 * simple path match method, only use string and regex
 */
const pathMatch = (filePath, matches) => {
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
exports.pathMatch = pathMatch;
const doJSXPathName = (name) => {
    const visitors = {
        JSXIdentifier: doJSXIdentifierName,
        JSXMemberExpression: doJSXMemberExpressionName,
        JSXNamespacedName: doJSXNamespacedNameName,
    };
    return visitors[name.type](name);
};
const doJSXOpeningElement = (node, option) => {
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
        : (0, generated_1.jsxAttribute)((0, generated_1.jsxIdentifier)('trae-inspector-start-line'), (0, generated_1.stringLiteral)(line.toString()));
    const startColumnAttr = isNil(column)
        ? null
        : (0, generated_1.jsxAttribute)((0, generated_1.jsxIdentifier)('trae-inspector-start-column'), (0, generated_1.stringLiteral)(column.toString()));
    const endLineAttr = isNil(endLine)
        ? null
        : (0, generated_1.jsxAttribute)((0, generated_1.jsxIdentifier)('trae-inspector-end-line'), (0, generated_1.stringLiteral)(endLine.toString()));
    const endColumnAttr = isNil(endColumn)
        ? null
        : (0, generated_1.jsxAttribute)((0, generated_1.jsxIdentifier)('trae-inspector-end-column'), (0, generated_1.stringLiteral)(endColumn.toString()));
    const relativePathAttr = (0, generated_1.jsxAttribute)((0, generated_1.jsxIdentifier)('trae-inspector-file-path'), (0, generated_1.stringLiteral)(relativePath));
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
            if ((0, types_1.isJSXText)(child)) {
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
    const staticAttr = (0, generated_1.jsxAttribute)((0, generated_1.jsxIdentifier)('trae-inspector-static-props'), (0, generated_1.stringLiteral)(encodeURIComponent(JSON.stringify(staticProps))));
    const attributes = [startLineAttr, startColumnAttr, endLineAttr, endColumnAttr, relativePathAttr, staticAttr];
    // Make sure that there are exist together
    if (attributes.every(Boolean)) {
        node.attributes.unshift(...attributes);
    }
    return { result: node };
};
exports.doJSXOpeningElement = doJSXOpeningElement;
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
        if ((0, types_1.isJSXText)(child)) {
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
