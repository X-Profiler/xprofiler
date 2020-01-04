'use strict';

function defineProperty(obj, name, value) {
  const enumerable = !!obj[name] && obj.propertyIsEnumerable(name); // eslint-disable-line
  Object.defineProperty(obj, name, {
    configurable: true,
    enumerable,
    writable: true,
    value: value
  });
}

function isFunction(funktion) {
  return typeof funktion === 'function';
}

function wrap(nodule, name, wrapper) {
  if (
    !nodule ||
    !nodule[name] ||
    !wrapper ||
    !isFunction(nodule[name]) ||
    !isFunction(wrapper)
  ) {
    return;
  }

  const original = nodule[name];
  const wrapped = wrapper(original, name);

  defineProperty(wrapped, '__original', original);
  defineProperty(wrapped, '__wrapped', true);

  defineProperty(nodule, name, wrapped);

  defineProperty(wrapped, '__unwrap', function () {
    if (nodule[name] === wrapped) { defineProperty(nodule, name, original); }
  });
  return wrapped;
}

function unwrap(nodule, name) {
  if (
    !nodule ||
    !nodule[name] ||
    !nodule[name].__unwrap) {
    return;
  }

  return nodule[name].__unwrap();
}

module.exports = { wrap, unwrap };