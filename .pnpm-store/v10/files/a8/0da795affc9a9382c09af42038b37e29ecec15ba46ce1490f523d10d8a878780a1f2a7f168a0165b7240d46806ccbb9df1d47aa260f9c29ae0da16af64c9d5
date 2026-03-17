/**
 * TRAE Badge 类型定义
 */

export type { LogoVariant } from './constants';
export type BadgeVariant = 'light' | 'dark';
export type BadgePosition = 'bottom-right' | 'bottom-left' | 'top-right' | 'top-left';

export interface TraeBadgeOptions {
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

export interface TraeBadgeInstance {
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