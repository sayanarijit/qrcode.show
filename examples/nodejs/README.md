# Nodejs Usage Example
> Using [Got](https://www.npmjs.com/package/got) -  _Lightweight HTTP request library for Node.js_

## Basic Usage
```js
const got = require('got');
(async () => {
	try {
		const response = await got('http://qrcode.show/this+is+nodejs');
		console.log(response.body);
	} catch (error) {
		console.log(error.response.body);
	}
})()
```
## Adjust Width & Height
```js
(async () => {
	try {
		const response = await got('http://qrcode.show/this+is+nodejs', {
			headers:{
				'X-QR-Width': '50',
				'X-QR-Height': '50'
			}
		});
		console.log(response.body);
	} catch (error) {
		console.log(error.response.body);
	}
})();
```
## Set QR Version Type
```js
(async () => {
	try {
		const response = await got('http://qrcode.show/this+is+nodejs', {
			headers:{
				'X-QR-Version-Type':'micro'
			}
		});
		console.log(response.body);
	} catch (error) {
		console.log(error.response.body);
	}
})();
```
List of all headers : [http://qrcode.show/](http://qrcode.show/)
