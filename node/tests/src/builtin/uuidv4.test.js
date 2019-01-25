const bt = require('balena-temen');

test('UUIDV4() generates proper random UUID', () => {
    const result = {
        id: expect.stringMatching(/^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/)
    };

    expect(
        bt.evaluate({
            "id": {
                "$$formula": "UUIDV4()"
            }
        })
    ).toMatchObject(result);
});
