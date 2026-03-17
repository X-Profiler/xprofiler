import { createAction, createListenerMiddleware } from '@reduxjs/toolkit';
import { setKeyboardInteraction } from './tooltipSlice';
import { selectTooltipAxisDomain, selectTooltipAxisTicks, selectTooltipDisplayedData } from './selectors/tooltipSelectors';
import { selectCoordinateForDefaultIndex } from './selectors/selectors';
import { selectChartDirection, selectTooltipAxisDataKey } from './selectors/axisSelectors';
import { combineActiveTooltipIndex } from './selectors/combiners/combineActiveTooltipIndex';
export var keyDownAction = createAction('keyDown');
export var focusAction = createAction('focus');
export var blurAction = createAction('blur');
export var keyboardEventsMiddleware = createListenerMiddleware();
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
        var resolvedIndex = combineActiveTooltipIndex(keyboardInteraction, selectTooltipDisplayedData(currentState), selectTooltipAxisDataKey(currentState), selectTooltipAxisDomain(currentState));
        var currentIndex = resolvedIndex == null ? -1 : Number(resolvedIndex);
        if (!Number.isFinite(currentIndex) || currentIndex < 0) {
          return;
        }
        var tooltipTicks = selectTooltipAxisTicks(currentState);
        if (key === 'Enter') {
          var _coordinate = selectCoordinateForDefaultIndex(currentState, 'axis', 'hover', String(keyboardInteraction.index));
          listenerApi.dispatch(setKeyboardInteraction({
            active: !keyboardInteraction.active,
            activeIndex: keyboardInteraction.index,
            activeCoordinate: _coordinate
          }));
          return;
        }
        var direction = selectChartDirection(currentState);
        var directionMultiplier = direction === 'left-to-right' ? 1 : -1;
        var movement = key === 'ArrowRight' ? 1 : -1;
        var nextIndex = currentIndex + movement * directionMultiplier;
        if (tooltipTicks == null || nextIndex >= tooltipTicks.length || nextIndex < 0) {
          return;
        }
        var coordinate = selectCoordinateForDefaultIndex(currentState, 'axis', 'hover', String(nextIndex));
        listenerApi.dispatch(setKeyboardInteraction({
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
      var coordinate = selectCoordinateForDefaultIndex(state, 'axis', 'hover', String(nextIndex));
      listenerApi.dispatch(setKeyboardInteraction({
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
      listenerApi.dispatch(setKeyboardInteraction({
        active: false,
        activeIndex: keyboardInteraction.index,
        activeCoordinate: keyboardInteraction.coordinate
      }));
    }
  }
});