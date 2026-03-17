"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.setRenderedTicks = exports.renderedTicksSlice = exports.renderedTicksReducer = exports.removeRenderedTicks = void 0;
var _toolkit = require("@reduxjs/toolkit");
var _immer = require("immer");
/**
 * @fileOverview this stores actually rendered ticks.
 *
 * What we do is that we have the domain -> ticks mapping in the cartesianSlice,
 * which is fine but the result then goes to CartesianAxis where we use DOM measurement
 * to decide which ticks to actually render.
 *
 * This renderedTickSlice stores those actually rendered ticks so that we can return them from a hook later.
 */

var initialState = {
  xAxis: {},
  yAxis: {}
};
var renderedTicksSlice = exports.renderedTicksSlice = (0, _toolkit.createSlice)({
  name: 'renderedTicks',
  initialState,
  reducers: {
    setRenderedTicks: (state, action) => {
      var {
        axisType,
        axisId,
        ticks
      } = action.payload;
      state[axisType][axisId] = (0, _immer.castDraft)(ticks);
    },
    removeRenderedTicks: (state, action) => {
      var {
        axisType,
        axisId
      } = action.payload;
      delete state[axisType][axisId];
    }
  }
});
var {
  setRenderedTicks,
  removeRenderedTicks
} = renderedTicksSlice.actions;
exports.removeRenderedTicks = removeRenderedTicks;
exports.setRenderedTicks = setRenderedTicks;
var renderedTicksReducer = exports.renderedTicksReducer = renderedTicksSlice.reducer;