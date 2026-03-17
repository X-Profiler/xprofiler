"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.usePrefersReducedMotion = usePrefersReducedMotion;
var _react = require("react");
var _Global = require("./Global");
/**
 * Detects and subscribes to the user's `prefers-reduced-motion` system preference.
 * Returns `true` when the user prefers reduced motion, `false` otherwise.
 * SSR-safe: always returns `false` during server-side rendering.
 */
function usePrefersReducedMotion() {
  var [prefersReducedMotion, setPrefersReducedMotion] = (0, _react.useState)(() => {
    if (_Global.Global.isSsr) {
      return false;
    }
    if (!window.matchMedia) {
      return false;
    }
    return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  });
  (0, _react.useEffect)(() => {
    if (!window.matchMedia) {
      return;
    }
    var mediaQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
    var handleChange = () => {
      setPrefersReducedMotion(mediaQuery.matches);
    };
    mediaQuery.addEventListener('change', handleChange);
    // eslint-disable-next-line consistent-return
    return () => {
      mediaQuery.removeEventListener('change', handleChange);
    };
  }, []);
  return prefersReducedMotion;
}