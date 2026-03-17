/**
 * TRAE Badge 常量定义
 */

// SVG 文件导入
import traeLightSvg from '../assets/trae-light.svg';
import traeDarkSvg from '../assets/trae-dark.svg';
import cancelDarkSvg from '../assets/stop-dark.svg';
import cancelLightSvg from '../assets/stop-light.svg';
import shareSvg from '../assets/share.svg';
// SVG Logo 类型定义
export type LogoVariant = 'light' | 'dark';

// TRAE Logo SVG 数据 (通过 import 导入)
export const TRAE_LOGOS: Record<LogoVariant, string> = {
  light: traeLightSvg,
  dark: traeDarkSvg
};

// 取消按钮 SVG
export const CANCEL_SVG_DARK = cancelDarkSvg;
export const CANCEL_SVG_LIGHT = cancelLightSvg;

// 分享按钮 SVG
export const SHARE_SVG = shareSvg;

// Badge CSS 样式 (更新以适配完整的 SVG)
export const BADGE_CSS = `
.trae-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
  font-weight: 500;
  transition: all 0.2s ease;
  cursor: pointer;
  box-sizing: border-box;
  border-radius: 6px;
  position: fixed;
  padding: 8px 8px 8px 10px;
  height: 28px;
  width: 130px;
  z-index: 9999;
}

.trae-badge:hover.trae-badge--light {
  background-color: #EDEFF2;
}

.trae-badge:hover.trae-badge--dark {
  background-color: #2A2D31;
}

.trae-badge-content {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
}

.trae-badge-logo {
  flex-shrink: 0;
}

.trae-badge-share {
  flex-shrink: 0;
  color: rgba(78, 82, 87, 1);
}

.trae-badge:hover.trae-badge--light .trae-badge-share {
  color: rgba(38, 41, 44, 1);
}

.trae-badge:hover.trae-badge--dark .trae-badge-share {
  color: #D1D3DB;
}

.trae-badge-cancel {
  position: absolute;
  top: -8px;
  right: -8px;
  width: 16px;
  height: 16px;
  cursor: pointer;
  opacity: 0;
  visibility: hidden;
  z-index: 10001;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #000000;
}

.trae-badge:hover .trae-badge-cancel {
  opacity: 1 !important;
  visibility: visible !important;
}

/* 在移动端设备上始终显示取消按钮 */
@media (hover: none) and (pointer: coarse) {
  .trae-badge-cancel {
    opacity: 1 !important;
    visibility: visible !important;
  }
}

.trae-badge--light {
  background-color: rgba(255, 255, 255, 1);
  color: #ffffff;
  box-shadow: 0px 0px 0.5px 0px rgba(0, 0, 0, 0.12), 0px 1px 3px 0px rgba(0, 0, 0, 0.08), 0px 4px 8px 0px rgba(0, 0, 0, 0.12);
}

.trae-badge--dark {
  background-color: #000000;
  color: #ffffff;
  box-shadow: 0px 0px 0.5px 0px rgba(0, 0, 0, 0.12), 0px 1px 3px 0px rgba(0, 0, 0, 0.08), 0px 4px 8px 0px rgba(0, 0, 0, 0.12);
}

.trae-badge--bottom-right { bottom: 20px; right: 20px; }
.trae-badge--bottom-left { bottom: 20px; left: 20px; }
.trae-badge--top-right { top: 20px; right: 20px; }
.trae-badge--top-left { top: 20px; left: 20px; }
`;

// 默认配置
export const DEFAULT_OPTIONS = {
  variant: 'dark' as const,
  position: 'bottom-right' as const,
  clickable: true,
  clickUrl: null,
  disabled: false
};