/**
 * TRAE Badge Vite Plugin
 * 通过复用现有代码实现的 Vite 插件
 */

import type { Plugin } from 'vite';
import { TRAE_LOGOS, BADGE_CSS, CANCEL_SVG_DARK, CANCEL_SVG_LIGHT, SHARE_SVG } from './constants';
import type { TraeBadgeOptions } from './types';

export interface VitePluginOptions extends TraeBadgeOptions {
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
export function traeBadgePlugin(options: VitePluginOptions = {}): Plugin {
  const config = {
    variant: 'dark' as const,
    position: 'bottom-right' as const,
    clickable: true,
    clickUrl: null,
    disabled: false,
    devOnly: false,
    prodOnly: false,
    autoTheme: true,
    autoThemeTarget: 'body',
    ...options
  };

  return {
    name: 'vite-plugin-trae-badge',
    
    configResolved(resolvedConfig) {
      const isDev = resolvedConfig.command === 'serve';
      const isProd = resolvedConfig.command === 'build';
      
      if (config.disabled) return;
      if (config.devOnly && !isDev) {
        config.disabled = true;
        return;
      }
      if (config.prodOnly && !isProd) {
        config.disabled = true;
        return;
      }
    },

    transformIndexHtml: {
      order: 'post',
      handler(html: string) {
        if (config.disabled) {
          return html;
        }

        // 将主题函数转换为字符串以便注入
        const themeFunctions = `
          // 主题选择函数 (复用自 theme.ts)
          function getLuminance(r, g, b) {
            return 0.299 * r + 0.587 * g + 0.114 * b;
          }
          
          function parseColor(color) {
            const rgbaMatch = color.match(/rgba?\\((\\d+),\\s*(\\d+),\\s*(\\d+)(?:,\\s*[\\d.]+)?\\)/);
            if (rgbaMatch) {
              return [parseInt(rgbaMatch[1]), parseInt(rgbaMatch[2]), parseInt(rgbaMatch[3])];
            }
            
            const hexMatch = color.match(/^#([0-9a-f]{6})$/i);
            if (hexMatch) {
              const hex = hexMatch[1];
              return [
                parseInt(hex.substr(0, 2), 16),
                parseInt(hex.substr(2, 2), 16),
                parseInt(hex.substr(4, 2), 16)
              ];
            }
            
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
          
          function detectBackgroundColor(element) {
            let targetElement;
            
            if (typeof element === 'string') {
              const found = document.querySelector(element);
              console.debug('Target element found:', found);
              if (!found) return 'rgb(255, 255, 255)';
              targetElement = found;
            } else if (element) {
              targetElement = element;
            } else {
              targetElement = document.body;
            }
            
            let currentElement = targetElement;
            
            while (currentElement && currentElement !== document.documentElement) {
              const computedStyle = window.getComputedStyle(currentElement);
              const backgroundColor = computedStyle.backgroundColor;
              
              console.debug('Checking element:', currentElement.tagName, currentElement.id, currentElement.className);
              console.debug('Background color:', backgroundColor);
              
              if (backgroundColor && backgroundColor !== 'rgba(0, 0, 0, 0)' && backgroundColor !== 'transparent') {
                console.debug('Found background color:', backgroundColor, 'on element:', currentElement);
                return backgroundColor;
              }
              
              currentElement = currentElement.parentElement;
            }
            
            const bodyStyle = window.getComputedStyle(document.body);
            const bodyBackground = bodyStyle.backgroundColor;
            console.debug('Body background:', bodyBackground);
            if (bodyBackground && bodyBackground !== 'rgba(0, 0, 0, 0)' && bodyBackground !== 'transparent') {
              return bodyBackground;
            }
            
            const htmlStyle = window.getComputedStyle(document.documentElement);
            const htmlBackground = htmlStyle.backgroundColor;
            console.debug('HTML background:', htmlBackground);
            if (htmlBackground && htmlBackground !== 'rgba(0, 0, 0, 0)' && htmlBackground !== 'transparent') {
              return htmlBackground;
            }
            
            console.debug('No background found, returning default white');
            return 'rgb(255, 255, 255)';
          }
          
          function autoSelectTheme(container) {
            const backgroundColor = detectBackgroundColor(container);
            const rgb = parseColor(backgroundColor);
            
            console.debug('TRAE Badge Plugin - Background detection:', {
              container,
              backgroundColor,
              rgb
            });
            
            if (!rgb) {
              console.debug('TRAE Badge Plugin - No RGB found, using dark theme');
              return 'dark';
            }
            
            const [r, g, b] = rgb;
            const luminance = getLuminance(r, g, b);
            const selectedTheme = luminance > 128 ? 'dark' : 'light';
            
            console.debug('TRAE Badge Plugin - Theme selection:', {
              luminance,
              selectedTheme,
              reason: luminance > 128 ? 'Light background -> Dark badge' : 'Dark background -> Light badge'
            });
            
            return selectedTheme;
          }
        `;

        const badgeScript = `
          <script>
            (function() {
              'use strict';
              
              if (window.TraeBadgePlugin) return;
              
              const css = \`${BADGE_CSS}\`;
              const logos = ${JSON.stringify(TRAE_LOGOS)};
              const cancelDark = \`${CANCEL_SVG_DARK}\`;
              const cancelLight = \`${CANCEL_SVG_LIGHT}\`;
              const shareIcon = \`${SHARE_SVG}\`;
              
              ${themeFunctions}
              
              // 注入样式
              const style = document.createElement('style');
              style.id = 'trae-badge-plugin-styles';
              style.textContent = css;
              document.head.appendChild(style);
              
              // 创建 Badge 函数
              function createBadge() {
                // 移除已存在的 badge
                const existingBadge = document.getElementById('trae-badge-plugin');
                if (existingBadge) {
                  existingBadge.remove();
                }
                
                // 确定主题
                let variant = '${config.variant}';
                if (${config.autoTheme}) {
                  variant = autoSelectTheme('${config.autoThemeTarget}');
                }
                
                const isDarkBackground = variant === 'dark';
                const logoType = isDarkBackground ? 'light' : 'dark';
                const cancelIcon = isDarkBackground ? cancelLight : cancelDark;
                
                const badge = document.createElement('div');
                badge.id = 'trae-badge-plugin';
                badge.className = \`trae-badge trae-badge--\${variant} trae-badge--${config.position}\`;
                badge.innerHTML = \`
                  <div class="trae-badge-content">
                    <div class="trae-badge-logo">\${logos[logoType]}</div>
                    <div class="trae-badge-share">\${shareIcon}</div>
                  </div>
                  <div class="trae-badge-cancel">\${cancelIcon}</div>
                \`;
                
                // 添加点击事件
                ${config.clickable ? `
                badge.style.cursor = 'pointer';
                badge.addEventListener('click', (e) => {
                  if (e.target.closest('.trae-badge-cancel')) return;
                  ${config.clickUrl ? `window.open('${config.clickUrl}', '_blank');` : `console.debug('TRAE Badge clicked (plugin)');`}
                });
                ` : ''}
                
                // 添加取消按钮事件
                const cancelButton = badge.querySelector('.trae-badge-cancel');
                if (cancelButton) {
                  cancelButton.addEventListener('click', (e) => {
                    e.stopPropagation();
                    badge.remove();
                  });
                }
                
                document.body.appendChild(badge);
                return badge;
              }
              
              // 初始创建 badge
              // 等待 React 组件渲染完成后再创建 badge
              function initBadge() {
                // 使用 requestAnimationFrame 等待渲染完成
                requestAnimationFrame(() => {
                  // 再次检查目标元素是否存在
                  const targetElement = document.querySelector('${config.autoThemeTarget}');
                  if (targetElement) {
                    const badge = createBadge();
                  } else {
                    // 如果还没有找到，等待一段时间再试
                    setTimeout(() => {
                      const badge = createBadge();
                    }, 500);
                  }
                });
              }
              
              if (document.readyState === 'loading') {
                document.addEventListener('DOMContentLoaded', initBadge);
              } else {
                // DOM 已经加载完成，但 React 可能还没渲染
                initBadge();
              }
              
              window.TraeBadgePlugin = { 
                destroy: () => {
                  const badge = document.getElementById('trae-badge-plugin');
                  if (badge) badge.remove();
                  style.remove();
                  delete window.TraeBadgePlugin;
                },
                recreate: createBadge
              };
            })();
          </script>
        `;

        return html.replace('</body>', `${badgeScript}\n</body>`);
      }
    }
  };
}

export default traeBadgePlugin;