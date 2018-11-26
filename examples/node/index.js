const bt = require('balena-temen');

console.log(
    bt.evaluate({
        "ssid": "Some Cool SSID!",
        "id": {
            "$$eval": "super.ssid | slugify"
        }
    })
);
