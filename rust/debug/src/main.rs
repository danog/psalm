#![allow(unused_imports, non_snake_case)]
use php_rt::prelude::*;
use psalm_base::Throw;

fn run() -> Result<(), Throw> {
    let root = "/home/daniil/repos/psalm-port";
    php_rt::support::set_src_root(root);
    let _ = std::env::set_current_dir(root);
    psalm_tests::init();
    let t = psalm_tests::psalm::tests::BinaryOperationTest::new(Str::from_static("dbg"))?;
    t.runSetUp()?;
    let code = std::env::args().nth(1).unwrap_or_else(|| "<?php\n$a = 1 + 2;\necho $undefined;\n".to_string());
    t.addFile(Str::from_static("somefile.php"), Str::from_string(code))?;
    let ctx = psalm_src::psalm::Context::new(None)?;
    let r = t.analyzeFile(Str::from_static("somefile.php"), ctx.clone(), true, false);
    match &r {
        Ok(()) => println!("analyzeFile: ok"),
        Err(e) => println!("analyzeFile: threw {}", e),
    }
    let codebase = t.p_project_analyzer_get().getCodebase()?;
    let fs = codebase.p_file_storage_provider_get().get(Str::from_static("somefile.php"))?;
    println!("file storage: deep_scan={} has_visitor_issues={}", fs.p_deep_scan_get(), fs.p_has_visitor_issues_get());
    let stmts = codebase.getStatementsForFile(Str::from_static("somefile.php"), None)?;
    println!("statements: {}", stmts.len());
    println!("contents: {:?}", codebase.p_file_provider_get().getContents(Str::from_static("somefile.php"), true)?);
    let parser = psalm_base::php_parser::ParserFactory::new()?.createForNewestSupportedVersion()?;
    match parser.parse(Str::from_static("<?php\n$a = 1 + 2;\necho $undefined;\n"), None) {
        Ok(Some(m)) => println!("php-parser direct: {} stmts", m.len()),
        Ok(None) => println!("php-parser direct: None"),
        Err(e) => println!("php-parser direct: threw {}", e),
    }
    let mut has_errors = false;
    match psalm_src::psalm::internal::provider::StatementsProvider::parseStatements(Str::from_static("<?php\n$a = 1 + 2;\necho $undefined;\n"), 80400, &mut has_errors, Some(Str::from_static("somefile.php")), None, None, None) {
        Ok(l) => println!("parseStatements: {} stmts, has_errors={}", l.len(), has_errors),
        Err(e) => println!("parseStatements: threw {}", e),
    }
    let vars = ctx.p_vars_in_scope_get();
    println!("vars_in_scope: {} entries", vars.len());
    for (k, v) in vars.into_iter() {
        println!("  {} => {}", k, v.getId(true)?);
    }
    let issues = psalm_src::psalm::IssueBuffer::getIssuesData()?;
    println!("issues: {} files", issues.len());
    for (f, list) in issues.into_iter() {
        for i in list.into_iter() {
            println!("  {}: {} {}", f, i.p_type__get(), i.p_message_get());
        }
    }
    Ok(())
}

fn main() {
    let h = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| {
        if let Err(e) = run() {
            println!("error: {}", e);
        }
    }).unwrap();
    h.join().unwrap();
}
