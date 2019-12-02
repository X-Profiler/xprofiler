'use strict';

const fs = require('fs');
const os = require('os');
const path = require('path');
const crypto = require('crypto');
const urllib = require('urllib');
const formstream = require('formstream');
const tunnel = require('tunnel-agent');
const pack = require('../package.json');

const server = process.env.UPLOAD_SERVER;
const filepath = path.join(__dirname, '../build/Release/xprofiler.node');
const keyid = process.env.UPLOAD_KEY_ID;
const keysecret = process.env.UPLOAD_KEY_SECRET;

// check args
if (!server || !keyid || !keysecret) {
  console.error('参数错误: server, keyid , keysecret 必填');
  return;
}

// check filepath
fs.stat(filepath, function (err, stat) {
  if (err) {
    console.error(`文件 ${filepath} 不存在: ${err.message}`);
    return;
  }

  if (stat.size <= 0) {
    console.error(`文件 ${filepath} 为空文件`);
    return;
  }

  const form = formstream();
  form.file('file', filepath, filepath, stat.size);

  const nonce = '' + (1 + parseInt((Math.random() * 100000000000), 10));
  // get signature
  const shasum = crypto.createHash('sha1');
  const timestamp = Date.now();
  shasum.update([keyid, keysecret, nonce, timestamp].join(''));
  const sign = shasum.digest('hex');

  const url = 'http://' + server + '/file_upload?keyid=' + keyid +
    '&nonce=' + nonce + '&sign=' + sign + '&timestamp=' + timestamp +
    '&version=' + pack.version + '&modules=' + process.versions.modules +
    '&platform=' + os.platform() + '&arch=' + os.arch() + '&filename=xprofiler.node';

  let agent = false;
  if (process.env.http_proxy) {
    const parts = process.env.http_proxy.split(':');
    agent = tunnel.httpOverHttp({
      proxy: {
        host: parts[0],
        port: parts[1]
      }
    });
  }

  const opts = {
    dataType: 'json',
    type: 'POST',
    timeout: 60000 * 20,
    headers: form.headers(),
    stream: form,
    agent: agent
  };

  urllib.request(url, opts, function (err, data, res) {
    if (err) {
      console.error(err);
      return;
    }
    if (res.statusCode !== 200) {
      console.error({ statusCode: res.statusCode, data: JSON.stringify(data) });
      return;
    }
    if (!data.ok) {
      console.error(`上传失败: ${JSON.stringify(data)}`);
      return;
    }
    console.log(JSON.stringify(data));
  });
});
