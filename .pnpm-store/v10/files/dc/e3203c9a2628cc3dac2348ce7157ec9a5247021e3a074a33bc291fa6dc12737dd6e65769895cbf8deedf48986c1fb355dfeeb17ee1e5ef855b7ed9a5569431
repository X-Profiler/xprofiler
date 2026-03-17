import { Plugin } from 'vite';

/**
 * TRAE Badge 类型定义
 */

type BadgeVariant = 'light' | 'dark';
type BadgePosition = 'bottom-right' | 'bottom-left' | 'top-right' | 'top-left';
interface TraeBadgeOptions {
    /** Badge 样式主题 */
    variant?: BadgeVariant;
    /** Badge 位置 */
    position?: BadgePosition;
    /** 是否可点击 */
    clickable?: boolean;
    /** 点击跳转 URL */
    clickUrl?: string | null;
    /** 是否禁用 */
    disabled?: boolean;
    /** 自定义 CSS 类名 */
    className?: string;
    /** 自定义样式 */
    style?: Partial<CSSStyleDeclaration>;
    /** 自定义容器，默认为 document.body */
    container?: HTMLElement | string;
    /** 是否启用自动主题选择（根据背景色自动选择合适的主题） */
    autoTheme?: boolean;
    /** 自动主题检测的目标元素，默认为badge的容器 */
    autoThemeTarget?: HTMLElement | string;
}

/**
 * TRAE Badge Vite Plugin
 * 通过复用现有代码实现的 Vite 插件
 */

interface VitePluginOptions extends TraeBadgeOptions {
    /** 仅在开发环境显示 */
    devOnly?: boolean;
    /** 仅在生产环境显示 */
    prodOnly?: boolean;
    /** 是否启用自动主题选择 */
    autoTheme?: boolean;
    /** 自动主题检测的目标元素 */
    autoThemeTarget?: string;
}
/**
 * TRAE Badge Vite Plugin
 */
declare function traeBadgePlugin(options?: VitePluginOptions): Plugin;

export { traeBadgePlugin as default, traeBadgePlugin };
export type { VitePluginOptions };
