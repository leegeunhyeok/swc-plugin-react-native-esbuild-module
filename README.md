# react-native-esbuild-module-plugin

Transform module imports statement for [@react-native-esbuild](https://github.com/leegeunhyeok/react-native-esbuild)'s custom module system.

> [!WARNING]
> This plugin is for custom module system to implement Hot Module Replacement(HMR) in some bundlers that don't support it.

## Installation

```bash
npm install react-native-esbuild-module-plugin
# or yarn
yarn add react-native-esbuild-module-plugin
```

## Usage

Inject global module manager to top of bundle.

```js
!((global) => {
  const _m: {};
  global.__modules = {
    get(moduleName) {
      return (
        _m[moduleName] ||
        (() => {
          throw new Error(`"${moduleName}" module not found`);
        })()
      );
    },
  };
})(
  typeof globalThis !== 'undefined'
    ? globalThis
    : typeof global !== 'undefined'
    ? global
    : typeof window !== 'undefined'
    ? window
    : this,
);
```

and add plugin to your swc options.

```ts
import { transform } from '@swc/core';

await transform(code, {
  jsc: {
    experimental: {
      plugins: [
        // Add plugin here
        ['react-native-esbuild-module-plugin', {}],
      ],
    },
  },
});
```

## Preview

```ts
// Before
import React, { useState, useEffect } from 'react';
import { Container, Section, Button, Text } from '@app/components';
import { useCustomHook } from '@app/hooks';

export function MyComponent (): JSX.Element {
  // ...
}

// After
var React = global.__modules.get("react").default;
var useState = global.__modules.get("react").useState;
var useEffect = global.__modules.get("react").useEffect;
var Container = global.__modules.get("@app/components").Container;
var Section = global.__modules.get("@app/components").Section;
var Button = global.__modules.get("@app/components").Button;
var Text = global.__modules.get("@app/components").Text;
var useCustomHook = global.__modules.get("@app/hooks").useCustomHook;

export function MyComponent () {
  // ...
}
```

## License

[LICENSE](./LICENSE)
