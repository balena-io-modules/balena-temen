const bt = require('balena-temen');

test('NOW() generates timestamp', () => {
    const result = {
        timestamp: expect.any(Number)
    };

    expect(
        bt.evaluate({
            "timestamp": {
                "$$formula": "NOW(true, true)"
            }
        })
    ).toMatchObject(result);
});

test('NOW() generates rfc 3339', () => {
    const result = {
        date: expect.stringMatching(/^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}\.[0-9]{3}\+[0-9]{2}:[0-9]{2}$/)
    };

    expect(
        bt.evaluate({
            "date": {
                "$$formula": "NOW()"
            }
        })
    ).toMatchObject(result);
});
