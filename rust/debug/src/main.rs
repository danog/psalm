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
    // --- docblock parsing probe: does @template survive DocComment::parsePreservingLength?
    {
        let text = Str::from_static("/**\n * @psalm-pure\n * @template T as string\n * @param T $string\n * @return (T is non-empty-string ? non-empty-string : string)\n */");
        let doc = psalm_base::php_parser::comment::Doc::new(text, 1, 0, 0, 5, 0, 0)?;
        match psalm_src::psalm::DocComment::parsePreservingLength(doc.clone(), false) {
            Ok(parsed) => {
                let tags = parsed.p_tags_get();
                println!("docblock tags: {}", tags.iter().map(|(k, v)| format!("{}({})", k, v.len())).collect::<Vec<_>>().join(", "));
                let combined = parsed.p_combined_tags_get();
                println!("combined tags: {}", combined.iter().map(|(k, v)| format!("{}={:?}", k, v.iter().map(|(_, s)| s.to_string()).collect::<Vec<_>>())).collect::<Vec<_>>().join("; "));
            }
            Err(e) => println!("parsePreservingLength threw: {}", e),
        }
    }
    // --- function docblock parse probe
    {
        let text = Str::from_static("/**\n * @psalm-pure\n * @template T as string\n * @param T $string\n * @return (T is non-empty-string ? non-empty-string : string)\n */");
        let doc = psalm_base::php_parser::comment::Doc::new(text, 1, 0, 0, 5, 0, 0)?;
        let parser = psalm_base::php_parser::ParserFactory::new()?.createForNewestSupportedVersion()?;
        let stmts = parser.parse(Str::from_static("<?php\nfunction sodium_bin2base64(string $string, int $id): string {}\n"), None)?.unwrap_or_default();
        let stmt = stmts.iter().next().map(|(_, s)| s.clone()).unwrap();
        let node = cast::<psalm_base::php_parser::Node>(stmt);
        let scanner = psalm_src::psalm::internal::scanner::FileScanner::new(Str::from_static("stub.php"), Str::from_static("stub.php"), false)?;
        let source = cast::<psalm_src::psalm::FileSource>(scanner);
        let loc = psalm_src::psalm::CodeLocation::new(source, node, None, false, None, None, None)?;
        match psalm_src::psalm::internal::php_visitor::reflector::FunctionLikeDocblockParser::parse(codebase.clone(), doc, loc, Str::from_static("sodium_bin2base64")) {
            Ok(info) => {
                let templates = info.p_templates_get();
                println!("function docblock templates: {}", templates.len());
                for (_, t) in templates.iter() {
                    println!("  template {:?} {:?} {:?} {} {}", t.0.to_string(), t.1.as_ref().map(|s| s.to_string()), t.2.as_ref().map(|s| s.to_string()), t.3, 0);
                }
            }
            Err(e) => println!("FunctionLikeDocblockParser::parse threw: {}", e),
        }
    }
    // --- template map probe: (T is int ? int : string) with T in the map
    {
        use psalm_src::psalm::internal::type_::{TypeParser, TypeTokenizer};
        let aliases = psalm_src::psalm::Aliases::new(None, Map::new(), Map::new(), Map::new(), Map::new(), Map::new(), Map::new())?;
        let mut inner: Map<Str, psalm_src::psalm::type_::Union> = Map::new();
        inner.insert(Str::from_static("fn-foo"), psalm_src::psalm::Type::getMixed(false, false)?);
        let mut tmap: Map<Str, Map<Str, psalm_src::psalm::type_::Union>> = Map::new();
        tmap.insert(Str::from_static("T"), inner.clone());
        let mut tmap_mixed: Map<Str, Mixed> = Map::new();
        tmap_mixed.insert(Str::from_static("T"), cast::<Mixed>(inner.clone()));
        println!("tmap has T: {} / mixed map has T: {}", tmap.contains_key(&Str::from_static("T")), tmap_mixed.contains_key(&Str::from_static("T")));
        match TypeTokenizer::getFullyQualifiedTokens(Str::from_static("(T is int ? int : string)"), aliases.clone(), Some(tmap_mixed.clone()), None, None, None, false) {
            Ok(tokens) => {
                println!("tokens: {}", tokens.len());
                match TypeParser::parseTokens(tokens, None, tmap.clone(), Map::new(), true) {
                    Ok(u) => println!("parsed: {}", u.getId(true)?),
                    Err(e) => println!("parseTokens threw: {}", e),
                }
            }
            Err(e) => println!("tokenizer threw: {}", e),
        }
        match TypeTokenizer::getFullyQualifiedTokens(Str::from_static("T"), aliases.clone(), Some(tmap_mixed.clone()), None, None, None, false) {
            Ok(tokens) => match TypeParser::parseTokens(tokens, None, tmap.clone(), Map::new(), true) {
                Ok(u) => println!("parsed T: {}", u.getId(true)?),
                Err(e) => println!("parseTokens(T) threw: {}", e),
            },
            Err(e) => println!("tokenizer(T) threw: {}", e),
        }
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
