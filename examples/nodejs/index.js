const got = require('got');
(async () => {

	try {
		const qrcode = await got('http://qrcode.show/this+is+nodejs');
		const qrcode50 = await got('http://qrcode.show/this+is+nodejs', {
			headers:{
				'X-QR-Width': '50',
				'X-QR-Height': '50'
			}
		});
		const qrcodeVer = await got('http://qrcode.show/this+is+nodejs', {
			headers:{
				'X-QR-Version-Type':'micro'
			}
		});

		console.log(qrcode.body);
		console.log(qrcode50.body);
		console.log(qrcodeVer.body);

	} catch (error) {
		console.log(error.response.body);
	}

})()