const temen = import('./temen');

temen.then(m => m.temen_evaluate({
    "number": 3,
    "value": {"$$eval": "super.number + 5"}
})).catch(console.error).then(value => {
    console.log(value)
});
