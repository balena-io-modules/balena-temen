const bt = require('balena-temen');

test('now() generates timestamp', () => {
    const result = {
        timestamp: expect.any(Number)
    };

    expect(
        bt.evaluate({
            "timestamp": {
                "$$formula": "now(timestamp=true)"
            }
        })
    ).toMatchObject(result);
});

test('now() generates rfc 3339', () => {
    const result = {
        date: expect.stringMatching(/^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}\.[0-9]{3}\+[0-9]{2}:[0-9]{2}$/)
    };

    expect(
        bt.evaluate({
            "date": {
                "$$formula": "now()"
            }
        })
    ).toMatchObject(result);
});
