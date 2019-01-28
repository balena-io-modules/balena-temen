const bt = require('balena-temen');

test('NOW(true) generates timestamp', () => {
    const result = {
        timestamp: expect.any(Number)
    };

    expect(
        bt.evaluate({
            "timestamp": {
                "$$formula": "NOW(true)"
            }
        })
    ).toMatchObject(result);
});

test('NOW() & NOW(false) generate rfc 3339', () => {
    const result = {
        rfc3339implicit: expect.stringMatching(/^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}\.[0-9]{3}\+[0-9]{2}:[0-9]{2}$/),
        rfc3339explicit: expect.stringMatching(/^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}\.[0-9]{3}\+[0-9]{2}:[0-9]{2}$/)
    };

    expect(
        bt.evaluate({
            "rfc3339implicit": {
                "$$formula": "NOW()"
            },
            "rfc3339explicit": {
                "$$formula": "NOW(false)"
            }
        })
    ).toMatchObject(result);
});
