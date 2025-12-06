import { describe, expect, test } from "vitest";
import { envl } from "./envl.js";

describe("envl", () => {
    test("vars check", () => {
        expect(envl).toStrictEqual({
            a: "123",
            b: 123,
            c: true,
            d: [123, 456],
            e: {
                v: { a: "hello world" },
                x: 111,
                y: false,
                z: ["hello", "world"],
                w: [{ a: "hi!" }]
            },
            f: [[true], [false]],
            g: 123,
            h: 123,
            i: null
        });
    });
});
