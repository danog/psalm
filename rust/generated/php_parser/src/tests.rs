#![allow(unused_imports, unused_variables, unused_mut, dead_code, non_snake_case, non_camel_case_types, unreachable_code, unused_parens, unused_braces, unused_assignments, unused_labels, unused_unsafe, clippy::all, irrefutable_let_patterns, unreachable_patterns, unused_must_use, non_upper_case_globals, deprecated, ambiguous_glob_reexports, hidden_glob_reexports)]
use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
#[test]
fn PhpParser_Builder_ClassConstTest__testModifiers() {
    php_rt::testing::run("PhpParser\\Builder\\ClassConstTest::testModifiers", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassConstTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassConstTest::new(Str::from_static("testModifiers"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testModifiers();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassConstTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassConstTest__testDocComment() {
    php_rt::testing::run("PhpParser\\Builder\\ClassConstTest::testDocComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassConstTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassConstTest::new(Str::from_static("testDocComment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testDocComment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassConstTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassConstTest__testAddConst() {
    php_rt::testing::run("PhpParser\\Builder\\ClassConstTest::testAddConst", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassConstTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassConstTest::new(Str::from_static("testAddConst"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddConst();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassConstTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassConstTest__testAddAttribute() {
    php_rt::testing::run("PhpParser\\Builder\\ClassConstTest::testAddAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassConstTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassConstTest::new(Str::from_static("testAddAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassConstTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassConstTest__testType() {
    php_rt::testing::run("PhpParser\\Builder\\ClassConstTest::testType", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassConstTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassConstTest::new(Str::from_static("testType"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testType();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassConstTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassConstTest__testValues() {
    php_rt::testing::run("PhpParser\\Builder\\ClassConstTest::testValues", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassConstTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1832 = crate::php_parser::builder::ClassConstTest::provideTestDefaultValues()?; List::from_vec(vec![U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Opt_Null_PhpParser_Node_Expr_ConstFetch(__c1832.0), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Bool_PhpParser_Node_Expr_ConstFetch(__c1832.1), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Bool_PhpParser_Node_Expr_ConstFetch(__c1832.2), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Int_PhpParser_Node_Scalar_Int_(__c1832.3), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Float_PhpParser_Node_Scalar_Float_(__c1832.4), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Str_PhpParser_Node_Scalar_String_(__c1832.5), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Tup3_Int_Int_Int_PhpParser_Node_Expr_Array_(__c1832.6), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Shape_foo_Str_bar_Str_PhpParser_Node_Expr_Array_(__c1832.7), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_PhpParser_Node_Scalar_MagicConst_Dir_PhpParser_Node_Scalar_MagicConst_Dir(__c1832.8)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::builder::ClassConstTest::new(Str::from_static("testValues"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testValues((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::builder::ClassConstTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassTest__testExtendsImplements() {
    php_rt::testing::run("PhpParser\\Builder\\ClassTest::testExtendsImplements", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassTest::new(Str::from_static("testExtendsImplements"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testExtendsImplements();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassTest__testAbstract() {
    php_rt::testing::run("PhpParser\\Builder\\ClassTest::testAbstract", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassTest::new(Str::from_static("testAbstract"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAbstract();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassTest__testFinal() {
    php_rt::testing::run("PhpParser\\Builder\\ClassTest::testFinal", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassTest::new(Str::from_static("testFinal"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testFinal();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassTest__testReadonly() {
    php_rt::testing::run("PhpParser\\Builder\\ClassTest::testReadonly", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassTest::new(Str::from_static("testReadonly"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testReadonly();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassTest__testStatementOrder() {
    php_rt::testing::run("PhpParser\\Builder\\ClassTest::testStatementOrder", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassTest::new(Str::from_static("testStatementOrder"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testStatementOrder();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassTest__testDocComment() {
    php_rt::testing::run("PhpParser\\Builder\\ClassTest::testDocComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassTest::new(Str::from_static("testDocComment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testDocComment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassTest__testAddAttribute() {
    php_rt::testing::run("PhpParser\\Builder\\ClassTest::testAddAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassTest::new(Str::from_static("testAddAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassTest__testInvalidStmtError() {
    php_rt::testing::run("PhpParser\\Builder\\ClassTest::testInvalidStmtError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassTest::new(Str::from_static("testInvalidStmtError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidStmtError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassTest__testInvalidDocComment() {
    php_rt::testing::run("PhpParser\\Builder\\ClassTest::testInvalidDocComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassTest::new(Str::from_static("testInvalidDocComment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidDocComment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassTest__testEmptyName() {
    php_rt::testing::run("PhpParser\\Builder\\ClassTest::testEmptyName", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassTest::new(Str::from_static("testEmptyName"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testEmptyName();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ClassTest__testInvalidName() {
    php_rt::testing::run("PhpParser\\Builder\\ClassTest::testInvalidName", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ClassTest::new(Str::from_static("testInvalidName"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidName();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_EnumCaseTest__testDocComment() {
    php_rt::testing::run("PhpParser\\Builder\\EnumCaseTest::testDocComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::EnumCaseTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::EnumCaseTest::new(Str::from_static("testDocComment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testDocComment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::EnumCaseTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_EnumCaseTest__testAddAttribute() {
    php_rt::testing::run("PhpParser\\Builder\\EnumCaseTest::testAddAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::EnumCaseTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::EnumCaseTest::new(Str::from_static("testAddAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::EnumCaseTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_EnumCaseTest__testValues() {
    php_rt::testing::run("PhpParser\\Builder\\EnumCaseTest::testValues", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::EnumCaseTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1833 = crate::php_parser::builder::EnumCaseTest::provideTestDefaultValues()?; List::from_vec(vec![U_Tup2_Int_PhpParser_Node_Scalar_Int__or_Tup2_Str_PhpParser_Node_Scalar_String_::Tup2_Int_PhpParser_Node_Scalar_Int_(__c1833.0), U_Tup2_Int_PhpParser_Node_Scalar_Int__or_Tup2_Str_PhpParser_Node_Scalar_String_::Tup2_Str_PhpParser_Node_Scalar_String_(__c1833.1)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::builder::EnumCaseTest::new(Str::from_static("testValues"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testValues((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::builder::EnumCaseTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_EnumTest__testImplements() {
    php_rt::testing::run("PhpParser\\Builder\\EnumTest::testImplements", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::EnumTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::EnumTest::new(Str::from_static("testImplements"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testImplements();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::EnumTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_EnumTest__testSetScalarType() {
    php_rt::testing::run("PhpParser\\Builder\\EnumTest::testSetScalarType", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::EnumTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::EnumTest::new(Str::from_static("testSetScalarType"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testSetScalarType();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::EnumTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_EnumTest__testStatementOrder() {
    php_rt::testing::run("PhpParser\\Builder\\EnumTest::testStatementOrder", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::EnumTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::EnumTest::new(Str::from_static("testStatementOrder"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testStatementOrder();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::EnumTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_EnumTest__testDocComment() {
    php_rt::testing::run("PhpParser\\Builder\\EnumTest::testDocComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::EnumTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::EnumTest::new(Str::from_static("testDocComment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testDocComment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::EnumTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_EnumTest__testAddAttribute() {
    php_rt::testing::run("PhpParser\\Builder\\EnumTest::testAddAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::EnumTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::EnumTest::new(Str::from_static("testAddAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::EnumTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_EnumTest__testInvalidStmtError() {
    php_rt::testing::run("PhpParser\\Builder\\EnumTest::testInvalidStmtError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::EnumTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::EnumTest::new(Str::from_static("testInvalidStmtError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidStmtError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::EnumTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_EnumTest__testInvalidDocComment() {
    php_rt::testing::run("PhpParser\\Builder\\EnumTest::testInvalidDocComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::EnumTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::EnumTest::new(Str::from_static("testInvalidDocComment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidDocComment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::EnumTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_EnumTest__testEmptyName() {
    php_rt::testing::run("PhpParser\\Builder\\EnumTest::testEmptyName", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::EnumTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::EnumTest::new(Str::from_static("testEmptyName"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testEmptyName();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::EnumTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_EnumTest__testInvalidName() {
    php_rt::testing::run("PhpParser\\Builder\\EnumTest::testInvalidName", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::EnumTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::EnumTest::new(Str::from_static("testInvalidName"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidName();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::EnumTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_FunctionTest__testReturnByRef() {
    php_rt::testing::run("PhpParser\\Builder\\FunctionTest::testReturnByRef", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::FunctionTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::FunctionTest::new(Str::from_static("testReturnByRef"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testReturnByRef();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::FunctionTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_FunctionTest__testParams() {
    php_rt::testing::run("PhpParser\\Builder\\FunctionTest::testParams", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::FunctionTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::FunctionTest::new(Str::from_static("testParams"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testParams();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::FunctionTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_FunctionTest__testStmts() {
    php_rt::testing::run("PhpParser\\Builder\\FunctionTest::testStmts", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::FunctionTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::FunctionTest::new(Str::from_static("testStmts"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testStmts();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::FunctionTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_FunctionTest__testDocComment() {
    php_rt::testing::run("PhpParser\\Builder\\FunctionTest::testDocComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::FunctionTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::FunctionTest::new(Str::from_static("testDocComment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testDocComment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::FunctionTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_FunctionTest__testAddAttribute() {
    php_rt::testing::run("PhpParser\\Builder\\FunctionTest::testAddAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::FunctionTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::FunctionTest::new(Str::from_static("testAddAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::FunctionTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_FunctionTest__testReturnType() {
    php_rt::testing::run("PhpParser\\Builder\\FunctionTest::testReturnType", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::FunctionTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::FunctionTest::new(Str::from_static("testReturnType"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testReturnType();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::FunctionTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_FunctionTest__testInvalidNullableVoidType() {
    php_rt::testing::run("PhpParser\\Builder\\FunctionTest::testInvalidNullableVoidType", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::FunctionTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::FunctionTest::new(Str::from_static("testInvalidNullableVoidType"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidNullableVoidType();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::FunctionTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_FunctionTest__testInvalidParamError() {
    php_rt::testing::run("PhpParser\\Builder\\FunctionTest::testInvalidParamError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::FunctionTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::FunctionTest::new(Str::from_static("testInvalidParamError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidParamError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::FunctionTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_FunctionTest__testAddNonStmt() {
    php_rt::testing::run("PhpParser\\Builder\\FunctionTest::testAddNonStmt", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::FunctionTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::FunctionTest::new(Str::from_static("testAddNonStmt"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddNonStmt();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::FunctionTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_InterfaceTest__testEmpty() {
    php_rt::testing::run("PhpParser\\Builder\\InterfaceTest::testEmpty", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::InterfaceTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::InterfaceTest::new(Str::from_static("testEmpty"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testEmpty();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::InterfaceTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_InterfaceTest__testExtending() {
    php_rt::testing::run("PhpParser\\Builder\\InterfaceTest::testExtending", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::InterfaceTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::InterfaceTest::new(Str::from_static("testExtending"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testExtending();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::InterfaceTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_InterfaceTest__testAddMethod() {
    php_rt::testing::run("PhpParser\\Builder\\InterfaceTest::testAddMethod", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::InterfaceTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::InterfaceTest::new(Str::from_static("testAddMethod"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddMethod();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::InterfaceTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_InterfaceTest__testAddConst() {
    php_rt::testing::run("PhpParser\\Builder\\InterfaceTest::testAddConst", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::InterfaceTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::InterfaceTest::new(Str::from_static("testAddConst"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddConst();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::InterfaceTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_InterfaceTest__testOrder() {
    php_rt::testing::run("PhpParser\\Builder\\InterfaceTest::testOrder", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::InterfaceTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::InterfaceTest::new(Str::from_static("testOrder"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testOrder();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::InterfaceTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_InterfaceTest__testDocComment() {
    php_rt::testing::run("PhpParser\\Builder\\InterfaceTest::testDocComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::InterfaceTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::InterfaceTest::new(Str::from_static("testDocComment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testDocComment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::InterfaceTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_InterfaceTest__testAddAttribute() {
    php_rt::testing::run("PhpParser\\Builder\\InterfaceTest::testAddAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::InterfaceTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::InterfaceTest::new(Str::from_static("testAddAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::InterfaceTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_InterfaceTest__testInvalidStmtError() {
    php_rt::testing::run("PhpParser\\Builder\\InterfaceTest::testInvalidStmtError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::InterfaceTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::InterfaceTest::new(Str::from_static("testInvalidStmtError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidStmtError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::InterfaceTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_InterfaceTest__testFullFunctional() {
    php_rt::testing::run("PhpParser\\Builder\\InterfaceTest::testFullFunctional", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::InterfaceTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::InterfaceTest::new(Str::from_static("testFullFunctional"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testFullFunctional();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::InterfaceTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_MethodTest__testModifiers() {
    php_rt::testing::run("PhpParser\\Builder\\MethodTest::testModifiers", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::MethodTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::MethodTest::new(Str::from_static("testModifiers"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testModifiers();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::MethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_MethodTest__testReturnByRef() {
    php_rt::testing::run("PhpParser\\Builder\\MethodTest::testReturnByRef", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::MethodTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::MethodTest::new(Str::from_static("testReturnByRef"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testReturnByRef();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::MethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_MethodTest__testParams() {
    php_rt::testing::run("PhpParser\\Builder\\MethodTest::testParams", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::MethodTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::MethodTest::new(Str::from_static("testParams"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testParams();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::MethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_MethodTest__testStmts() {
    php_rt::testing::run("PhpParser\\Builder\\MethodTest::testStmts", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::MethodTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::MethodTest::new(Str::from_static("testStmts"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testStmts();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::MethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_MethodTest__testDocComment() {
    php_rt::testing::run("PhpParser\\Builder\\MethodTest::testDocComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::MethodTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::MethodTest::new(Str::from_static("testDocComment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testDocComment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::MethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_MethodTest__testAddAttribute() {
    php_rt::testing::run("PhpParser\\Builder\\MethodTest::testAddAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::MethodTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::MethodTest::new(Str::from_static("testAddAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::MethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_MethodTest__testReturnType() {
    php_rt::testing::run("PhpParser\\Builder\\MethodTest::testReturnType", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::MethodTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::MethodTest::new(Str::from_static("testReturnType"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testReturnType();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::MethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_MethodTest__testAddStmtToAbstractMethodError() {
    php_rt::testing::run("PhpParser\\Builder\\MethodTest::testAddStmtToAbstractMethodError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::MethodTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::MethodTest::new(Str::from_static("testAddStmtToAbstractMethodError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddStmtToAbstractMethodError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::MethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_MethodTest__testMakeMethodWithStmtsAbstractError() {
    php_rt::testing::run("PhpParser\\Builder\\MethodTest::testMakeMethodWithStmtsAbstractError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::MethodTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::MethodTest::new(Str::from_static("testMakeMethodWithStmtsAbstractError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testMakeMethodWithStmtsAbstractError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::MethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_MethodTest__testInvalidParamError() {
    php_rt::testing::run("PhpParser\\Builder\\MethodTest::testInvalidParamError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::MethodTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::MethodTest::new(Str::from_static("testInvalidParamError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidParamError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::MethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_NamespaceTest__testCreation() {
    php_rt::testing::run("PhpParser\\Builder\\NamespaceTest::testCreation", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::NamespaceTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::NamespaceTest::new(Str::from_static("testCreation"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testCreation();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::NamespaceTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ParamTest__testDefaultValues() {
    php_rt::testing::run("PhpParser\\Builder\\ParamTest::testDefaultValues", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ParamTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1834 = crate::php_parser::builder::ParamTest::provideTestDefaultValues()?; List::from_vec(vec![U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Opt_Null_PhpParser_Node_Expr_ConstFetch(__c1834.0), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Bool_PhpParser_Node_Expr_ConstFetch(__c1834.1), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Bool_PhpParser_Node_Expr_ConstFetch(__c1834.2), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Int_PhpParser_Node_Scalar_Int_(__c1834.3), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Float_PhpParser_Node_Scalar_Float_(__c1834.4), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Str_PhpParser_Node_Scalar_String_(__c1834.5), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Tup3_Int_Int_Int_PhpParser_Node_Expr_Array_(__c1834.6), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Shape_foo_Str_bar_Str_PhpParser_Node_Expr_Array_(__c1834.7), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_PhpParser_Node_Scalar_MagicConst_Dir_PhpParser_Node_Scalar_MagicConst_Dir(__c1834.8)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::builder::ParamTest::new(Str::from_static("testDefaultValues"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testDefaultValues((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::builder::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ParamTest__testTypes() {
    php_rt::testing::run("PhpParser\\Builder\\ParamTest::testTypes", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ParamTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::builder::ParamTest::provideTestTypes()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::builder::ParamTest::new(Str::from_static("testTypes"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testTypes((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::builder::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ParamTest__testVoidTypeError() {
    php_rt::testing::run("PhpParser\\Builder\\ParamTest::testVoidTypeError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ParamTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ParamTest::new(Str::from_static("testVoidTypeError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testVoidTypeError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ParamTest__testInvalidTypeError() {
    php_rt::testing::run("PhpParser\\Builder\\ParamTest::testInvalidTypeError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ParamTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ParamTest::new(Str::from_static("testInvalidTypeError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidTypeError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ParamTest__testByRef() {
    php_rt::testing::run("PhpParser\\Builder\\ParamTest::testByRef", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ParamTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ParamTest::new(Str::from_static("testByRef"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testByRef();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ParamTest__testVariadic() {
    php_rt::testing::run("PhpParser\\Builder\\ParamTest::testVariadic", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ParamTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ParamTest::new(Str::from_static("testVariadic"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testVariadic();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ParamTest__testMakePublic() {
    php_rt::testing::run("PhpParser\\Builder\\ParamTest::testMakePublic", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ParamTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ParamTest::new(Str::from_static("testMakePublic"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testMakePublic();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ParamTest__testMakeProtected() {
    php_rt::testing::run("PhpParser\\Builder\\ParamTest::testMakeProtected", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ParamTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ParamTest::new(Str::from_static("testMakeProtected"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testMakeProtected();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ParamTest__testMakePrivate() {
    php_rt::testing::run("PhpParser\\Builder\\ParamTest::testMakePrivate", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ParamTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ParamTest::new(Str::from_static("testMakePrivate"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testMakePrivate();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ParamTest__testMakeReadonly() {
    php_rt::testing::run("PhpParser\\Builder\\ParamTest::testMakeReadonly", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ParamTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ParamTest::new(Str::from_static("testMakeReadonly"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testMakeReadonly();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_ParamTest__testAddAttribute() {
    php_rt::testing::run("PhpParser\\Builder\\ParamTest::testAddAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::ParamTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::ParamTest::new(Str::from_static("testAddAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_PropertyTest__testModifiers() {
    php_rt::testing::run("PhpParser\\Builder\\PropertyTest::testModifiers", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::PropertyTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::PropertyTest::new(Str::from_static("testModifiers"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testModifiers();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::PropertyTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_PropertyTest__testAbstractWithoutHook() {
    php_rt::testing::run("PhpParser\\Builder\\PropertyTest::testAbstractWithoutHook", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::PropertyTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::PropertyTest::new(Str::from_static("testAbstractWithoutHook"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAbstractWithoutHook();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::PropertyTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_PropertyTest__testDocComment() {
    php_rt::testing::run("PhpParser\\Builder\\PropertyTest::testDocComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::PropertyTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::PropertyTest::new(Str::from_static("testDocComment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testDocComment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::PropertyTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_PropertyTest__testDefaultValues() {
    php_rt::testing::run("PhpParser\\Builder\\PropertyTest::testDefaultValues", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::PropertyTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1835 = crate::php_parser::builder::PropertyTest::provideTestDefaultValues()?; List::from_vec(vec![U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Opt_Null_PhpParser_Node_Expr_ConstFetch(__c1835.0), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Bool_PhpParser_Node_Expr_ConstFetch(__c1835.1), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Bool_PhpParser_Node_Expr_ConstFetch(__c1835.2), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Int_PhpParser_Node_Scalar_Int_(__c1835.3), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Float_PhpParser_Node_Scalar_Float_(__c1835.4), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Str_PhpParser_Node_Scalar_String_(__c1835.5), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Tup3_Int_Int_Int_PhpParser_Node_Expr_Array_(__c1835.6), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_Shape_foo_Str_bar_Str_PhpParser_Node_Expr_Array_(__c1835.7), U_Tup2_Bool_PhpParser_Node_Expr_ConstFetch_or_Tup2_Float_PhpParser_Nod_b8f47d04e3::Tup2_PhpParser_Node_Scalar_MagicConst_Dir_PhpParser_Node_Scalar_MagicConst_Dir(__c1835.8)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::builder::PropertyTest::new(Str::from_static("testDefaultValues"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testDefaultValues((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::builder::PropertyTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_PropertyTest__testAddAttribute() {
    php_rt::testing::run("PhpParser\\Builder\\PropertyTest::testAddAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::PropertyTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::PropertyTest::new(Str::from_static("testAddAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::PropertyTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_PropertyTest__testAddHook() {
    php_rt::testing::run("PhpParser\\Builder\\PropertyTest::testAddHook", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::PropertyTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::PropertyTest::new(Str::from_static("testAddHook"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddHook();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::PropertyTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitTest__testStmtAddition() {
    php_rt::testing::run("PhpParser\\Builder\\TraitTest::testStmtAddition", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitTest::new(Str::from_static("testStmtAddition"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testStmtAddition();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitTest__testInvalidStmtError() {
    php_rt::testing::run("PhpParser\\Builder\\TraitTest::testInvalidStmtError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitTest::new(Str::from_static("testInvalidStmtError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidStmtError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitTest__testGetMethods() {
    php_rt::testing::run("PhpParser\\Builder\\TraitTest::testGetMethods", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitTest::new(Str::from_static("testGetMethods"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetMethods();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitTest__testGetProperties() {
    php_rt::testing::run("PhpParser\\Builder\\TraitTest::testGetProperties", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitTest::new(Str::from_static("testGetProperties"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetProperties();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitTest__testAddAttribute() {
    php_rt::testing::run("PhpParser\\Builder\\TraitTest::testAddAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitTest::new(Str::from_static("testAddAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitUseAdaptationTest__testAsMake() {
    php_rt::testing::run("PhpParser\\Builder\\TraitUseAdaptationTest::testAsMake", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitUseAdaptationTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitUseAdaptationTest::new(Str::from_static("testAsMake"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAsMake();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitUseAdaptationTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitUseAdaptationTest__testInsteadof() {
    php_rt::testing::run("PhpParser\\Builder\\TraitUseAdaptationTest::testInsteadof", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitUseAdaptationTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitUseAdaptationTest::new(Str::from_static("testInsteadof"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInsteadof();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitUseAdaptationTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitUseAdaptationTest__testAsOnNotAlias() {
    php_rt::testing::run("PhpParser\\Builder\\TraitUseAdaptationTest::testAsOnNotAlias", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitUseAdaptationTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitUseAdaptationTest::new(Str::from_static("testAsOnNotAlias"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAsOnNotAlias();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitUseAdaptationTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitUseAdaptationTest__testInsteadofOnNotPrecedence() {
    php_rt::testing::run("PhpParser\\Builder\\TraitUseAdaptationTest::testInsteadofOnNotPrecedence", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitUseAdaptationTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitUseAdaptationTest::new(Str::from_static("testInsteadofOnNotPrecedence"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInsteadofOnNotPrecedence();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitUseAdaptationTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitUseAdaptationTest__testInsteadofWithoutTrait() {
    php_rt::testing::run("PhpParser\\Builder\\TraitUseAdaptationTest::testInsteadofWithoutTrait", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitUseAdaptationTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitUseAdaptationTest::new(Str::from_static("testInsteadofWithoutTrait"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInsteadofWithoutTrait();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitUseAdaptationTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitUseAdaptationTest__testMakeOnNotAlias() {
    php_rt::testing::run("PhpParser\\Builder\\TraitUseAdaptationTest::testMakeOnNotAlias", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitUseAdaptationTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitUseAdaptationTest::new(Str::from_static("testMakeOnNotAlias"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testMakeOnNotAlias();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitUseAdaptationTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitUseAdaptationTest__testMultipleMake() {
    php_rt::testing::run("PhpParser\\Builder\\TraitUseAdaptationTest::testMultipleMake", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitUseAdaptationTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitUseAdaptationTest::new(Str::from_static("testMultipleMake"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testMultipleMake();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitUseAdaptationTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitUseAdaptationTest__testUndefinedType() {
    php_rt::testing::run("PhpParser\\Builder\\TraitUseAdaptationTest::testUndefinedType", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitUseAdaptationTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitUseAdaptationTest::new(Str::from_static("testUndefinedType"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testUndefinedType();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitUseAdaptationTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitUseTest__testAnd() {
    php_rt::testing::run("PhpParser\\Builder\\TraitUseTest::testAnd", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitUseTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitUseTest::new(Str::from_static("testAnd"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAnd();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitUseTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitUseTest__testWith() {
    php_rt::testing::run("PhpParser\\Builder\\TraitUseTest::testWith", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitUseTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitUseTest::new(Str::from_static("testWith"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testWith();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitUseTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_TraitUseTest__testInvalidAdaptationNode() {
    php_rt::testing::run("PhpParser\\Builder\\TraitUseTest::testInvalidAdaptationNode", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::TraitUseTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::TraitUseTest::new(Str::from_static("testInvalidAdaptationNode"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidAdaptationNode();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::TraitUseTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Builder_UseTest__testCreation() {
    php_rt::testing::run("PhpParser\\Builder\\UseTest::testCreation", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::builder::UseTest::setUpBeforeClass()?;
        let __t = crate::php_parser::builder::UseTest::new(Str::from_static("testCreation"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testCreation();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::builder::UseTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testFactory() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testFactory", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::BuilderFactoryTest::provideTestFactory()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testFactory"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testFactory(cast::<Mixed>(__row.0.clone()), cast::<Mixed>(__row.1.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testFactoryClassConst() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testFactoryClassConst", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testFactoryClassConst"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testFactoryClassConst();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testAttribute() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testVal() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testVal", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testVal"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testVal();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testConcat() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testConcat", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testConcat"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testConcat();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testConcatOneError() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testConcatOneError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testConcatOneError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testConcatOneError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testConcatInvalidExpr() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testConcatInvalidExpr", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testConcatInvalidExpr"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testConcatInvalidExpr();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testArgs() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testArgs", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testArgs"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testArgs();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testNamedArgs() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testNamedArgs", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testNamedArgs"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNamedArgs();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testCalls() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testCalls", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testCalls"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testCalls();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testConstFetches() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testConstFetches", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testConstFetches"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testConstFetches();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testVar() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testVar", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testVar"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testVar();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testPropertyFetch() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testPropertyFetch", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testPropertyFetch"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testPropertyFetch();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testInvalidIdentifier() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testInvalidIdentifier", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testInvalidIdentifier"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidIdentifier();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testInvalidIdentifierOrExpr() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testInvalidIdentifierOrExpr", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testInvalidIdentifierOrExpr"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidIdentifierOrExpr();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testInvalidNameOrExpr() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testInvalidNameOrExpr", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testInvalidNameOrExpr"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidNameOrExpr();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testInvalidVar() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testInvalidVar", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testInvalidVar"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidVar();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderFactoryTest__testIntegration() {
    php_rt::testing::run("PhpParser\\BuilderFactoryTest::testIntegration", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderFactoryTest::new(Str::from_static("testIntegration"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testIntegration();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeNode() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeNode", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeNode"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeNode();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeStmt() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeStmt", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeStmt"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeStmt();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeStmtInvalidType() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeStmtInvalidType", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeStmtInvalidType"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeStmtInvalidType();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeIdentifier() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeIdentifier", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeIdentifier"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeIdentifier();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeIdentifierOrExpr() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeIdentifierOrExpr", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeIdentifierOrExpr"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeIdentifierOrExpr();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeName() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeName", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeName"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeName();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeNameInvalidType() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeNameInvalidType", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeNameInvalidType"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeNameInvalidType();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeNameOrExpr() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeNameOrExpr", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeNameOrExpr"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeNameOrExpr();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeNameOrExpInvalidType() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeNameOrExpInvalidType", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeNameOrExpInvalidType"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeNameOrExpInvalidType();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeType() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeType", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeType"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeType();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeTypeNullableVoid() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeTypeNullableVoid", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeTypeNullableVoid"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeTypeNullableVoid();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeTypeNullableMixed() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeTypeNullableMixed", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeTypeNullableMixed"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeTypeNullableMixed();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeTypeNullableNever() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeTypeNullableNever", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeTypeNullableNever"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeTypeNullableNever();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeValue() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeValue", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeValue"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeValue();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeDocComment() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeDocComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeDocComment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeDocComment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeAttribute() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_BuilderHelpersTest__testNormalizeValueEnum() {
    php_rt::testing::run("PhpParser\\BuilderHelpersTest::testNormalizeValueEnum", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::BuilderHelpersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::BuilderHelpersTest::new(Str::from_static("testNormalizeValueEnum"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNormalizeValueEnum();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::BuilderHelpersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_CodeParsingTest__testParse() {
    php_rt::testing::run("PhpParser\\CodeParsingTest::testParse", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::CodeParsingTest::setUpBeforeClass()?;
        for (__key, __row) in cast::<Map<ArrayKey, Mixed>>(crate::php_parser::CodeParsingTest::provideTestParse()?).into_iter() {
            let __t = crate::php_parser::CodeParsingTest::new(Str::from_static("testParse"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testParse((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(2).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(3).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::CodeParsingTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_CommentTest__testGetters() {
    php_rt::testing::run("PhpParser\\CommentTest::testGetters", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::CommentTest::setUpBeforeClass()?;
        let __t = crate::php_parser::CommentTest::new(Str::from_static("testGetters"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetters();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::CommentTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_CommentTest__testReformatting() {
    php_rt::testing::run("PhpParser\\CommentTest::testReformatting", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::CommentTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1836 = crate::php_parser::CommentTest::provideTestReformatting()?; List::from_vec(vec![__c1836.0, __c1836.1, __c1836.2, __c1836.3, __c1836.4, __c1836.5, __c1836.6, __c1836.7, __c1836.8, __c1836.9]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::CommentTest::new(Str::from_static("testReformatting"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testReformatting(cast::<Mixed>(__row.0.clone()), cast::<Mixed>(__row.1.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::CommentTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_CompatibilityTest__testAliases1() {
    php_rt::testing::run("PhpParser\\CompatibilityTest::testAliases1", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::CompatibilityTest::setUpBeforeClass()?;
        let __t = crate::php_parser::CompatibilityTest::new(Str::from_static("testAliases1"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAliases1();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::CompatibilityTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_CompatibilityTest__testAliases2() {
    php_rt::testing::run("PhpParser\\CompatibilityTest::testAliases2", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::CompatibilityTest::setUpBeforeClass()?;
        let __t = crate::php_parser::CompatibilityTest::new(Str::from_static("testAliases2"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAliases2();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::CompatibilityTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ConstExprEvaluatorTest__testEvaluate() {
    php_rt::testing::run("PhpParser\\ConstExprEvaluatorTest::testEvaluate", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ConstExprEvaluatorTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::ConstExprEvaluatorTest::provideTestEvaluate()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::ConstExprEvaluatorTest::new(Str::from_static("testEvaluate"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testEvaluate((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::ConstExprEvaluatorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ConstExprEvaluatorTest__testEvaluateFails() {
    php_rt::testing::run("PhpParser\\ConstExprEvaluatorTest::testEvaluateFails", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ConstExprEvaluatorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::ConstExprEvaluatorTest::new(Str::from_static("testEvaluateFails"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testEvaluateFails();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::ConstExprEvaluatorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ConstExprEvaluatorTest__testEvaluateFallbackMagicConst() {
    php_rt::testing::run("PhpParser\\ConstExprEvaluatorTest::testEvaluateFallbackMagicConst", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ConstExprEvaluatorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::ConstExprEvaluatorTest::new(Str::from_static("testEvaluateFallbackMagicConst"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testEvaluateFallbackMagicConst();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::ConstExprEvaluatorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ConstExprEvaluatorTest__testEvaluateFallbackPipeOperator() {
    php_rt::testing::run("PhpParser\\ConstExprEvaluatorTest::testEvaluateFallbackPipeOperator", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ConstExprEvaluatorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::ConstExprEvaluatorTest::new(Str::from_static("testEvaluateFallbackPipeOperator"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testEvaluateFallbackPipeOperator();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::ConstExprEvaluatorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ConstExprEvaluatorTest__testEvaluateSilently() {
    php_rt::testing::run("PhpParser\\ConstExprEvaluatorTest::testEvaluateSilently", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ConstExprEvaluatorTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1837 = crate::php_parser::ConstExprEvaluatorTest::provideTestEvaluateSilently()?; List::from_vec(vec![U_Tup3_PhpParser_Node_Expr_BinaryOp_Mod_Str_Str_or_Tup3_PhpParser_Node_6dd3c5634c::Tup3_PhpParser_Node_Expr_BinaryOp_Mod_Str_Str(__c1837.0), U_Tup3_PhpParser_Node_Expr_BinaryOp_Mod_Str_Str_or_Tup3_PhpParser_Node_6dd3c5634c::Tup3_PhpParser_Node_Expr_BinaryOp_Plus_Str_Str(__c1837.1)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::ConstExprEvaluatorTest::new(Str::from_static("testEvaluateSilently"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testEvaluateSilently((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(2).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::ConstExprEvaluatorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ErrorHandler_CollectingTest__testHandleError() {
    php_rt::testing::run("PhpParser\\ErrorHandler\\CollectingTest::testHandleError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::error_handler::CollectingTest::setUpBeforeClass()?;
        let __t = crate::php_parser::error_handler::CollectingTest::new(Str::from_static("testHandleError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testHandleError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::error_handler::CollectingTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ErrorHandler_ThrowingTest__testHandleError() {
    php_rt::testing::run("PhpParser\\ErrorHandler\\ThrowingTest::testHandleError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::error_handler::ThrowingTest::setUpBeforeClass()?;
        let __t = crate::php_parser::error_handler::ThrowingTest::new(Str::from_static("testHandleError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testHandleError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::error_handler::ThrowingTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ErrorTest__testConstruct() {
    php_rt::testing::run("PhpParser\\ErrorTest::testConstruct", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ErrorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::ErrorTest::new(Str::from_static("testConstruct"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testConstruct();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::ErrorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ErrorTest__testSetMessageAndLine() {
    php_rt::testing::run("PhpParser\\ErrorTest::testSetMessageAndLine", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ErrorTest::setUpBeforeClass()?;
        let __dep0 = { let __d = crate::php_parser::ErrorTest::new(Str::from_static("testSetMessageAndLine"))?; __d.runSetUp()?; let __r = __d.testConstruct()?; __d.runTearDown()?; __r };
        let __t = crate::php_parser::ErrorTest::new(Str::from_static("testSetMessageAndLine"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testSetMessageAndLine(__dep0.clone());
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::ErrorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ErrorTest__testUnknownLine() {
    php_rt::testing::run("PhpParser\\ErrorTest::testUnknownLine", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ErrorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::ErrorTest::new(Str::from_static("testUnknownLine"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testUnknownLine();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::ErrorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ErrorTest__testColumnInfo() {
    php_rt::testing::run("PhpParser\\ErrorTest::testColumnInfo", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ErrorTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1838 = crate::php_parser::ErrorTest::provideTestColumnInfo()?; List::from_vec(vec![__c1838.0, __c1838.1, __c1838.2, __c1838.3, __c1838.4, __c1838.5, __c1838.6, __c1838.7, __c1838.8, __c1838.9, __c1838.10, __c1838.11]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::ErrorTest::new(Str::from_static("testColumnInfo"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testColumnInfo(cast::<Mixed>(__row.0.clone()), cast::<Mixed>(__row.1.clone()), cast::<Mixed>(__row.2.clone()), cast::<Mixed>(__row.3.clone()), cast::<Mixed>(__row.4.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::ErrorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ErrorTest__testNoColumnInfo() {
    php_rt::testing::run("PhpParser\\ErrorTest::testNoColumnInfo", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ErrorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::ErrorTest::new(Str::from_static("testNoColumnInfo"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNoColumnInfo();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::ErrorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ErrorTest__testInvalidPosInfo() {
    php_rt::testing::run("PhpParser\\ErrorTest::testInvalidPosInfo", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ErrorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::ErrorTest::new(Str::from_static("testInvalidPosInfo"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidPosInfo();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::ErrorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Internal_DifferTest__testDiff() {
    php_rt::testing::run("PhpParser\\Internal\\DifferTest::testDiff", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::internal::DifferTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1839 = crate::php_parser::internal::DifferTest::provideTestDiff()?; List::from_vec(vec![__c1839.0, __c1839.1, __c1839.2, __c1839.3, __c1839.4, __c1839.5, __c1839.6]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::internal::DifferTest::new(Str::from_static("testDiff"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testDiff(cast::<Mixed>(__row.0.clone()), cast::<Mixed>(__row.1.clone()), cast::<Mixed>(__row.2.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::internal::DifferTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Internal_DifferTest__testDiffWithReplacements() {
    php_rt::testing::run("PhpParser\\Internal\\DifferTest::testDiffWithReplacements", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::internal::DifferTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1840 = crate::php_parser::internal::DifferTest::provideTestDiffWithReplacements()?; List::from_vec(vec![__c1840.0, __c1840.1, __c1840.2, __c1840.3]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::internal::DifferTest::new(Str::from_static("testDiffWithReplacements"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testDiffWithReplacements(cast::<Mixed>(__row.0.clone()), cast::<Mixed>(__row.1.clone()), cast::<Mixed>(__row.2.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::internal::DifferTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Internal_DifferTest__testNonContiguousIndices() {
    php_rt::testing::run("PhpParser\\Internal\\DifferTest::testNonContiguousIndices", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::internal::DifferTest::setUpBeforeClass()?;
        let __t = crate::php_parser::internal::DifferTest::new(Str::from_static("testNonContiguousIndices"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNonContiguousIndices();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::internal::DifferTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_JsonDecoderTest__testRoundTrip() {
    php_rt::testing::run("PhpParser\\JsonDecoderTest::testRoundTrip", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::JsonDecoderTest::setUpBeforeClass()?;
        let __t = crate::php_parser::JsonDecoderTest::new(Str::from_static("testRoundTrip"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testRoundTrip();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::JsonDecoderTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_JsonDecoderTest__testDecodingError() {
    php_rt::testing::run("PhpParser\\JsonDecoderTest::testDecodingError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::JsonDecoderTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1841 = crate::php_parser::JsonDecoderTest::provideTestDecodingError()?; List::from_vec(vec![__c1841.0, __c1841.1, __c1841.2, __c1841.3, __c1841.4]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::JsonDecoderTest::new(Str::from_static("testDecodingError"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testDecodingError(cast::<Mixed>(__row.0.clone()), cast::<Mixed>(__row.1.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::JsonDecoderTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testReplaceKeywords() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testReplaceKeywords", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::lexer::EmulativeTest::provideTestReplaceKeywords()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testReplaceKeywords"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testReplaceKeywords(__row.0.clone(), __row.1.clone());
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testReplaceKeywordsUppercase() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testReplaceKeywordsUppercase", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::lexer::EmulativeTest::provideTestReplaceKeywords()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testReplaceKeywordsUppercase"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testReplaceKeywordsUppercase(__row.0.clone(), __row.1.clone());
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testNoReplaceKeywordsAfterObjectOperator() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testNoReplaceKeywordsAfterObjectOperator", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::lexer::EmulativeTest::provideTestReplaceKeywords()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testNoReplaceKeywordsAfterObjectOperator"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testNoReplaceKeywordsAfterObjectOperator(__row.0.clone());
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testNoReplaceKeywordsAfterObjectOperatorWithSpaces() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testNoReplaceKeywordsAfterObjectOperatorWithSpaces", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::lexer::EmulativeTest::provideTestReplaceKeywords()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testNoReplaceKeywordsAfterObjectOperatorWithSpaces"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testNoReplaceKeywordsAfterObjectOperatorWithSpaces(__row.0.clone());
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testNoReplaceKeywordsAfterObjectOperatorWithComment() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testNoReplaceKeywordsAfterObjectOperatorWithComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testNoReplaceKeywordsAfterObjectOperatorWithComment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNoReplaceKeywordsAfterObjectOperatorWithComment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testNoReplaceKeywordsAfterNullsafeObjectOperator() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testNoReplaceKeywordsAfterNullsafeObjectOperator", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::lexer::EmulativeTest::provideTestReplaceKeywords()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testNoReplaceKeywordsAfterNullsafeObjectOperator"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testNoReplaceKeywordsAfterNullsafeObjectOperator(__row.0.clone());
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testLexNewFeatures() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testLexNewFeatures", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::lexer::EmulativeTest::provideTestLexNewFeatures()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testLexNewFeatures"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testLexNewFeatures((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => cast::<Str>(__a), None => <Str>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a), None => <Map<ArrayKey, Mixed>>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testLeaveStuffAloneInStrings() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testLeaveStuffAloneInStrings", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::lexer::EmulativeTest::provideTestLexNewFeatures()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testLeaveStuffAloneInStrings"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testLeaveStuffAloneInStrings((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => cast::<Str>(__a), None => <Str>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testErrorAfterEmulation() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testErrorAfterEmulation", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::lexer::EmulativeTest::provideTestLexNewFeatures()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testErrorAfterEmulation"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testErrorAfterEmulation((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testTargetVersion() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testTargetVersion", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::lexer::EmulativeTest::provideTestTargetVersion()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testTargetVersion"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testTargetVersion((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => cast::<Str>(__a), None => <Str>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => cast::<Str>(__a), None => <Str>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(2).cloned() { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a), None => <Map<ArrayKey, Mixed>>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testError() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1842 = crate::php_parser::lexer::EmulativeTest::provideTestError()?; List::from_vec(vec![U_Tup2_Str_Tup1_Str_or_Tup2_Str_Tup3_Str_Str_Str::Tup2_Str_Tup1_Str(__c1842.0), U_Tup2_Str_Tup1_Str_or_Tup2_Str_Tup3_Str_Str_Str::Tup2_Str_Tup1_Str(__c1842.1), U_Tup2_Str_Tup1_Str_or_Tup2_Str_Tup3_Str_Str_Str::Tup2_Str_Tup1_Str(__c1842.2), U_Tup2_Str_Tup1_Str_or_Tup2_Str_Tup3_Str_Str_Str::Tup2_Str_Tup1_Str(__c1842.3), U_Tup2_Str_Tup1_Str_or_Tup2_Str_Tup3_Str_Str_Str::Tup2_Str_Tup1_Str(__c1842.4), U_Tup2_Str_Tup1_Str_or_Tup2_Str_Tup3_Str_Str_Str::Tup2_Str_Tup3_Str_Str_Str(__c1842.5)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testError"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testError((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testDefaultErrorHandler() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testDefaultErrorHandler", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testDefaultErrorHandler"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testDefaultErrorHandler();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testLex() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testLex", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1843 = crate::php_parser::lexer::EmulativeTest::provideTestLex()?; List::from_vec(vec![__c1843.0, __c1843.1]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testLex"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testLex(cast::<Mixed>(__row.0.clone()), { let __c1844 = __row.1.clone(); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push({ let __c1845 = __c1844.0; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1845.0)); __m.push(cast::<Mixed>(__c1845.1)); Mixed::Arr(__m) }); __m.push({ let __c1846 = __c1844.1; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1846.0)); __m.push(cast::<Mixed>(__c1846.1)); Mixed::Arr(__m) }); __m.push({ let __c1847 = __c1844.2; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1847.0)); __m.push(cast::<Mixed>(__c1847.1)); Mixed::Arr(__m) }); __m.push({ let __c1848 = __c1844.3; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1848.0)); __m.push(cast::<Mixed>(__c1848.1)); Mixed::Arr(__m) }); __m.push({ let __c1849 = __c1844.4; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1849.0)); __m.push(cast::<Mixed>(__c1849.1)); Mixed::Arr(__m) }); Mixed::Arr(__m) });
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Lexer_EmulativeTest__testGetTokens() {
    php_rt::testing::run("PhpParser\\Lexer\\EmulativeTest::testGetTokens", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::lexer::EmulativeTest::setUpBeforeClass()?;
        let __t = crate::php_parser::lexer::EmulativeTest::new(Str::from_static("testGetTokens"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetTokens();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::lexer::EmulativeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_LexerTest__testError() {
    php_rt::testing::run("PhpParser\\LexerTest::testError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::LexerTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1850 = crate::php_parser::LexerTest::provideTestError()?; List::from_vec(vec![U_Tup2_Str_Tup1_Str_or_Tup2_Str_Tup3_Str_Str_Str::Tup2_Str_Tup1_Str(__c1850.0), U_Tup2_Str_Tup1_Str_or_Tup2_Str_Tup3_Str_Str_Str::Tup2_Str_Tup1_Str(__c1850.1), U_Tup2_Str_Tup1_Str_or_Tup2_Str_Tup3_Str_Str_Str::Tup2_Str_Tup1_Str(__c1850.2), U_Tup2_Str_Tup1_Str_or_Tup2_Str_Tup3_Str_Str_Str::Tup2_Str_Tup1_Str(__c1850.3), U_Tup2_Str_Tup1_Str_or_Tup2_Str_Tup3_Str_Str_Str::Tup2_Str_Tup1_Str(__c1850.4), U_Tup2_Str_Tup1_Str_or_Tup2_Str_Tup3_Str_Str_Str::Tup2_Str_Tup3_Str_Str_Str(__c1850.5)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::LexerTest::new(Str::from_static("testError"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testError((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::LexerTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_LexerTest__testDefaultErrorHandler() {
    php_rt::testing::run("PhpParser\\LexerTest::testDefaultErrorHandler", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::LexerTest::setUpBeforeClass()?;
        let __t = crate::php_parser::LexerTest::new(Str::from_static("testDefaultErrorHandler"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testDefaultErrorHandler();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::LexerTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_LexerTest__testLex() {
    php_rt::testing::run("PhpParser\\LexerTest::testLex", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::LexerTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1851 = crate::php_parser::LexerTest::provideTestLex()?; List::from_vec(vec![__c1851.0, __c1851.1]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::LexerTest::new(Str::from_static("testLex"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testLex(cast::<Mixed>(__row.0.clone()), { let __c1852 = __row.1.clone(); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push({ let __c1853 = __c1852.0; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1853.0)); __m.push(cast::<Mixed>(__c1853.1)); Mixed::Arr(__m) }); __m.push({ let __c1854 = __c1852.1; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1854.0)); __m.push(cast::<Mixed>(__c1854.1)); Mixed::Arr(__m) }); __m.push({ let __c1855 = __c1852.2; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1855.0)); __m.push(cast::<Mixed>(__c1855.1)); Mixed::Arr(__m) }); __m.push({ let __c1856 = __c1852.3; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1856.0)); __m.push(cast::<Mixed>(__c1856.1)); Mixed::Arr(__m) }); __m.push({ let __c1857 = __c1852.4; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1857.0)); __m.push(cast::<Mixed>(__c1857.1)); Mixed::Arr(__m) }); Mixed::Arr(__m) });
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::LexerTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_LexerTest__testGetTokens() {
    php_rt::testing::run("PhpParser\\LexerTest::testGetTokens", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::LexerTest::setUpBeforeClass()?;
        let __t = crate::php_parser::LexerTest::new(Str::from_static("testGetTokens"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetTokens();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::LexerTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ModifiersTest__testToString() {
    php_rt::testing::run("PhpParser\\ModifiersTest::testToString", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ModifiersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::ModifiersTest::new(Str::from_static("testToString"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testToString();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::ModifiersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ModifiersTest__testToStringInvalid() {
    php_rt::testing::run("PhpParser\\ModifiersTest::testToStringInvalid", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ModifiersTest::setUpBeforeClass()?;
        let __t = crate::php_parser::ModifiersTest::new(Str::from_static("testToStringInvalid"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testToStringInvalid();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::ModifiersTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NameContextTest__testGetPossibleNames() {
    php_rt::testing::run("PhpParser\\NameContextTest::testGetPossibleNames", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NameContextTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::NameContextTest::provideTestGetPossibleNames()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::NameContextTest::new(Str::from_static("testGetPossibleNames"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testGetPossibleNames((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(2).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::NameContextTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Expr_CallableLikeTest__testIsFirstClassCallable() {
    php_rt::testing::run("PhpParser\\Node\\Expr\\CallableLikeTest::testIsFirstClassCallable", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::expr::CallableLikeTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1858 = crate::php_parser::node::expr::CallableLikeTest::provideTestIsFirstClassCallable()?; List::from_vec(vec![U_Tup2_PhpParser_Node_Expr_FuncCall_Bool_or_Tup2_PhpParser_Node_Expr_M_b9c5c13063::Tup2_PhpParser_Node_Expr_FuncCall_Bool(__c1858.0), U_Tup2_PhpParser_Node_Expr_FuncCall_Bool_or_Tup2_PhpParser_Node_Expr_M_b9c5c13063::Tup2_PhpParser_Node_Expr_FuncCall_Bool(__c1858.1), U_Tup2_PhpParser_Node_Expr_FuncCall_Bool_or_Tup2_PhpParser_Node_Expr_M_b9c5c13063::Tup2_PhpParser_Node_Expr_MethodCall_Bool(__c1858.2), U_Tup2_PhpParser_Node_Expr_FuncCall_Bool_or_Tup2_PhpParser_Node_Expr_M_b9c5c13063::Tup2_PhpParser_Node_Expr_MethodCall_Bool(__c1858.3), U_Tup2_PhpParser_Node_Expr_FuncCall_Bool_or_Tup2_PhpParser_Node_Expr_M_b9c5c13063::Tup2_PhpParser_Node_Expr_StaticCall_Bool(__c1858.4), U_Tup2_PhpParser_Node_Expr_FuncCall_Bool_or_Tup2_PhpParser_Node_Expr_M_b9c5c13063::Tup2_PhpParser_Node_Expr_StaticCall_Bool(__c1858.5), U_Tup2_PhpParser_Node_Expr_FuncCall_Bool_or_Tup2_PhpParser_Node_Expr_M_b9c5c13063::Tup2_PhpParser_Node_Expr_New__Bool(__c1858.6), U_Tup2_PhpParser_Node_Expr_FuncCall_Bool_or_Tup2_PhpParser_Node_Expr_M_b9c5c13063::Tup2_PhpParser_Node_Expr_NullsafeMethodCall_Bool(__c1858.7), U_Tup2_PhpParser_Node_Expr_FuncCall_Bool_or_Tup2_PhpParser_Node_Expr_M_b9c5c13063::Tup2_PhpParser_Node_Expr_New__Bool(__c1858.8), U_Tup2_PhpParser_Node_Expr_FuncCall_Bool_or_Tup2_PhpParser_Node_Expr_M_b9c5c13063::Tup2_PhpParser_Node_Expr_NullsafeMethodCall_Bool(__c1858.9)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::expr::CallableLikeTest::new(Str::from_static("testIsFirstClassCallable"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testIsFirstClassCallable((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => cast::<crate::php_parser::node::expr::CallLike>(__a), None => unreachable!("no default for crate::php_parser::node::expr::CallLike") }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => cast::<bool>(__a), None => <bool>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::expr::CallableLikeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Expr_CallableLikeTest__testGetArg() {
    php_rt::testing::run("PhpParser\\Node\\Expr\\CallableLikeTest::testGetArg", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::expr::CallableLikeTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::node::expr::CallableLikeTest::provideTestGetArg()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::expr::CallableLikeTest::new(Str::from_static("testGetArg"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testGetArg((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => cast::<crate::php_parser::node::expr::CallLike>(__a), None => unreachable!("no default for crate::php_parser::node::expr::CallLike") }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a.to_option().map(|__m| cast::<crate::php_parser::node::Arg>(__m)), None => <Option<crate::php_parser::node::Arg>>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::expr::CallableLikeTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_IdentifierTest__testConstructorThrows() {
    php_rt::testing::run("PhpParser\\Node\\IdentifierTest::testConstructorThrows", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::IdentifierTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::IdentifierTest::new(Str::from_static("testConstructorThrows"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testConstructorThrows();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::IdentifierTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_IdentifierTest__testToString() {
    php_rt::testing::run("PhpParser\\Node\\IdentifierTest::testToString", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::IdentifierTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::IdentifierTest::new(Str::from_static("testToString"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testToString();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::IdentifierTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_IdentifierTest__testIsSpecialClassName() {
    php_rt::testing::run("PhpParser\\Node\\IdentifierTest::testIsSpecialClassName", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::IdentifierTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1859 = crate::php_parser::node::IdentifierTest::provideTestIsSpecialClassName()?; List::from_vec(vec![__c1859.0, __c1859.1, __c1859.2, __c1859.3]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::IdentifierTest::new(Str::from_static("testIsSpecialClassName"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testIsSpecialClassName(cast::<Mixed>(__row.0.clone()), cast::<Mixed>(__row.1.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::IdentifierTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testConstruct() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testConstruct", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testConstruct"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testConstruct();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testGet() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testGet", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testGet"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGet();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testToString() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testToString", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testToString"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testToString();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testSlice() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testSlice", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testSlice"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testSlice();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testSliceOffsetTooLarge() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testSliceOffsetTooLarge", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testSliceOffsetTooLarge"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testSliceOffsetTooLarge();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testSliceOffsetTooSmall() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testSliceOffsetTooSmall", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testSliceOffsetTooSmall"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testSliceOffsetTooSmall();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testSliceLengthTooLarge() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testSliceLengthTooLarge", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testSliceLengthTooLarge"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testSliceLengthTooLarge();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testSliceLengthTooSmall() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testSliceLengthTooSmall", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testSliceLengthTooSmall"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testSliceLengthTooSmall();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testSliceLengthTooLargeWithOffset() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testSliceLengthTooLargeWithOffset", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testSliceLengthTooLargeWithOffset"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testSliceLengthTooLargeWithOffset();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testConcat() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testConcat", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testConcat"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testConcat();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testNameTypes() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testNameTypes", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testNameTypes"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNameTypes();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testInvalidArg() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testInvalidArg", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testInvalidArg"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidArg();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testInvalidEmptyString() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testInvalidEmptyString", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testInvalidEmptyString"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidEmptyString();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testInvalidEmptyArray() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testInvalidEmptyArray", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::NameTest::new(Str::from_static("testInvalidEmptyArray"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidEmptyArray();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_NameTest__testIsSpecialClassName() {
    php_rt::testing::run("PhpParser\\Node\\NameTest::testIsSpecialClassName", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::NameTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1860 = crate::php_parser::node::NameTest::provideTestIsSpecialClassName()?; List::from_vec(vec![__c1860.0, __c1860.1, __c1860.2, __c1860.3, __c1860.4]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::NameTest::new(Str::from_static("testIsSpecialClassName"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testIsSpecialClassName(cast::<Mixed>(__row.0.clone()), cast::<Mixed>(__row.1.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::NameTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_ParamTest__testNoModifiers() {
    php_rt::testing::run("PhpParser\\Node\\ParamTest::testNoModifiers", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::ParamTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::ParamTest::new(Str::from_static("testNoModifiers"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNoModifiers();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_ParamTest__testModifiers() {
    php_rt::testing::run("PhpParser\\Node\\ParamTest::testModifiers", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::ParamTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1861 = crate::php_parser::node::ParamTest::provideModifiers()?; List::from_vec(vec![__c1861.0, __c1861.1, __c1861.2, __c1861.3, __c1861.4]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::ParamTest::new(Str::from_static("testModifiers"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testModifiers(__row.0.clone());
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_ParamTest__testSetVisibility() {
    php_rt::testing::run("PhpParser\\Node\\ParamTest::testSetVisibility", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::ParamTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::ParamTest::new(Str::from_static("testSetVisibility"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testSetVisibility();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_ParamTest__testPromotedPropertyWithoutVisibilityModifier() {
    php_rt::testing::run("PhpParser\\Node\\ParamTest::testPromotedPropertyWithoutVisibilityModifier", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::ParamTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::ParamTest::new(Str::from_static("testPromotedPropertyWithoutVisibilityModifier"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testPromotedPropertyWithoutVisibilityModifier();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_ParamTest__testNonPromotedPropertyIsNotPublic() {
    php_rt::testing::run("PhpParser\\Node\\ParamTest::testNonPromotedPropertyIsNotPublic", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::ParamTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::ParamTest::new(Str::from_static("testNonPromotedPropertyIsNotPublic"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNonPromotedPropertyIsNotPublic();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::ParamTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_PropertyHookTest__testModifiers() {
    php_rt::testing::run("PhpParser\\Node\\PropertyHookTest::testModifiers", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::PropertyHookTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1862 = crate::php_parser::node::PropertyHookTest::provideModifiers()?; List::from_vec(vec![__c1862.0]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::PropertyHookTest::new(Str::from_static("testModifiers"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testModifiers(cast::<Mixed>(__row.0.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::PropertyHookTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_PropertyHookTest__testNoModifiers() {
    php_rt::testing::run("PhpParser\\Node\\PropertyHookTest::testNoModifiers", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::PropertyHookTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::PropertyHookTest::new(Str::from_static("testNoModifiers"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNoModifiers();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::PropertyHookTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_PropertyHookTest__testGetStmts() {
    php_rt::testing::run("PhpParser\\Node\\PropertyHookTest::testGetStmts", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::PropertyHookTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::PropertyHookTest::new(Str::from_static("testGetStmts"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetStmts();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::PropertyHookTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_PropertyHookTest__testGetStmtsSetHookFromParser() {
    php_rt::testing::run("PhpParser\\Node\\PropertyHookTest::testGetStmtsSetHookFromParser", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::PropertyHookTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::PropertyHookTest::new(Str::from_static("testGetStmtsSetHookFromParser"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetStmtsSetHookFromParser();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::PropertyHookTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_PropertyHookTest__testGetStmtsUnknownHook() {
    php_rt::testing::run("PhpParser\\Node\\PropertyHookTest::testGetStmtsUnknownHook", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::PropertyHookTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::PropertyHookTest::new(Str::from_static("testGetStmtsUnknownHook"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetStmtsUnknownHook();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::PropertyHookTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_PropertyHookTest__testGetStmtsSetHookWithoutPropertyName() {
    php_rt::testing::run("PhpParser\\Node\\PropertyHookTest::testGetStmtsSetHookWithoutPropertyName", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::PropertyHookTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::PropertyHookTest::new(Str::from_static("testGetStmtsSetHookWithoutPropertyName"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetStmtsSetHookWithoutPropertyName();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::PropertyHookTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Scalar_DNumberTest__testRawValue() {
    php_rt::testing::run("PhpParser\\Node\\Scalar\\DNumberTest::testRawValue", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::scalar::DNumberTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::scalar::DNumberTest::new(Str::from_static("testRawValue"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testRawValue();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::scalar::DNumberTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Scalar_MagicConstTest__testGetName() {
    php_rt::testing::run("PhpParser\\Node\\Scalar\\MagicConstTest::testGetName", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::scalar::MagicConstTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1863 = crate::php_parser::node::scalar::MagicConstTest::provideTestGetName()?; List::from_vec(vec![U_Tup2_PhpParser_Node_Scalar_MagicConst_Class__Str_or_Tup2_PhpParser_N_dc768bc889::Tup2_PhpParser_Node_Scalar_MagicConst_Class__Str(__c1863.0), U_Tup2_PhpParser_Node_Scalar_MagicConst_Class__Str_or_Tup2_PhpParser_N_dc768bc889::Tup2_PhpParser_Node_Scalar_MagicConst_Dir_Str(__c1863.1), U_Tup2_PhpParser_Node_Scalar_MagicConst_Class__Str_or_Tup2_PhpParser_N_dc768bc889::Tup2_PhpParser_Node_Scalar_MagicConst_File_Str(__c1863.2), U_Tup2_PhpParser_Node_Scalar_MagicConst_Class__Str_or_Tup2_PhpParser_N_dc768bc889::Tup2_PhpParser_Node_Scalar_MagicConst_Function__Str(__c1863.3), U_Tup2_PhpParser_Node_Scalar_MagicConst_Class__Str_or_Tup2_PhpParser_N_dc768bc889::Tup2_PhpParser_Node_Scalar_MagicConst_Line_Str(__c1863.4), U_Tup2_PhpParser_Node_Scalar_MagicConst_Class__Str_or_Tup2_PhpParser_N_dc768bc889::Tup2_PhpParser_Node_Scalar_MagicConst_Method_Str(__c1863.5), U_Tup2_PhpParser_Node_Scalar_MagicConst_Class__Str_or_Tup2_PhpParser_N_dc768bc889::Tup2_PhpParser_Node_Scalar_MagicConst_Namespace__Str(__c1863.6), U_Tup2_PhpParser_Node_Scalar_MagicConst_Class__Str_or_Tup2_PhpParser_N_dc768bc889::Tup2_PhpParser_Node_Scalar_MagicConst_Trait__Str(__c1863.7)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::scalar::MagicConstTest::new(Str::from_static("testGetName"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testGetName((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => cast::<crate::php_parser::node::scalar::MagicConst>(__a), None => unreachable!("no default for crate::php_parser::node::scalar::MagicConst") }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::scalar::MagicConstTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Scalar_NumberTest__testRawValue() {
    php_rt::testing::run("PhpParser\\Node\\Scalar\\NumberTest::testRawValue", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::scalar::NumberTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::scalar::NumberTest::new(Str::from_static("testRawValue"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testRawValue();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::scalar::NumberTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Scalar_StringTest__testRawValue() {
    php_rt::testing::run("PhpParser\\Node\\Scalar\\StringTest::testRawValue", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::scalar::StringTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::scalar::StringTest::new(Str::from_static("testRawValue"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testRawValue();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::scalar::StringTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Scalar_StringTest__testParseEscapeSequences() {
    php_rt::testing::run("PhpParser\\Node\\Scalar\\StringTest::testParseEscapeSequences", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::scalar::StringTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1864 = crate::php_parser::node::scalar::StringTest::provideTestParseEscapeSequences()?; List::from_vec(vec![U_Tup3_Str_Str_Opt_Null_or_Tup3_Str_Str_Str::Tup3_Str_Str_Str(__c1864.0), U_Tup3_Str_Str_Opt_Null_or_Tup3_Str_Str_Str::Tup3_Str_Str_Str(__c1864.1), U_Tup3_Str_Str_Opt_Null_or_Tup3_Str_Str_Str::Tup3_Str_Str_Opt_Null(__c1864.2), U_Tup3_Str_Str_Opt_Null_or_Tup3_Str_Str_Str::Tup3_Str_Str_Opt_Null(__c1864.3), U_Tup3_Str_Str_Opt_Null_or_Tup3_Str_Str_Str::Tup3_Str_Str_Opt_Null(__c1864.4), U_Tup3_Str_Str_Opt_Null_or_Tup3_Str_Str_Str::Tup3_Str_Str_Opt_Null(__c1864.5), U_Tup3_Str_Str_Opt_Null_or_Tup3_Str_Str_Str::Tup3_Str_Str_Opt_Null(__c1864.6), U_Tup3_Str_Str_Opt_Null_or_Tup3_Str_Str_Str::Tup3_Str_Str_Opt_Null(__c1864.7), U_Tup3_Str_Str_Opt_Null_or_Tup3_Str_Str_Str::Tup3_Str_Str_Opt_Null(__c1864.8), U_Tup3_Str_Str_Opt_Null_or_Tup3_Str_Str_Str::Tup3_Str_Str_Opt_Null(__c1864.9)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::scalar::StringTest::new(Str::from_static("testParseEscapeSequences"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testParseEscapeSequences((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(2).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::scalar::StringTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Scalar_StringTest__testCreate() {
    php_rt::testing::run("PhpParser\\Node\\Scalar\\StringTest::testCreate", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::scalar::StringTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::node::scalar::StringTest::provideTestParse()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::scalar::StringTest::new(Str::from_static("testCreate"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testCreate(__row.0.clone(), cast::<Mixed>(__row.1.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::scalar::StringTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassConstTest__testModifiers() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassConstTest::testModifiers", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassConstTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1865 = crate::php_parser::node::stmt::ClassConstTest::provideModifiers()?; List::from_vec(vec![__c1865.0, __c1865.1, __c1865.2, __c1865.3]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::stmt::ClassConstTest::new(Str::from_static("testModifiers"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testModifiers(cast::<Mixed>(__row.0.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::stmt::ClassConstTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassConstTest__testNoModifiers() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassConstTest::testNoModifiers", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassConstTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::ClassConstTest::new(Str::from_static("testNoModifiers"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNoModifiers();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::ClassConstTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassMethodTest__testModifiers() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassMethodTest::testModifiers", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassMethodTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1866 = crate::php_parser::node::stmt::ClassMethodTest::provideModifiers()?; List::from_vec(vec![__c1866.0, __c1866.1, __c1866.2, __c1866.3, __c1866.4, __c1866.5]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::stmt::ClassMethodTest::new(Str::from_static("testModifiers"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testModifiers(cast::<Mixed>(__row.0.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::stmt::ClassMethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassMethodTest__testNoModifiers() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassMethodTest::testNoModifiers", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassMethodTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::ClassMethodTest::new(Str::from_static("testNoModifiers"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNoModifiers();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::ClassMethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassMethodTest__testImplicitPublic() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassMethodTest::testImplicitPublic", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassMethodTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1867 = crate::php_parser::node::stmt::ClassMethodTest::implicitPublicModifiers()?; List::from_vec(vec![__c1867.0, __c1867.1, __c1867.2]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::stmt::ClassMethodTest::new(Str::from_static("testImplicitPublic"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testImplicitPublic(__row.0.clone());
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::stmt::ClassMethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassMethodTest__testMagic() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassMethodTest::testMagic", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassMethodTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::node::stmt::ClassMethodTest::provideMagics()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::stmt::ClassMethodTest::new(Str::from_static("testMagic"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testMagic(__row.0.clone());
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::stmt::ClassMethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassMethodTest__testFunctionLike() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassMethodTest::testFunctionLike", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassMethodTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::ClassMethodTest::new(Str::from_static("testFunctionLike"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testFunctionLike();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::ClassMethodTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassTest__testIsAbstract() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassTest::testIsAbstract", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::ClassTest::new(Str::from_static("testIsAbstract"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testIsAbstract();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassTest__testIsFinal() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassTest::testIsFinal", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::ClassTest::new(Str::from_static("testIsFinal"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testIsFinal();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassTest__testGetTraitUses() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassTest::testGetTraitUses", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::ClassTest::new(Str::from_static("testGetTraitUses"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetTraitUses();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassTest__testGetMethods() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassTest::testGetMethods", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::ClassTest::new(Str::from_static("testGetMethods"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetMethods();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassTest__testGetConstants() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassTest::testGetConstants", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::ClassTest::new(Str::from_static("testGetConstants"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetConstants();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassTest__testGetProperties() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassTest::testGetProperties", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::ClassTest::new(Str::from_static("testGetProperties"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetProperties();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassTest__testGetProperty() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassTest::testGetProperty", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::ClassTest::new(Str::from_static("testGetProperty"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetProperty();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_ClassTest__testGetMethod() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\ClassTest::testGetMethod", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::ClassTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::ClassTest::new(Str::from_static("testGetMethod"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetMethod();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::ClassTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_InterfaceTest__testGetMethods() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\InterfaceTest::testGetMethods", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::InterfaceTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::InterfaceTest::new(Str::from_static("testGetMethods"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetMethods();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::InterfaceTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_InterfaceTest__testGetConstants() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\InterfaceTest::testGetConstants", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::InterfaceTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::InterfaceTest::new(Str::from_static("testGetConstants"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetConstants();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::InterfaceTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_PropertyTest__testModifiers() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\PropertyTest::testModifiers", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::PropertyTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1868 = crate::php_parser::node::stmt::PropertyTest::provideModifiers()?; List::from_vec(vec![__c1868.0, __c1868.1, __c1868.2, __c1868.3, __c1868.4]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node::stmt::PropertyTest::new(Str::from_static("testModifiers"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testModifiers(cast::<Mixed>(__row.0.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node::stmt::PropertyTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_PropertyTest__testNoModifiers() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\PropertyTest::testNoModifiers", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::PropertyTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::PropertyTest::new(Str::from_static("testNoModifiers"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNoModifiers();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::PropertyTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_PropertyTest__testStaticImplicitlyPublic() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\PropertyTest::testStaticImplicitlyPublic", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::PropertyTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::PropertyTest::new(Str::from_static("testStaticImplicitlyPublic"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testStaticImplicitlyPublic();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::PropertyTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_PropertyTest__testSetVisibility() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\PropertyTest::testSetVisibility", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::PropertyTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::PropertyTest::new(Str::from_static("testSetVisibility"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testSetVisibility();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::PropertyTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_PropertyTest__testIsFinal() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\PropertyTest::testIsFinal", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::PropertyTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::PropertyTest::new(Str::from_static("testIsFinal"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testIsFinal();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::PropertyTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Node_Stmt_PropertyTest__testIsAbstract() {
    php_rt::testing::run("PhpParser\\Node\\Stmt\\PropertyTest::testIsAbstract", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node::stmt::PropertyTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node::stmt::PropertyTest::new(Str::from_static("testIsAbstract"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testIsAbstract();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node::stmt::PropertyTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeAbstractTest__testConstruct() {
    php_rt::testing::run("PhpParser\\NodeAbstractTest::testConstruct", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeAbstractTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1869 = crate::php_parser::NodeAbstractTest::provideNodes()?; List::from_vec(vec![__c1869.0]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::NodeAbstractTest::new(Str::from_static("testConstruct"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testConstruct({ let __c1870 = __row.0.clone(); let mut __m: Map<ArrayKey, Mixed> = Default::default(); __m.insert(ArrayKey::from(Str::from_static("startLine")), cast::<Mixed>(__c1870.startLine)); __m.insert(ArrayKey::from(Str::from_static("endLine")), cast::<Mixed>(__c1870.endLine)); __m.insert(ArrayKey::from(Str::from_static("startTokenPos")), cast::<Mixed>(__c1870.startTokenPos)); __m.insert(ArrayKey::from(Str::from_static("endTokenPos")), cast::<Mixed>(__c1870.endTokenPos)); __m.insert(ArrayKey::from(Str::from_static("startFilePos")), cast::<Mixed>(__c1870.startFilePos)); __m.insert(ArrayKey::from(Str::from_static("endFilePos")), cast::<Mixed>(__c1870.endFilePos)); __m.insert(ArrayKey::from(Str::from_static("comments")), { let __c1871 = __c1870.comments; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1871.0)); __m.push(cast::<Mixed>(__c1871.1)); __m.push(cast::<Mixed>(__c1871.2)); Mixed::Arr(__m) }); __m }, cast::<crate::php_parser::Node>(__row.1.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::NodeAbstractTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeAbstractTest__testGetDocComment() {
    php_rt::testing::run("PhpParser\\NodeAbstractTest::testGetDocComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeAbstractTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1872 = crate::php_parser::NodeAbstractTest::provideNodes()?; List::from_vec(vec![__c1872.0]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::NodeAbstractTest::new(Str::from_static("testGetDocComment"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testGetDocComment({ let __c1873 = __row.0.clone(); let mut __m: Map<ArrayKey, Mixed> = Default::default(); __m.insert(ArrayKey::from(Str::from_static("startLine")), cast::<Mixed>(__c1873.startLine)); __m.insert(ArrayKey::from(Str::from_static("endLine")), cast::<Mixed>(__c1873.endLine)); __m.insert(ArrayKey::from(Str::from_static("startTokenPos")), cast::<Mixed>(__c1873.startTokenPos)); __m.insert(ArrayKey::from(Str::from_static("endTokenPos")), cast::<Mixed>(__c1873.endTokenPos)); __m.insert(ArrayKey::from(Str::from_static("startFilePos")), cast::<Mixed>(__c1873.startFilePos)); __m.insert(ArrayKey::from(Str::from_static("endFilePos")), cast::<Mixed>(__c1873.endFilePos)); __m.insert(ArrayKey::from(Str::from_static("comments")), { let __c1874 = __c1873.comments; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1874.0)); __m.push(cast::<Mixed>(__c1874.1)); __m.push(cast::<Mixed>(__c1874.2)); Mixed::Arr(__m) }); __m }, cast::<crate::php_parser::Node>(__row.1.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::NodeAbstractTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeAbstractTest__testSetDocComment() {
    php_rt::testing::run("PhpParser\\NodeAbstractTest::testSetDocComment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeAbstractTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeAbstractTest::new(Str::from_static("testSetDocComment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testSetDocComment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeAbstractTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeAbstractTest__testChange() {
    php_rt::testing::run("PhpParser\\NodeAbstractTest::testChange", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeAbstractTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1875 = crate::php_parser::NodeAbstractTest::provideNodes()?; List::from_vec(vec![__c1875.0]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::NodeAbstractTest::new(Str::from_static("testChange"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testChange({ let __c1876 = __row.0.clone(); let mut __m: Map<ArrayKey, Mixed> = Default::default(); __m.insert(ArrayKey::from(Str::from_static("startLine")), cast::<Mixed>(__c1876.startLine)); __m.insert(ArrayKey::from(Str::from_static("endLine")), cast::<Mixed>(__c1876.endLine)); __m.insert(ArrayKey::from(Str::from_static("startTokenPos")), cast::<Mixed>(__c1876.startTokenPos)); __m.insert(ArrayKey::from(Str::from_static("endTokenPos")), cast::<Mixed>(__c1876.endTokenPos)); __m.insert(ArrayKey::from(Str::from_static("startFilePos")), cast::<Mixed>(__c1876.startFilePos)); __m.insert(ArrayKey::from(Str::from_static("endFilePos")), cast::<Mixed>(__c1876.endFilePos)); __m.insert(ArrayKey::from(Str::from_static("comments")), { let __c1877 = __c1876.comments; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1877.0)); __m.push(cast::<Mixed>(__c1877.1)); __m.push(cast::<Mixed>(__c1877.2)); Mixed::Arr(__m) }); __m }, __row.1.clone());
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::NodeAbstractTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeAbstractTest__testIteration() {
    php_rt::testing::run("PhpParser\\NodeAbstractTest::testIteration", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeAbstractTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1878 = crate::php_parser::NodeAbstractTest::provideNodes()?; List::from_vec(vec![__c1878.0]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::NodeAbstractTest::new(Str::from_static("testIteration"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testIteration({ let __c1879 = __row.0.clone(); let mut __m: Map<ArrayKey, Mixed> = Default::default(); __m.insert(ArrayKey::from(Str::from_static("startLine")), cast::<Mixed>(__c1879.startLine)); __m.insert(ArrayKey::from(Str::from_static("endLine")), cast::<Mixed>(__c1879.endLine)); __m.insert(ArrayKey::from(Str::from_static("startTokenPos")), cast::<Mixed>(__c1879.startTokenPos)); __m.insert(ArrayKey::from(Str::from_static("endTokenPos")), cast::<Mixed>(__c1879.endTokenPos)); __m.insert(ArrayKey::from(Str::from_static("startFilePos")), cast::<Mixed>(__c1879.startFilePos)); __m.insert(ArrayKey::from(Str::from_static("endFilePos")), cast::<Mixed>(__c1879.endFilePos)); __m.insert(ArrayKey::from(Str::from_static("comments")), { let __c1880 = __c1879.comments; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1880.0)); __m.push(cast::<Mixed>(__c1880.1)); __m.push(cast::<Mixed>(__c1880.2)); Mixed::Arr(__m) }); __m }, cast::<crate::php_parser::Node>(__row.1.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::NodeAbstractTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeAbstractTest__testAttributes() {
    php_rt::testing::run("PhpParser\\NodeAbstractTest::testAttributes", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeAbstractTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeAbstractTest::new(Str::from_static("testAttributes"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAttributes();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeAbstractTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeAbstractTest__testJsonSerialization() {
    php_rt::testing::run("PhpParser\\NodeAbstractTest::testJsonSerialization", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeAbstractTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeAbstractTest::new(Str::from_static("testJsonSerialization"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testJsonSerialization();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeAbstractTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeDumperTest__testDump() {
    php_rt::testing::run("PhpParser\\NodeDumperTest::testDump", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeDumperTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1881 = crate::php_parser::NodeDumperTest::provideTestDump()?; List::from_vec(vec![U_Tup2_Map_ArrayKey_Mixed_Str_or_Tup2_PhpParser_Node_Expr_Array__Str_o_c5e731aedd::Tup2_Map_ArrayKey_Mixed_Str(__c1881.0), U_Tup2_Map_ArrayKey_Mixed_Str_or_Tup2_PhpParser_Node_Expr_Array__Str_o_c5e731aedd::Tup2_Shape_0_Str_1_Str_Key_Str_Str(__c1881.1), U_Tup2_Map_ArrayKey_Mixed_Str_or_Tup2_PhpParser_Node_Expr_Array__Str_o_c5e731aedd::Tup2_PhpParser_Node_Name_Str(__c1881.2), U_Tup2_Map_ArrayKey_Mixed_Str_or_Tup2_PhpParser_Node_Expr_Array__Str_o_c5e731aedd::Tup2_PhpParser_Node_Expr_Array__Str(__c1881.3)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::NodeDumperTest::new(Str::from_static("testDump"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testDump((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::NodeDumperTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeDumperTest__testDumpWithPositions() {
    php_rt::testing::run("PhpParser\\NodeDumperTest::testDumpWithPositions", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeDumperTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeDumperTest::new(Str::from_static("testDumpWithPositions"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testDumpWithPositions();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeDumperTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeDumperTest__testError() {
    php_rt::testing::run("PhpParser\\NodeDumperTest::testError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeDumperTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeDumperTest::new(Str::from_static("testError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeDumperTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeFinderTest__testFind() {
    php_rt::testing::run("PhpParser\\NodeFinderTest::testFind", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeFinderTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeFinderTest::new(Str::from_static("testFind"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testFind();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeFinderTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeFinderTest__testFindInstanceOf() {
    php_rt::testing::run("PhpParser\\NodeFinderTest::testFindInstanceOf", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeFinderTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeFinderTest::new(Str::from_static("testFindInstanceOf"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testFindInstanceOf();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeFinderTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeFinderTest__testFindFirst() {
    php_rt::testing::run("PhpParser\\NodeFinderTest::testFindFirst", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeFinderTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeFinderTest::new(Str::from_static("testFindFirst"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testFindFirst();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeFinderTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeFinderTest__testFindFirstInstanceOf() {
    php_rt::testing::run("PhpParser\\NodeFinderTest::testFindFirstInstanceOf", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeFinderTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeFinderTest::new(Str::from_static("testFindFirstInstanceOf"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testFindFirstInstanceOf();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeFinderTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testNonModifying() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testNonModifying", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testNonModifying"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNonModifying();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testModifying() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testModifying", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testModifying"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testModifying();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testRemoveFromLeave() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testRemoveFromLeave", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testRemoveFromLeave"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testRemoveFromLeave();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testRemoveFromEnter() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testRemoveFromEnter", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testRemoveFromEnter"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testRemoveFromEnter();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testReturnArrayFromEnter() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testReturnArrayFromEnter", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testReturnArrayFromEnter"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testReturnArrayFromEnter();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testMerge() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testMerge", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testMerge"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testMerge();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testInvalidDeepArray() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testInvalidDeepArray", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testInvalidDeepArray"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidDeepArray();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testDontTraverseChildren() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testDontTraverseChildren", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testDontTraverseChildren"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testDontTraverseChildren();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testDontTraverseCurrentAndChildren() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testDontTraverseCurrentAndChildren", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testDontTraverseCurrentAndChildren"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testDontTraverseCurrentAndChildren();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testStopTraversal() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testStopTraversal", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testStopTraversal"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testStopTraversal();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testReplaceWithNull() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testReplaceWithNull", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testReplaceWithNull"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testReplaceWithNull();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testRemovingVisitor() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testRemovingVisitor", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testRemovingVisitor"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testRemovingVisitor();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testNoCloneNodes() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testNoCloneNodes", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testNoCloneNodes"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNoCloneNodes();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeTraverserTest__testInvalidReturn() {
    php_rt::testing::run("PhpParser\\NodeTraverserTest::testInvalidReturn", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::NodeTraverserTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1882 = crate::php_parser::NodeTraverserTest::provideTestInvalidReturn()?; List::from_vec(vec![__c1882.0, __c1882.1, __c1882.2, __c1882.3, __c1882.4, __c1882.5, __c1882.6, __c1882.7, __c1882.8, __c1882.9]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::NodeTraverserTest::new(Str::from_static("testInvalidReturn"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testInvalidReturn({ let __c1883 = __row.0.clone(); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1883.0)); Mixed::Arr(__m) }, cast::<Mixed>(__row.1.clone()), cast::<Mixed>(__row.2.clone()));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::NodeTraverserTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_FindingVisitorTest__testFindVariables() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\FindingVisitorTest::testFindVariables", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::FindingVisitorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::FindingVisitorTest::new(Str::from_static("testFindVariables"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testFindVariables();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::FindingVisitorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_FindingVisitorTest__testFindAll() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\FindingVisitorTest::testFindAll", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::FindingVisitorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::FindingVisitorTest::new(Str::from_static("testFindAll"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testFindAll();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::FindingVisitorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_FirstFindingVisitorTest__testFindFirstVariable() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\FirstFindingVisitorTest::testFindFirstVariable", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::FirstFindingVisitorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::FirstFindingVisitorTest::new(Str::from_static("testFindFirstVariable"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testFindFirstVariable();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::FirstFindingVisitorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_FirstFindingVisitorTest__testFindNone() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\FirstFindingVisitorTest::testFindNone", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::FirstFindingVisitorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::FirstFindingVisitorTest::new(Str::from_static("testFindNone"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testFindNone();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::FirstFindingVisitorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_NameResolverTest__testResolveNames() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\NameResolverTest::testResolveNames", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::NameResolverTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::NameResolverTest::new(Str::from_static("testResolveNames"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testResolveNames();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::NameResolverTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_NameResolverTest__testResolveLocations() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\NameResolverTest::testResolveLocations", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::NameResolverTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::NameResolverTest::new(Str::from_static("testResolveLocations"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testResolveLocations();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::NameResolverTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_NameResolverTest__testNoResolveSpecialName() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\NameResolverTest::testNoResolveSpecialName", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::NameResolverTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::NameResolverTest::new(Str::from_static("testNoResolveSpecialName"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testNoResolveSpecialName();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::NameResolverTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_NameResolverTest__testAddDeclarationNamespacedName() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\NameResolverTest::testAddDeclarationNamespacedName", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::NameResolverTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::NameResolverTest::new(Str::from_static("testAddDeclarationNamespacedName"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddDeclarationNamespacedName();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::NameResolverTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_NameResolverTest__testAddRuntimeResolvedNamespacedName() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\NameResolverTest::testAddRuntimeResolvedNamespacedName", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::NameResolverTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::NameResolverTest::new(Str::from_static("testAddRuntimeResolvedNamespacedName"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddRuntimeResolvedNamespacedName();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::NameResolverTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_NameResolverTest__testError() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\NameResolverTest::testError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::NameResolverTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1884 = crate::php_parser::node_visitor::NameResolverTest::provideTestError()?; List::from_vec(vec![U_Tup2_PhpParser_Node_Expr_New__Str_or_Tup2_PhpParser_Node_Stmt_Use__Str::Tup2_PhpParser_Node_Stmt_Use__Str(__c1884.0), U_Tup2_PhpParser_Node_Expr_New__Str_or_Tup2_PhpParser_Node_Stmt_Use__Str::Tup2_PhpParser_Node_Stmt_Use__Str(__c1884.1), U_Tup2_PhpParser_Node_Expr_New__Str_or_Tup2_PhpParser_Node_Stmt_Use__Str::Tup2_PhpParser_Node_Stmt_Use__Str(__c1884.2), U_Tup2_PhpParser_Node_Expr_New__Str_or_Tup2_PhpParser_Node_Stmt_Use__Str::Tup2_PhpParser_Node_Expr_New__Str(__c1884.3), U_Tup2_PhpParser_Node_Expr_New__Str_or_Tup2_PhpParser_Node_Stmt_Use__Str::Tup2_PhpParser_Node_Expr_New__Str(__c1884.4), U_Tup2_PhpParser_Node_Expr_New__Str_or_Tup2_PhpParser_Node_Stmt_Use__Str::Tup2_PhpParser_Node_Expr_New__Str(__c1884.5), U_Tup2_PhpParser_Node_Expr_New__Str_or_Tup2_PhpParser_Node_Stmt_Use__Str::Tup2_PhpParser_Node_Expr_New__Str(__c1884.6)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::node_visitor::NameResolverTest::new(Str::from_static("testError"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testError((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => cast::<crate::php_parser::Node>(__a), None => unreachable!("no default for crate::php_parser::Node") }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::node_visitor::NameResolverTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_NameResolverTest__testClassNameIsCaseInsensitive() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\NameResolverTest::testClassNameIsCaseInsensitive", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::NameResolverTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::NameResolverTest::new(Str::from_static("testClassNameIsCaseInsensitive"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testClassNameIsCaseInsensitive();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::NameResolverTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_NameResolverTest__testSpecialClassNamesAreCaseInsensitive() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\NameResolverTest::testSpecialClassNamesAreCaseInsensitive", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::NameResolverTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::NameResolverTest::new(Str::from_static("testSpecialClassNamesAreCaseInsensitive"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testSpecialClassNamesAreCaseInsensitive();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::NameResolverTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_NameResolverTest__testAddOriginalNames() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\NameResolverTest::testAddOriginalNames", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::NameResolverTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::NameResolverTest::new(Str::from_static("testAddOriginalNames"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAddOriginalNames();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::NameResolverTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_NameResolverTest__testAttributeOnlyMode() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\NameResolverTest::testAttributeOnlyMode", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::NameResolverTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::NameResolverTest::new(Str::from_static("testAttributeOnlyMode"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAttributeOnlyMode();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::NameResolverTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_NodeConnectingVisitorTest__testConnectsNodeToItsParentNodeAndItsSiblingNodes() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\NodeConnectingVisitorTest::testConnectsNodeToItsParentNodeAndItsSiblingNodes", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::NodeConnectingVisitorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::NodeConnectingVisitorTest::new(Str::from_static("testConnectsNodeToItsParentNodeAndItsSiblingNodes"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testConnectsNodeToItsParentNodeAndItsSiblingNodes();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::NodeConnectingVisitorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_NodeConnectingVisitorTest__testWeakReferences() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\NodeConnectingVisitorTest::testWeakReferences", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::NodeConnectingVisitorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::NodeConnectingVisitorTest::new(Str::from_static("testWeakReferences"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testWeakReferences();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::NodeConnectingVisitorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_ParentConnectingVisitorTest__testConnectsChildNodeToParentNode() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\ParentConnectingVisitorTest::testConnectsChildNodeToParentNode", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::ParentConnectingVisitorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::ParentConnectingVisitorTest::new(Str::from_static("testConnectsChildNodeToParentNode"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testConnectsChildNodeToParentNode();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::ParentConnectingVisitorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_NodeVisitor_ParentConnectingVisitorTest__testWeakReferences() {
    php_rt::testing::run("PhpParser\\NodeVisitor\\ParentConnectingVisitorTest::testWeakReferences", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::node_visitor::ParentConnectingVisitorTest::setUpBeforeClass()?;
        let __t = crate::php_parser::node_visitor::ParentConnectingVisitorTest::new(Str::from_static("testWeakReferences"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testWeakReferences();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::node_visitor::ParentConnectingVisitorTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php7Test__testParserThrowsSyntaxError() {
    php_rt::testing::run("PhpParser\\Parser\\Php7Test::testParserThrowsSyntaxError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php7Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php7Test::new(Str::from_static("testParserThrowsSyntaxError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testParserThrowsSyntaxError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php7Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php7Test__testParserThrowsSpecialError() {
    php_rt::testing::run("PhpParser\\Parser\\Php7Test::testParserThrowsSpecialError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php7Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php7Test::new(Str::from_static("testParserThrowsSpecialError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testParserThrowsSpecialError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php7Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php7Test__testParserThrowsLexerError() {
    php_rt::testing::run("PhpParser\\Parser\\Php7Test::testParserThrowsLexerError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php7Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php7Test::new(Str::from_static("testParserThrowsLexerError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testParserThrowsLexerError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php7Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php7Test__testAttributeAssignment() {
    php_rt::testing::run("PhpParser\\Parser\\Php7Test::testAttributeAssignment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php7Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php7Test::new(Str::from_static("testAttributeAssignment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAttributeAssignment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php7Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php7Test__testInvalidToken() {
    php_rt::testing::run("PhpParser\\Parser\\Php7Test::testInvalidToken", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php7Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php7Test::new(Str::from_static("testInvalidToken"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidToken();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php7Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php7Test__testExtraAttributes() {
    php_rt::testing::run("PhpParser\\Parser\\Php7Test::testExtraAttributes", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php7Test::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::parser::Php7Test::provideTestExtraAttributes()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::parser::Php7Test::new(Str::from_static("testExtraAttributes"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testExtraAttributes((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::parser::Php7Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php7Test__testListKindAttribute() {
    php_rt::testing::run("PhpParser\\Parser\\Php7Test::testListKindAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php7Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php7Test::new(Str::from_static("testListKindAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testListKindAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php7Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php7Test__testGetTokens() {
    php_rt::testing::run("PhpParser\\Parser\\Php7Test::testGetTokens", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php7Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php7Test::new(Str::from_static("testGetTokens"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetTokens();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php7Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php8Test__testParserThrowsSyntaxError() {
    php_rt::testing::run("PhpParser\\Parser\\Php8Test::testParserThrowsSyntaxError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php8Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php8Test::new(Str::from_static("testParserThrowsSyntaxError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testParserThrowsSyntaxError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php8Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php8Test__testParserThrowsSpecialError() {
    php_rt::testing::run("PhpParser\\Parser\\Php8Test::testParserThrowsSpecialError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php8Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php8Test::new(Str::from_static("testParserThrowsSpecialError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testParserThrowsSpecialError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php8Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php8Test__testParserThrowsLexerError() {
    php_rt::testing::run("PhpParser\\Parser\\Php8Test::testParserThrowsLexerError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php8Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php8Test::new(Str::from_static("testParserThrowsLexerError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testParserThrowsLexerError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php8Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php8Test__testAttributeAssignment() {
    php_rt::testing::run("PhpParser\\Parser\\Php8Test::testAttributeAssignment", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php8Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php8Test::new(Str::from_static("testAttributeAssignment"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testAttributeAssignment();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php8Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php8Test__testInvalidToken() {
    php_rt::testing::run("PhpParser\\Parser\\Php8Test::testInvalidToken", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php8Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php8Test::new(Str::from_static("testInvalidToken"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidToken();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php8Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php8Test__testExtraAttributes() {
    php_rt::testing::run("PhpParser\\Parser\\Php8Test::testExtraAttributes", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php8Test::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::parser::Php8Test::provideTestExtraAttributes()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::parser::Php8Test::new(Str::from_static("testExtraAttributes"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testExtraAttributes((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::parser::Php8Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php8Test__testListKindAttribute() {
    php_rt::testing::run("PhpParser\\Parser\\Php8Test::testListKindAttribute", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php8Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php8Test::new(Str::from_static("testListKindAttribute"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testListKindAttribute();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php8Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_Parser_Php8Test__testGetTokens() {
    php_rt::testing::run("PhpParser\\Parser\\Php8Test::testGetTokens", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::parser::Php8Test::setUpBeforeClass()?;
        let __t = crate::php_parser::parser::Php8Test::new(Str::from_static("testGetTokens"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetTokens();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::parser::Php8Test::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_ParserFactoryTest__testCreate() {
    php_rt::testing::run("PhpParser\\ParserFactoryTest::testCreate", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::ParserFactoryTest::setUpBeforeClass()?;
        let __t = crate::php_parser::ParserFactoryTest::new(Str::from_static("testCreate"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testCreate();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::ParserFactoryTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PhpVersionTest__testConstruction() {
    php_rt::testing::run("PhpParser\\PhpVersionTest::testConstruction", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PhpVersionTest::setUpBeforeClass()?;
        let __t = crate::php_parser::PhpVersionTest::new(Str::from_static("testConstruction"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testConstruction();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::PhpVersionTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PhpVersionTest__testInvalidVersion() {
    php_rt::testing::run("PhpParser\\PhpVersionTest::testInvalidVersion", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PhpVersionTest::setUpBeforeClass()?;
        let __t = crate::php_parser::PhpVersionTest::new(Str::from_static("testInvalidVersion"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidVersion();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::PhpVersionTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PhpVersionTest__testEquals() {
    php_rt::testing::run("PhpParser\\PhpVersionTest::testEquals", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PhpVersionTest::setUpBeforeClass()?;
        let __t = crate::php_parser::PhpVersionTest::new(Str::from_static("testEquals"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testEquals();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::PhpVersionTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testPrettyPrint() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testPrettyPrint", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        for (__key, __row) in cast::<Map<ArrayKey, Mixed>>(crate::php_parser::PrettyPrinterTest::provideTestPrettyPrint()?).into_iter() {
            let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testPrettyPrint"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testPrettyPrint((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(2).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(3).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testPrettyPrintFile() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testPrettyPrintFile", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        for (__key, __row) in cast::<Map<ArrayKey, Mixed>>(crate::php_parser::PrettyPrinterTest::provideTestPrettyPrintFile()?).into_iter() {
            let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testPrettyPrintFile"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testPrettyPrintFile((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(2).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(3).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testPrettyPrintExpr() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testPrettyPrintExpr", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testPrettyPrintExpr"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testPrettyPrintExpr();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testCommentBeforeInlineHTML() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testCommentBeforeInlineHTML", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testCommentBeforeInlineHTML"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testCommentBeforeInlineHTML();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testArraySyntaxDefault() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testArraySyntaxDefault", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testArraySyntaxDefault"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testArraySyntaxDefault();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testKindAttributes() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testKindAttributes", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::PrettyPrinterTest::provideTestKindAttributes()?.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testKindAttributes"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testKindAttributes((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testUnnaturalLiterals() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testUnnaturalLiterals", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1885 = crate::php_parser::PrettyPrinterTest::provideTestUnnaturalLiterals()?; List::from_vec(vec![U_Tup2_PhpParser_Node_Scalar_Float__Str_or_Tup2_PhpParser_Node_Scalar_Int__Str::Tup2_PhpParser_Node_Scalar_Int__Str(__c1885.0), U_Tup2_PhpParser_Node_Scalar_Float__Str_or_Tup2_PhpParser_Node_Scalar_Int__Str::Tup2_PhpParser_Node_Scalar_Int__Str(__c1885.1), U_Tup2_PhpParser_Node_Scalar_Float__Str_or_Tup2_PhpParser_Node_Scalar_Int__Str::Tup2_PhpParser_Node_Scalar_Int__Str(__c1885.2), U_Tup2_PhpParser_Node_Scalar_Float__Str_or_Tup2_PhpParser_Node_Scalar_Int__Str::Tup2_PhpParser_Node_Scalar_Int__Str(__c1885.3), U_Tup2_PhpParser_Node_Scalar_Float__Str_or_Tup2_PhpParser_Node_Scalar_Int__Str::Tup2_PhpParser_Node_Scalar_Int__Str(__c1885.4), U_Tup2_PhpParser_Node_Scalar_Float__Str_or_Tup2_PhpParser_Node_Scalar_Int__Str::Tup2_PhpParser_Node_Scalar_Float__Str(__c1885.5), U_Tup2_PhpParser_Node_Scalar_Float__Str_or_Tup2_PhpParser_Node_Scalar_Int__Str::Tup2_PhpParser_Node_Scalar_Float__Str(__c1885.6), U_Tup2_PhpParser_Node_Scalar_Float__Str_or_Tup2_PhpParser_Node_Scalar_Int__Str::Tup2_PhpParser_Node_Scalar_Float__Str(__c1885.7)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testUnnaturalLiterals"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testUnnaturalLiterals((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testPrettyPrintWithError() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testPrettyPrintWithError", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testPrettyPrintWithError"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testPrettyPrintWithError();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testPrettyPrintWithErrorInClassConstFetch() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testPrettyPrintWithErrorInClassConstFetch", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testPrettyPrintWithErrorInClassConstFetch"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testPrettyPrintWithErrorInClassConstFetch();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testFormatPreservingPrint() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testFormatPreservingPrint", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        for (__key, __row) in cast::<Map<ArrayKey, Mixed>>(crate::php_parser::PrettyPrinterTest::provideTestFormatPreservingPrint()?).into_iter() {
            let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testFormatPreservingPrint"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testFormatPreservingPrint((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(2).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(3).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(4).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testRoundTripPrint() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testRoundTripPrint", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        for (__key, __row) in crate::php_parser::PrettyPrinterTest::provideTestRoundTripPrint()?.into_iter() {
            let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testRoundTripPrint"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testRoundTripPrint((match cast::<List<Mixed>>(__row.clone()).get(0).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(1).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(2).cloned() { Some(__a) => __a, None => <Mixed>::default() }), (match cast::<List<Mixed>>(__row.clone()).get(3).cloned() { Some(__a) => __a, None => <Mixed>::default() }));
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testWindowsNewline() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testWindowsNewline", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testWindowsNewline"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testWindowsNewline();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testInvalidNewline() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testInvalidNewline", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testInvalidNewline"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidNewline();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_PrettyPrinterTest__testInvalidIndent() {
    php_rt::testing::run("PhpParser\\PrettyPrinterTest::testInvalidIndent", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::PrettyPrinterTest::setUpBeforeClass()?;
        let __t = crate::php_parser::PrettyPrinterTest::new(Str::from_static("testInvalidIndent"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testInvalidIndent();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::PrettyPrinterTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_TokenTest__testGetTokenName() {
    php_rt::testing::run("PhpParser\\TokenTest::testGetTokenName", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::TokenTest::setUpBeforeClass()?;
        let __t = crate::php_parser::TokenTest::new(Str::from_static("testGetTokenName"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testGetTokenName();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::TokenTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_TokenTest__testIs() {
    php_rt::testing::run("PhpParser\\TokenTest::testIs", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::TokenTest::setUpBeforeClass()?;
        let __t = crate::php_parser::TokenTest::new(Str::from_static("testIs"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testIs();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::TokenTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_TokenTest__testIsIgnorable() {
    php_rt::testing::run("PhpParser\\TokenTest::testIsIgnorable", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::TokenTest::setUpBeforeClass()?;
        for (__key, __row) in { let __c1886 = crate::php_parser::TokenTest::provideTestIsIgnorable()?; List::from_vec(vec![__c1886.0, __c1886.1, __c1886.2, __c1886.3, __c1886.4]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
            let __t = crate::php_parser::TokenTest::new(Str::from_static("testIsIgnorable"))?;
            php_rt::testing::case(to_str(&__key), || -> Result<(), Throw> {
                __t.runSetUp()?;
                let __outcome = __t.testIsIgnorable(__row.0.clone(), __row.1.clone(), __row.2.clone());
                let __td = __t.runTearDown();
                match __outcome {
                    Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                    Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
                }
                __td?;
                Ok(())
            })?;
        }
        crate::php_parser::TokenTest::tearDownAfterClass()?;
        Ok(())
    });
}
#[test]
fn PhpParser_TokenTest__testToString() {
    php_rt::testing::run("PhpParser\\TokenTest::testToString", "/home/daniil/repos/psalm-port", || -> Result<(), Throw> {
        crate::init();
        crate::php_parser::TokenTest::setUpBeforeClass()?;
        let __t = crate::php_parser::TokenTest::new(Str::from_static("testToString"))?;
        php_rt::testing::case(Str::from_static(""), || -> Result<(), Throw> {
            __t.runSetUp()?;
            let __outcome = __t.testToString();
            let __td = __t.runTearDown();
            match __outcome {
                Ok(_) => { if __t.expectsException()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \""), __t.expectedExceptionDescription()?, Str::from_static("\" is thrown")))); } }
                Err(__e) => { if __t.expectsException()? && !php_rt::testing::is_skip(&__e) { __t.verifyExpectedException(__e)?; } else { return Err(__e); } }
            }
            __td?;
            Ok(())
        })?;
        crate::php_parser::TokenTest::tearDownAfterClass()?;
        Ok(())
    });
}
