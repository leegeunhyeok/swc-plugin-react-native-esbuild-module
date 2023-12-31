use super::ReactNativeEsbuildModule;
use swc_core::ecma::{
    transforms::testing::test,
    visit::{as_folder, Folder},
};

fn plugin() -> Folder<ReactNativeEsbuildModule> {
    as_folder(ReactNativeEsbuildModule {
        module_name: String::from("test.js"),
        runtime_module: true,
    })
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
    global.__modules.export("test.js", null);
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
    global.__modules.export("test.js", null);
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
    var useContext = global.__modules.import("react").useContext;
    global.__modules.export("test.js", null);
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
    global.__modules.export("test.js", null);
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
    global.__modules.export("test.js", null);
    "#
);
