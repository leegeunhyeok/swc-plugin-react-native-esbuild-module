# react-native-esbuild-module-plugin

Transform module imports statement for [@react-native-esbuild](https://github.com/leegeunhyeok/react-native-esbuild)'s custom module system.

## Installation

```bash
npm install react-native-esbuild-module-plugin
# or yarn
yarn add react-native-esbuild-module-plugin
```

## Usage

Add plugin to your swc options.

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
var React = global.__modules["react"].default;
var useState = global.__modules["react"].useState;
var useEffect = global.__modules["react"].useEffect;
var Container = global.__modules["@app/components"].Container;
var Section = global.__modules["@app/components"].Section;
var Button = global.__modules["@app/components"].Button;
var Text = global.__modules["@app/components"].Text;
var useCustomHook = global.__modules["@app/hooks"].useCustomHook;

export function MyComponent () {
  // ...
}
```

## License

[LICENSE](./LICENSE)
