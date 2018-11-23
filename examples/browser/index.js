import * as temen from "balena-temen";

console.log(temen.evaluate({
    "ssid": "Some Cool SSID Network!",
    "id": {
        "$$eval": "super.ssid | slugify"
    }
}));
