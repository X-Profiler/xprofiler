'use strict';

const xprofiler = require('../../../');
xprofiler.start();

const bs = 4 * 1024 * 1024; // 4 MiB
const retained = [];
let i = 0, flag = false;

const rssMap = {
  before: [],
  after: [],
};

function tick() {
  i++;
  const rss = Math.round(process.memoryUsage().rss / 1024 / 1024);
  if (i % 1000 === 0) {
    console.log(`RSS [${i}]: ${rss} MiB`);
    if (flag) {
      rssMap.after.push(rss);
    } else {
      rssMap.before.push(rss);
    }
  }
  retained.push(Buffer.allocUnsafe(bs));
  if (i === 5000) {
    console.log('Clearing retained and enabling alloc');
    retained.length = 0;
    flag = true;
  }
  if (flag) {
    // Buffer.alloc(bs - 10) seems to be fine here
    Buffer.alloc(bs);
  }

  if (i < 10000 && rss < 2 * 1024) {
    setImmediate(tick);
  } else {
    process.send(rssMap)
  }
}

tick();