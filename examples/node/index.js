const temen = require('balena-temen');

console.log(
    temen.evaluate({
        "ssid": "Some Cool SSID!",
        "id": {
            "$$eval": "super.ssid | slugify"
        }
    })
);
