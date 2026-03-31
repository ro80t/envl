# Environment Variables Language (envl)

## Packages

|language|lib                                  |repository                            |
|--------|-------------------------------------|--------------------------------------|
|Rust    |[envl](https://crates.io/crates/envl)|[ro80t/envl](./packages/envl/)|

## Cli

|name    |pm                                              |repository                                |
|--------|------------------------------------------------|------------------------------------------|
|envl-cli|[crates.io](https://crates.io/crates/envl-cli)  |[ro80t/envl](./packages/envl-cli/)|
|envl-cli|[npmjs.com](https://npmjs.com/packages/envl-cli)|[ro80t/envl](./packages/envl-cli/)|

## Examples

**.envl**
```
a = "123";
b = 123;
c = true;
d = [123, 456];
e = struct {
    v: struct {
        a: "hello world"
    },
    w: [
        struct {
            a: "hi!"
        }
    ],
    x: 111,
    y: false,
    z: ["hello", "world"],
};
f = [
    [true],
    [false]
];
h = 123;
```

**.envlconf**
```
settings {}

vars {
    a: string,
    b: int,
    c: bool,
    d: Array<int>,
    e: struct {
        v: struct {
            a: string;
        };
        w: Array<struct {
            a: string;
        }>;
        x: int;
        y: bool;
        z: Array<string>;
    },
    f: Array<Array<bool>>,
    g: int (
        default: 123
    ),
    h: Option<int>,
    i: Option<string>
}
```
