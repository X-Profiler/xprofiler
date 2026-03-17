/**
 * TRAE Badge 主题选择工具
 */

import type { BadgeVariant } from './types';

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

/**
 * 根据背景色自动选择合适的主题
 */
export function autoSelectTheme(container?: HTMLElement | string): BadgeVariant {
  const backgroundColor = detectBackgroundColor(container);
  const rgb = parseColor(backgroundColor);
  
  if (!rgb) {
    return 'dark'; // 默认深色主题
  }
  
  const [r, g, b] = rgb;
  const luminance = getLuminance(r, g, b);
  
  // 根据亮度选择主题，只使用 light 和 dark 两种主题
  if (luminance > 128) {
    // 浅色背景，选择深色主题
    return 'dark';
  } else {
    // 深色背景，选择浅色主题
    return 'light';
  }
}

/**
 * 检查背景是否为深色
 */
export function isDarkBackground(container?: HTMLElement | string): boolean {
  const backgroundColor = detectBackgroundColor(container);
  const rgb = parseColor(backgroundColor);
  
  if (!rgb) {
    return false; // 默认认为是浅色背景
  }
  
  const [r, g, b] = rgb;
  const luminance = getLuminance(r, g, b);
  
  return luminance < 128;
} 