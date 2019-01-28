const bt = require('balena-temen');

test('evaluate fn succeeds', () => {
    expect(
        bt.evaluate({
            "math": {
                "$$formula": "5 + 10"
            }
        })
    ).toEqual(
        { "math": 15 }
    );
});

test('evaluate fn throws', () => {
    expect(
        () => {
            bt.evaluate({
                "prop": {
                    "$$formula": "super.notExistingProperty"
                }
            });
        }
    ).toThrow();
});

test('evaluate fn throws with one failing and one succeeding formula', () => {
    expect(
        () => {
            bt.evaluate({
                "foo": {
                    "$$formula": "UUIDV4()"
                },
                "prop": {
                    "$$formula": "super.notExistingProperty"
                }
            });
        }
    ).toThrow();
});

test('chained evaluation', () => {
    expect(
        bt.evaluate({
            "first": "a",
            "second": {
                "$$formula": "first ~ `a`"
            },
            "third": {
                "$$formula": "second ~ `a`"
            },
            "fourth": {
                "$$formula": "third ~ `a`"
            }
        })
    ).toEqual({
        "first": "a",
        "second": "aa",
        "third": "aaa",
        "fourth": "aaaa"
    });
});
