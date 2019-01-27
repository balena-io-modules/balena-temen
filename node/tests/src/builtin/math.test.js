const bt = require('balena-temen');

test('POW(10, 2) is 100', () => {
    const result = {
        result: 100
    };

    expect(
        bt.evaluate({
            "result": {
                "$$formula": "POW(10, 2)"
            }
        })
    ).toMatchObject(result);
});

test('POW(5 + 5, 2 * 3 - 4) is 100', () => {
    const result = {
        result: 100
    };

    expect(
        bt.evaluate({
            "result": {
                "$$formula": "POW(5 + 5, 2 * 3 - 4)"
            }
        })
    ).toMatchObject(result);
});


test('LOG10(1) is 0', () => {
    const result = {
        result: 0
    };

    expect(
        bt.evaluate({
            "result": {
                "$$formula": "LOG10(1)"
            }
        })
    ).toMatchObject(result);
});
