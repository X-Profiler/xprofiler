import type { PluginPass } from '@babel/core';
import type { JSXOpeningElement, Node as NodeType } from '@babel/types';
import type { NodePath, Visitor } from '@babel/traverse';
export declare const createVisitor: ({ cwd, excludes }: {
    cwd?: string | undefined;
    excludes?: (string | RegExp)[] | undefined;
}) => Visitor<PluginPass>;
/**
 * simple path match method, only use string and regex
 */
export declare const pathMatch: (filePath: string, matches?: (string | RegExp)[]) => boolean;
export declare const doJSXOpeningElement: NodeHandler<JSXOpeningElement, {
    relativePath: string;
    cwd: string;
    path: NodePath<JSXOpeningElement>;
}>;
type NodeHandler<Node extends NodeType = any, Option = void> = (node: Node, option: Option) => {
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
};
export {};
