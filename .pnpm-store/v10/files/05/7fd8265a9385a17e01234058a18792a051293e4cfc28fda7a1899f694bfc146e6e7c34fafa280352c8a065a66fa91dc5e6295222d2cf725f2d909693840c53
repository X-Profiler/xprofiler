import { Plugin } from 'vite';

/**
 * TRAE Badge 常量定义
 */
type LogoVariant = 'light' | 'dark';
declare const TRAE_LOGOS: Record<LogoVariant, string>;
declare const BADGE_CSS = "\n.trae-badge {\n  display: inline-flex;\n  align-items: center;\n  justify-content: center;\n  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;\n  font-weight: 500;\n  transition: all 0.2s ease;\n  cursor: pointer;\n  box-sizing: border-box;\n  border-radius: 6px;\n  position: fixed;\n  padding: 8px 8px 8px 10px;\n  height: 28px;\n  width: 130px;\n  z-index: 9999;\n}\n\n.trae-badge:hover.trae-badge--light {\n  background-color: #EDEFF2;\n}\n\n.trae-badge:hover.trae-badge--dark {\n  background-color: #2A2D31;\n}\n\n.trae-badge-content {\n  display: flex;\n  align-items: center;\n  justify-content: center;\n  gap: 4px;\n}\n\n.trae-badge-logo {\n  flex-shrink: 0;\n}\n\n.trae-badge-share {\n  flex-shrink: 0;\n  color: rgba(78, 82, 87, 1);\n}\n\n.trae-badge:hover.trae-badge--light .trae-badge-share {\n  color: rgba(38, 41, 44, 1);\n}\n\n.trae-badge:hover.trae-badge--dark .trae-badge-share {\n  color: #D1D3DB;\n}\n\n.trae-badge-cancel {\n  position: absolute;\n  top: -8px;\n  right: -8px;\n  width: 16px;\n  height: 16px;\n  cursor: pointer;\n  opacity: 0;\n  visibility: hidden;\n  z-index: 10001;\n  display: flex;\n  align-items: center;\n  justify-content: center;\n  color: #000000;\n}\n\n.trae-badge:hover .trae-badge-cancel {\n  opacity: 1 !important;\n  visibility: visible !important;\n}\n\n/* \u5728\u79FB\u52A8\u7AEF\u8BBE\u5907\u4E0A\u59CB\u7EC8\u663E\u793A\u53D6\u6D88\u6309\u94AE */\n@media (hover: none) and (pointer: coarse) {\n  .trae-badge-cancel {\n    opacity: 1 !important;\n    visibility: visible !important;\n  }\n}\n\n.trae-badge--light {\n  background-color: rgba(255, 255, 255, 1);\n  color: #ffffff;\n  box-shadow: 0px 0px 0.5px 0px rgba(0, 0, 0, 0.12), 0px 1px 3px 0px rgba(0, 0, 0, 0.08), 0px 4px 8px 0px rgba(0, 0, 0, 0.12);\n}\n\n.trae-badge--dark {\n  background-color: #000000;\n  color: #ffffff;\n  box-shadow: 0px 0px 0.5px 0px rgba(0, 0, 0, 0.12), 0px 1px 3px 0px rgba(0, 0, 0, 0.08), 0px 4px 8px 0px rgba(0, 0, 0, 0.12);\n}\n\n.trae-badge--bottom-right { bottom: 20px; right: 20px; }\n.trae-badge--bottom-left { bottom: 20px; left: 20px; }\n.trae-badge--top-right { top: 20px; right: 20px; }\n.trae-badge--top-left { top: 20px; left: 20px; }\n";
declare const DEFAULT_OPTIONS: {
    variant: "dark";
    position: "bottom-right";
    clickable: boolean;
    clickUrl: null;
    disabled: boolean;
};

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
interface TraeBadgeInstance {
    /** 更新配置 */
    updateOptions(options: Partial<TraeBadgeOptions>): void;
    /** 销毁 Badge */
    destroy(): void;
    /** 获取当前配置 */
    getOptions(): TraeBadgeOptions;
    /** 获取 Badge 元素 */
    getElement(): HTMLElement | null;
    /** 刷新自动主题（如果启用） */
    refreshAutoTheme(): void;
}

/**
 * TRAE Badge 主类
 */

declare class TraeBadge implements TraeBadgeInstance {
    private options;
    private element;
    private styleElement;
    private badgeId;
    private originalVariant?;
    constructor(options?: TraeBadgeOptions);
    private init;
    private injectStyles;
    private createBadge;
    private addCancelEvent;
    private handleCancel;
    private getBadgeClassName;
    private handleClick;
    updateOptions(newOptions: Partial<TraeBadgeOptions>): void;
    destroy(): void;
    getOptions(): TraeBadgeOptions;
    getElement(): HTMLElement | null;
    /**
     * 刷新自动主题（如果启用）
     * 重新检测背景色并更新主题
     */
    refreshAutoTheme(): void;
    static destroyAll(): void;
    static isStylesInjected(): boolean;
}

/**
 * TRAE Badge 工具函数
 */

/**
 * 自动初始化 Badge
 */
declare function autoInit(config?: TraeBadgeOptions): TraeBadge | null;
/**
 * 创建 Badge 实例
 */
declare function createBadge(options?: TraeBadgeOptions): TraeBadge;
/**
 * 销毁所有 Badge 实例
 */
declare function destroyAllBadges(): void;
/**
 * 检查是否支持 Badge
 */
declare function isSupported(): boolean;
/**
 * 获取全局 Badge 实例
 */
declare function getGlobalInstance(): TraeBadge | null;

/**
 * TRAE Badge 主题选择工具
 */

/**
 * 将RGB颜色转换为亮度值 (0-255)
 */
declare function getLuminance(r: number, g: number, b: number): number;
/**
 * 解析颜色字符串为RGB值
 */
declare function parseColor(color: string): [number, number, number] | null;
/**
 * 检测元素或其父元素的背景色
 */
declare function detectBackgroundColor(element?: HTMLElement | string): string;
/**
 * 根据背景色自动选择合适的主题
 */
declare function autoSelectTheme(container?: HTMLElement | string): BadgeVariant;
/**
 * 检查背景是否为深色
 */
declare function isDarkBackground(container?: HTMLElement | string): boolean;

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

export { BADGE_CSS, DEFAULT_OPTIONS, TRAE_LOGOS, TraeBadge, autoInit, autoSelectTheme, createBadge, TraeBadge as default, destroyAllBadges, detectBackgroundColor, getGlobalInstance, getLuminance, isDarkBackground, isSupported, parseColor, traeBadgePlugin };
export type { BadgePosition, BadgeVariant, TraeBadgeInstance, TraeBadgeOptions };
