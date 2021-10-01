const got = require('got');
const fs = require('fs');

(async ()=>{

	const basicQrcode = await got.post('http://qrcode.show/', {
		body: 'This is Nodejs',
	});
	console.log(basicQrcode.body);
	
	const qrcodePng = await got.post('http://qrcode.show/',{
		body: 'This is Nodejs',
		headers:{
			accept:'image/png'
		}
	});
	fs.writeFileSync('./qrcode.png',response.rawBody)
	
	const bigQrcode = await got.post('http://qrcode.show/', {
		body: 'This is Nodejs',
		headers:{
			'X-QR-Width': '100',
			'X-QR-Height': '100'
		}
	});
	console.log(bigQrcode.body);


})