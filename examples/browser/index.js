import * as temen from "balena-temen";

console.log(temen.temen_evaluate({
    "ssid": "Some Cool SSID Network!",
    "id": {
        "$$eval": "super.ssid | slugify"
    }
}));
