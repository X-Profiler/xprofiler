"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.InspectorBabelPlugin = void 0;
const visitor_1 = require("./visitor");
function InspectorBabelPlugin(babel, options) {
    return {
        name: 'babel-plugin-react-dev-locator',
        visitor: (0, visitor_1.createVisitor)({
            cwd: options?.cwd,
            excludes: [
                'node_modules/',
                ...options?.excludes ?? [],
            ],
        }),
    };
}
exports.InspectorBabelPlugin = InspectorBabelPlugin;
