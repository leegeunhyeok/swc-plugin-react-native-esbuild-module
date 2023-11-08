use super::ReactNativeEsbuildModule;
use swc_core::ecma::{transforms::testing::test, visit::as_folder};

test!(
    Default::default(),
    |_| as_folder(ReactNativeEsbuildModule::default()),
    default_import,
    // Input codes
    r#"
    import React from 'react';
    "#,
    // Output codes after transformed with plugin
    r#"
    var React = global.__modules["react"].default;
    "#
);

test!(
    Default::default(),
    |_| as_folder(ReactNativeEsbuildModule::default()),
    named_import,
    // Input codes
    r#"
    import { useState, useContext } from 'react';
    "#,
    // Output codes after transformed with plugin
    r#"
    var useState = global.__modules["react"].useState;
    var useContext = global.__modules["react"].useContext;
    "#
);

test!(
    Default::default(),
    |_| as_folder(ReactNativeEsbuildModule::default()),
    mixed_import,
    // Input codes
    r#"
    import React, { useState, useContext } from 'react';
    "#,
    // Output codes after transformed with plugin
    r#"
    var React = global.__modules["react"].default;
    var useState = global.__modules["react"].useState;
    var useContext = global.__modules["react"].useContext;
    "#
);
