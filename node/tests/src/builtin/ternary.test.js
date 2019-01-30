const bt = require('balena-temen');

test('bool value as condition', () => {
    expect(bt.evaluate({
        "$$formula": "(true ? `yes` : `no`)"
    })).toMatch("yes");
    expect(bt.evaluate({
        "$$formula": "(false ? `yes` : `no`)"
    })).toMatch("no");
});

test('fn in condition', () => {
    expect(bt.evaluate({
        "$$formula": "(POW(10, 2) == 100 ? `yes` : `no`)"
    })).toMatch("yes");
});

test('fn in condition', () => {
    expect(bt.evaluate({
        "$$formula": "(`a` ~ `b` == `ab` ? `yes` : `no`)"
    })).toMatch("yes");
});

test('nested', () => {
    expect(bt.evaluate({
        "$$formula": "(true ? (true ? (true ? (true ? `yes` : `no`) : `no`) : `no`) : `no`)"
    })).toMatch("yes");
});

test('math expression as truthy', () => {
    expect(bt.evaluate({
        "$$formula": "(true ? 3 + 5 : 0)"
    })).toEqual(8);
});

test('math expression as falsy', () => {
    expect(bt.evaluate({
        "$$formula": "(not true ? 0 : 3 + 5)"
    })).toEqual(8);
});

test('comparison in condition', () => {
    expect(bt.evaluate({
        "$$formula": "(3 > 2 ? `yes` : `no`)"
    })).toMatch("yes");
});

test('math & comparison in condition', () => {
    expect(bt.evaluate({
        "$$formula": "(1 + 2 + 3 == 2 * 3 ? `yes` : `no`)"
    })).toMatch("yes");
});

test('price calculator', () => {
    expect(bt.evaluate({
        devices: 20,
        months: 12,
        yearDiscount: 0.2,
        "price": {
            "$$formula": "devices * (1 - (((months > 11 ? 13 : 0) + (months - 1)) / 24) * yearDiscount)"
        }
    })).toMatchObject({
        devices: 20,
        months: 12,
        yearDiscount: 0.2,
        price: 16
    });
});
