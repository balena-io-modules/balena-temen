const t = require('balena-temen');

console.log(
    t.temen_evaluate({
        "ssid": "Some Cool SSID!",
        "id": {
            "$$eval": "super.ssid | slugify"
        }
    })
);
