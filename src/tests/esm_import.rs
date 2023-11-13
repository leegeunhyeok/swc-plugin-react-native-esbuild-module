use super::ReactNativeEsbuildModule;
use swc_core::ecma::{
    transforms::testing::test,
    visit::{as_folder, Folder},
};

fn plugin() -> Folder<ReactNativeEsbuildModule> {
    as_folder(ReactNativeEsbuildModule::default(String::from("test.js")))
}

test!(
    Default::default(),
    |_| plugin(),
    default_import,
    // Input codes
    r#"
    import React from 'react';
    "#,
    // Output codes after transformed with plugin
    r#"
    var React = global.__modules.import("react").default;
    "#
);

test!(
    Default::default(),
    |_| plugin(),
    named_import,
    // Input codes
    r#"
    import { useState, useContext } from 'react';
    "#,
    // Output codes after transformed with plugin
    r#"
    var useState = global.__modules.import("react").useState;
    var useContext = global.__modules.import("react").useContext;
    "#
);

test!(
    Default::default(),
    |_| plugin(),
    mixed_import,
    // Input codes
    r#"
    import React, { useState, useContext } from 'react';
    "#,
    // Output codes after transformed with plugin
    r#"
    var React = global.__modules.import("react").default;
    var useState = global.__modules.import("react").useState;
    var useContext = global.__modules.import("react").useContext
    "#
);

test!(
    Default::default(),
    |_| plugin(),
    import_all,
    // Input codes
    r#"
    import * as ReactAll from 'react';
    "#,
    // Output codes after transformed with plugin
    r#"
    var ReactAll = global.__modules.import("react");
    "#
);

test!(
    Default::default(),
    |_| plugin(),
    import_with_stmt,
    // Input codes
    r#"
    import React, { useState, useContext } from 'react';
    function testFn() {}
    class TestClass{}
    "#,
    // Output codes after transformed with plugin
    r#"
    var React = global.__modules.import("react").default;
    var useState = global.__modules.import("react").useState;
    var useContext = global.__modules.import("react").useContext;
    function testFn() {}
    class TestClass {}
    "#
);
