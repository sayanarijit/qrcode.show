# Nodejs Usage Example

> Using [Got](https://www.npmjs.com/package/got) -  _Lightweight HTTP request library for Node.js_

## Basic Usage
```js
const got = require('got');
const response = await got.post('http://qrcode.show/', {
	body: 'This is Nodejs',
});
```
## Adjust Width & Height
```js
const got = require('got');
const response = await got.post('http://qrcode.show/', {
	body: 'This is Nodejs',
	headers:{
		'X-QR-Width': '50',
		'X-QR-Height': '50'
	}
});
console.log(response.body);
```
## Save QRCode as PNG file
```js
const got = require('got');
const fs = require('fs');
const response = await got.post('http://qrcode.show/',{
	body: 'This is Nodejs',
	headers:{
		accept:'image/png'
	}
});
fs.writeFileSync('qrcode.png',response.rawBody)
```

### List of all headers
[http://qrcode.show/](http://qrcode.show/)