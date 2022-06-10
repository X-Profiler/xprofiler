'use strict';

const build = require('./build');
const { os7u, os8u } = require('./versions');

const all = [].concat(os7u).concat(os8u);
build(all);
