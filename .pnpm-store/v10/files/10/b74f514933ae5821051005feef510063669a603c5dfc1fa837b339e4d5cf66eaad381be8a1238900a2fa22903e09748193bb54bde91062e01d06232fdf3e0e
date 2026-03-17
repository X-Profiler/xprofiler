"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.keyboardEventsMiddleware = exports.keyDownAction = exports.focusAction = exports.blurAction = void 0;
var _toolkit = require("@reduxjs/toolkit");
var _tooltipSlice = require("./tooltipSlice");
var _tooltipSelectors = require("./selectors/tooltipSelectors");
var _selectors = require("./selectors/selectors");
var _axisSelectors = require("./selectors/axisSelectors");
var _combineActiveTooltipIndex = require("./selectors/combiners/combineActiveTooltipIndex");
var keyDownAction = exports.keyDownAction = (0, _toolkit.createAction)('keyDown');
var focusAction = exports.focusAction = (0, _toolkit.createAction)('focus');
var blurAction = exports.blurAction = (0, _toolkit.createAction)('blur');
var keyboardEventsMiddleware = exports.keyboardEventsMiddleware = (0, _toolkit.createListenerMiddleware)();
var rafId = null;
var timeoutId = null;
var latestKeyboardActionPayload = null;
keyboardEventsMiddleware.startListening({
  actionCreator: keyDownAction,
  effect: (action, listenerApi) => {
    latestKeyboardActionPayload = action.payload;
    if (rafId !== null) {
      cancelAnimationFrame(rafId);
      rafId = null;
    }
    var state = listenerApi.getState();
    var {
      throttleDelay,
      throttledEvents
    } = state.eventSettings;
    var isThrottled = throttledEvents === 'all' || throttledEvents.includes('keydown');
    if (timeoutId !== null && (typeof throttleDelay !== 'number' || !isThrottled)) {
      clearTimeout(timeoutId);
      timeoutId = null;
    }
    var callback = () => {
      try {
        var currentState = listenerApi.getState();
        var accessibilityLayerIsActive = currentState.rootProps.accessibilityLayer !== false;
        if (!accessibilityLayerIsActive) {
          return;
        }
        var {
          keyboardInteraction
        } = currentState.tooltip;
        var key = latestKeyboardActionPayload;
        if (key !== 'ArrowRight' && key !== 'ArrowLeft' && key !== 'Enter') {
          return;
        }

        // TODO this is lacking index for charts that do not support numeric indexes
        var resolvedIndex = (0, _combineActiveTooltipIndex.combineActiveTooltipIndex)(keyboardInteraction, (0, _tooltipSelectors.selectTooltipDisplayedData)(currentState), (0, _axisSelectors.selectTooltipAxisDataKey)(currentState), (0, _tooltipSelectors.selectTooltipAxisDomain)(currentState));
        var currentIndex = resolvedIndex == null ? -1 : Number(resolvedIndex);
        if (!Number.isFinite(currentIndex) || currentIndex < 0) {
          return;
        }
        var tooltipTicks = (0, _tooltipSelectors.selectTooltipAxisTicks)(currentState);
        if (key === 'Enter') {
          var _coordinate = (0, _selectors.selectCoordinateForDefaultIndex)(currentState, 'axis', 'hover', String(keyboardInteraction.index));
          listenerApi.dispatch((0, _tooltipSlice.setKeyboardInteraction)({
            active: !keyboardInteraction.active,
            activeIndex: keyboardInteraction.index,
            activeCoordinate: _coordinate
          }));
          return;
        }
        var direction = (0, _axisSelectors.selectChartDirection)(currentState);
        var directionMultiplier = direction === 'left-to-right' ? 1 : -1;
        var movement = key === 'ArrowRight' ? 1 : -1;
        var nextIndex = currentIndex + movement * directionMultiplier;
        if (tooltipTicks == null || nextIndex >= tooltipTicks.length || nextIndex < 0) {
          return;
        }
        var coordinate = (0, _selectors.selectCoordinateForDefaultIndex)(currentState, 'axis', 'hover', String(nextIndex));
        listenerApi.dispatch((0, _tooltipSlice.setKeyboardInteraction)({
          active: true,
          activeIndex: nextIndex.toString(),
          activeCoordinate: coordinate
        }));
      } finally {
        rafId = null;
        timeoutId = null;
      }
    };
    if (!isThrottled) {
      callback();
      return;
    }
    if (throttleDelay === 'raf') {
      rafId = requestAnimationFrame(callback);
    } else if (typeof throttleDelay === 'number') {
      if (timeoutId === null) {
        callback();
        latestKeyboardActionPayload = null;
        timeoutId = setTimeout(() => {
          if (latestKeyboardActionPayload) {
            callback();
          } else {
            timeoutId = null;
            rafId = null;
          }
        }, throttleDelay);
      }
    }
  }
});
keyboardEventsMiddleware.startListening({
  actionCreator: focusAction,
  effect: (_action, listenerApi) => {
    var state = listenerApi.getState();
    var accessibilityLayerIsActive = state.rootProps.accessibilityLayer !== false;
    if (!accessibilityLayerIsActive) {
      return;
    }
    var {
      keyboardInteraction
    } = state.tooltip;
    if (keyboardInteraction.active) {
      return;
    }
    if (keyboardInteraction.index == null) {
      var nextIndex = '0';
      var coordinate = (0, _selectors.selectCoordinateForDefaultIndex)(state, 'axis', 'hover', String(nextIndex));
      listenerApi.dispatch((0, _tooltipSlice.setKeyboardInteraction)({
        active: true,
        activeIndex: nextIndex,
        activeCoordinate: coordinate
      }));
    }
  }
});
keyboardEventsMiddleware.startListening({
  actionCreator: blurAction,
  effect: (_action, listenerApi) => {
    var state = listenerApi.getState();
    var accessibilityLayerIsActive = state.rootProps.accessibilityLayer !== false;
    if (!accessibilityLayerIsActive) {
      return;
    }
    var {
      keyboardInteraction
    } = state.tooltip;
    if (keyboardInteraction.active) {
      listenerApi.dispatch((0, _tooltipSlice.setKeyboardInteraction)({
        active: false,
        activeIndex: keyboardInteraction.index,
        activeCoordinate: keyboardInteraction.coordinate
      }));
    }
  }
});