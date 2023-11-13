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
  const __m: {};
  global.__modules = {
    import(moduleName) {
      return (
        __m[moduleName] ||
        (() => {
          throw new Error(`"${moduleName}" module not found`);
        })()
      );
    },
    export(moduleName, exports) {
      return __m[moduleName] = exports;
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
        // Add plugin here.
        ['react-native-esbuild-module-plugin', {}],
      ],
    },
    externalHelpers: false, // You should disable external helpers.
  },
});
```

## Preview

Before

```ts
import React, { useState, useEffect } from 'react';
import { Container, Section, Button, Text } from '@app/components';
import { useCustomHook } from '@app/hooks';
import * as app from '@app/core';

export function MyComponent (): JSX.Element {
  // ...
}

// anonymous class
export default class {}
```

After

```js
var React = global.__modules.import("react").default;
var useState = global.__modules.import("react").useState;
var useEffect = global.__modules.import("react").useEffect;
var Container = global.__modules.import("@app/components").Container;
var Section = global.__modules.import("@app/components").Section;
var Button = global.__modules.import("@app/components").Button;
var Text = global.__modules.import("@app/components").Text;
var useCustomHook = global.__modules.import("@app/hooks").useCustomHook;
var app = global.__modules.import("@app/core");

function MyComponent () {
  // ...
}

var __export_default = class {}

global.__modules.export("<module-file-name>", {
  "MyComponent": MyComponent,
  "default": __export_default
});
```

## License

[MIT](./LICENSE)
