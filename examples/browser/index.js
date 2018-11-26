import * as bt from "balena-temen";

console.log(
    bt.evaluate({
        "ssid": "Some Cool SSID Network!",
        "id": {
            "$$eval": "super.ssid | slugify"
        }
    })
);
