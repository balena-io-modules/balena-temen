const bt = require('balena-temen');

test('evaluate fn succeeds', () => {
    expect(
        bt.evaluate({
            "math": {
                "$$eval": "5 + 10"
            }
        })
    ).toEqual(
        {"math": 15}
    );
});

test('evaluate fn throws', () => {
    expect(
        () => {
            bt.evaluate({
                "prop": {
                    "$$eval": "super.notExistingProperty"
                }
            });
        }
    ).toThrow();
});
