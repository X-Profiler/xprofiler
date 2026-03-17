/**
 * TRAE Badge 主类
 */

import type { TraeBadgeOptions, TraeBadgeInstance, LogoVariant } from './types';
import { BADGE_CSS, TRAE_LOGOS, CANCEL_SVG_DARK, CANCEL_SVG_LIGHT, DEFAULT_OPTIONS, SHARE_SVG } from './constants';
import { autoSelectTheme } from './theme';

export class TraeBadge implements TraeBadgeInstance {
  private options: TraeBadgeOptions;
  private element: HTMLElement | null = null;
  private styleElement: HTMLStyleElement | null = null;
  private badgeId: string;
  private originalVariant?: TraeBadgeOptions['variant']; // 保存原始主题设置

  constructor(options: TraeBadgeOptions = {}) {
    this.options = { ...DEFAULT_OPTIONS, ...options };
    this.originalVariant = options.variant; // 保存用户设置的原始主题
    this.badgeId = `trae-badge-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    this.init();
  }

  private init(): void {
    this.injectStyles();
    this.createBadge();
  }

  private injectStyles(): void {
    // 避免重复注入样式
    if (document.getElementById('trae-badge-styles')) {
      return;
    }

    this.styleElement = document.createElement('style');
    this.styleElement.id = 'trae-badge-styles';
    this.styleElement.textContent = BADGE_CSS;
    document.head.appendChild(this.styleElement);
  }

  private createBadge(): void {
    // 移除已存在的 Badge
    this.destroy();

    if (this.options.disabled) {
      return;
    }

    // 如果启用了自动主题选择，则自动选择主题
    if (this.options.autoTheme) {
      const autoThemeTarget = this.options.autoThemeTarget || this.options.container;
      this.options.variant = autoSelectTheme(autoThemeTarget);
    }

    this.element = document.createElement('div');
    this.element.id = this.badgeId;
    this.element.className = this.getBadgeClassName();
    
    const isDarkBackground = this.options.variant === 'dark';
    const logoType: LogoVariant = isDarkBackground ? 'light' : 'dark';
    
    this.element.innerHTML = `
      <div class="trae-badge-content">
        <div class="trae-badge-logo">${TRAE_LOGOS[logoType]}</div>
        <div class="trae-badge-share">${SHARE_SVG}</div>
      </div>
      <div class="trae-badge-cancel">
        ${isDarkBackground ? CANCEL_SVG_LIGHT : CANCEL_SVG_DARK}
      </div>
    `;

    // 应用自定义样式
    if (this.options.style) {
      Object.assign(this.element.style, this.options.style);
    }

    // 添加取消按钮点击事件
    this.addCancelEvent();

    // 添加点击事件（点击badge本身）
    if (this.options.clickable) {
      this.element.style.cursor = 'pointer';
      this.element.addEventListener('click', this.handleClick.bind(this));
    }

    // 确定容器
    let container: HTMLElement = document.body;
    if (this.options.container) {
      if (typeof this.options.container === 'string') {
        const found = document.querySelector(this.options.container);
        if (found) {
          container = found as HTMLElement;
        }
      } else {
        container = this.options.container;
      }
    }

    container.appendChild(this.element);
  }

  private addCancelEvent(): void {
    if (!this.element) return;

    const cancelButton = this.element.querySelector('.trae-badge-cancel');
    if (cancelButton) {
      cancelButton.addEventListener('click', (e) => {
        e.stopPropagation(); // 防止触发 badge 本身的点击事件
        this.handleCancel();
      });
    }
  }

  private handleCancel(): void {
    // 隐藏/销毁 badge
    this.destroy();
  }

  private getBadgeClassName(): string {
    const classes = [
      'trae-badge',
      `trae-badge--${this.options.variant}`,
      `trae-badge--${this.options.position}`
    ];

    if (this.options.className) {
      classes.push(this.options.className);
    }

    return classes.join(' ');
  }

  private handleClick(): void {
    if (this.options.clickUrl) {
      window.open(this.options.clickUrl, '_blank');
    } else {
      console.log('TRAE Badge clicked');
    }
  }

  // 公共方法
  updateOptions(newOptions: Partial<TraeBadgeOptions>): void {
    // 更新原始主题设置
    if (newOptions.variant !== undefined) {
      this.originalVariant = newOptions.variant;
    }
    
    this.options = { ...this.options, ...newOptions };
    this.createBadge();
  }

  destroy(): void {
    if (this.element) {
      this.element.remove();
      this.element = null;
    }
  }

  getOptions(): TraeBadgeOptions {
    return { ...this.options };
  }

  getElement(): HTMLElement | null {
    return this.element;
  }

  /**
   * 刷新自动主题（如果启用）
   * 重新检测背景色并更新主题
   */
  refreshAutoTheme(): void {
    if (this.options.autoTheme) {
      const autoThemeTarget = this.options.autoThemeTarget || this.options.container;
      const newVariant = autoSelectTheme(autoThemeTarget);
      
      // 只有当主题发生变化时才重新创建badge
      if (newVariant !== this.options.variant) {
        this.options.variant = newVariant;
        this.createBadge();
      }
    }
  }

  // 静态方法
  static destroyAll(): void {
    const badges = document.querySelectorAll('[id^="trae-badge-"]');
    badges.forEach(badge => badge.remove());
  }

  static isStylesInjected(): boolean {
    return !!document.getElementById('trae-badge-styles');
  }
} 