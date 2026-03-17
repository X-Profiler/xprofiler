import { createVisitor } from './visitor';
export function InspectorBabelPlugin(babel, options) {
    return {
        name: 'babel-plugin-react-dev-locator',
        visitor: createVisitor({
            cwd: options?.cwd,
            excludes: [
                'node_modules/',
                ...options?.excludes ?? [],
            ],
        }),
    };
}
