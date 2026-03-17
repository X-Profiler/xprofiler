/**
 * TRAE Badge 工具函数
 */

import { TraeBadge } from './badge';
import type { TraeBadgeOptions } from './types';

/**
 * 自动初始化 Badge
 */
export function autoInit(config: TraeBadgeOptions = {}): TraeBadge | null {
  // 检查是否在浏览器环境
  if (typeof window === 'undefined' || typeof document === 'undefined') {
    return null;
  }

  // 避免重复初始化
  if ((window as any).TraeBadgeInstance) {
    return (window as any).TraeBadgeInstance;
  }

  // 等待 DOM 加载完成
  const initialize = () => {
    const badge = new TraeBadge(config);
    (window as any).TraeBadgeInstance = badge;
    return badge;
  };

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initialize);
    return null;
  } else {
    return initialize();
  }
}

/**
 * 创建 Badge 实例
 */
export function createBadge(options: TraeBadgeOptions = {}): TraeBadge {
  return new TraeBadge(options);
}

/**
 * 销毁所有 Badge 实例
 */
export function destroyAllBadges(): void {
  TraeBadge.destroyAll();
  delete (window as any).TraeBadgeInstance;
}

/**
 * 检查是否支持 Badge
 */
export function isSupported(): boolean {
  return typeof window !== 'undefined' && 
         typeof document !== 'undefined' && 
         'querySelector' in document;
}

/**
 * 获取全局 Badge 实例
 */
export function getGlobalInstance(): TraeBadge | null {
  return (window as any).TraeBadgeInstance || null;
}

/**
 * 将RGB颜色转换为亮度值 (0-255)
 */
export function getLuminance(r: number, g: number, b: number): number {
  // 使用标准的亮度计算公式
  return 0.299 * r + 0.587 * g + 0.114 * b;
}

/**
 * 解析颜色字符串为RGB值
 */
export function parseColor(color: string): [number, number, number] | null {
  // 处理 rgba 格式
  const rgbaMatch = color.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)(?:,\s*[\d.]+)?\)/);
  if (rgbaMatch) {
    return [parseInt(rgbaMatch[1]), parseInt(rgbaMatch[2]), parseInt(rgbaMatch[3])];
  }
  
  // 处理 hex 格式
  const hexMatch = color.match(/^#([0-9a-f]{6})$/i);
  if (hexMatch) {
    const hex = hexMatch[1];
    return [
      parseInt(hex.substr(0, 2), 16),
      parseInt(hex.substr(2, 2), 16),
      parseInt(hex.substr(4, 2), 16)
    ];
  }
  
  // 处理 hex 短格式
  const hexShortMatch = color.match(/^#([0-9a-f]{3})$/i);
  if (hexShortMatch) {
    const hex = hexShortMatch[1];
    return [
      parseInt(hex[0] + hex[0], 16),
      parseInt(hex[1] + hex[1], 16),
      parseInt(hex[2] + hex[2], 16)
    ];
  }
  
  return null;
}

/**
 * 检测元素或其父元素的背景色
 */
export function detectBackgroundColor(element?: HTMLElement | string): string {
  let targetElement: HTMLElement;
  
  if (typeof element === 'string') {
    const found = document.querySelector(element);
    if (!found) return 'rgb(255, 255, 255)'; // 默认白色
    targetElement = found as HTMLElement;
  } else if (element) {
    targetElement = element;
  } else {
    targetElement = document.body;
  }
  
  // 向上查找直到找到非透明的背景色
  let currentElement: HTMLElement | null = targetElement;
  
  while (currentElement && currentElement !== document.documentElement) {
    const computedStyle = window.getComputedStyle(currentElement);
    const backgroundColor = computedStyle.backgroundColor;
    
    // 如果找到了非透明/默认的背景色
    if (backgroundColor && backgroundColor !== 'rgba(0, 0, 0, 0)' && backgroundColor !== 'transparent') {
      return backgroundColor;
    }
    
    currentElement = currentElement.parentElement;
  }
  
  // 检查 body 的背景色
  const bodyStyle = window.getComputedStyle(document.body);
  const bodyBackground = bodyStyle.backgroundColor;
  if (bodyBackground && bodyBackground !== 'rgba(0, 0, 0, 0)' && bodyBackground !== 'transparent') {
    return bodyBackground;
  }
  
  // 检查 html 的背景色
  const htmlStyle = window.getComputedStyle(document.documentElement);
  const htmlBackground = htmlStyle.backgroundColor;
  if (htmlBackground && htmlBackground !== 'rgba(0, 0, 0, 0)' && htmlBackground !== 'transparent') {
    return htmlBackground;
  }
  
  // 默认返回白色
  return 'rgb(255, 255, 255)';
} 