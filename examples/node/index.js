const bt = require('balena-temen');

console.log(
    bt.evaluate({
        "ssid": "Some Cool SSID!",
        "id": {
            "$$formula": "super.ssid | slugify"
        }
    })
);
