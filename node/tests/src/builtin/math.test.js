const bt = require('balena-temen');

test('MIN(10, 2) is 2', () => {
    const result = {
        result: 2
    };

    expect(
        bt.evaluate({
            "result": {
                "$$formula": "MIN(10, 2)"
            }
        })
    ).toMatchObject(result);
});

test('MIN(1, 2, 3, 4, 5) is 1', () => {
    const result = {
        result: 1
    };

    expect(
        bt.evaluate({
            "result": {
                "$$formula": "MIN(1, 2, 3, 4, 5)"
            }
        })
    ).toMatchObject(result);
});

test('MAX(10, 2) is 10', () => {
    const result = {
        result: 10
    };

    expect(
        bt.evaluate({
            "result": {
                "$$formula": "MAX(10, 2)"
            }
        })
    ).toMatchObject(result);
});

test('MAX(1, 2, 3, 4, 5) is 5', () => {
    const result = {
        result: 5
    };

    expect(
        bt.evaluate({
            "result": {
                "$$formula": "MAX(1, 2, 3, 4, 5)"
            }
        })
    ).toMatchObject(result);
});

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
