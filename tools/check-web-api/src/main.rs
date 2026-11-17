//! Source inventory for the repository's public API. No nightly rustdoc required.
use quote::ToTokens;
use serde::Serialize;
use std::{collections::BTreeMap, error::Error, fs, path::Path};
use syn::{Attribute, Fields, Item, Token, Type, UseTree, Visibility, punctuated::Punctuated};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Serialize, PartialEq)]
struct Entry {
    id: String,
    kind: String,
    signature: String,
    source: String,
    conditions: Vec<String>,
    receiver: String,
}

struct Unit {
    module: String,
    source: String,
    conditions: Vec<String>,
    items: Vec<Item>,
    public: bool,
}

fn tokens(value: &impl ToTokens) -> String {
    value.to_token_stream().to_string()
}

fn public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

fn path(module: &str, name: &str) -> String {
    if module.is_empty() {
        name.into()
    } else {
        format!("{module}::{name}")
    }
}

fn conditions(parent: &[String], attrs: &[Attribute]) -> Vec<String> {
    parent
        .iter()
        .cloned()
        .chain(
            attrs
                .iter()
                .filter(|a| a.path().is_ident("cfg") || a.path().is_ident("cfg_attr"))
                .map(|a| tokens(&a.meta)),
        )
        .collect()
}

fn test_only(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|a| a.path().is_ident("cfg") && tokens(&a.meta) == "cfg (test)")
}

fn load_units(
    root: &Path,
    file: &Path,
    module: &str,
    inherited: &[String],
    exposed: bool,
    out: &mut Vec<Unit>,
) -> Result<()> {
    let parsed = syn::parse_file(&fs::read_to_string(file)?)?;
    walk_units(root, file, module, inherited, exposed, parsed.items, out)
}

fn walk_units(
    root: &Path,
    file: &Path,
    module: &str,
    inherited: &[String],
    exposed: bool,
    items: Vec<Item>,
    out: &mut Vec<Unit>,
) -> Result<()> {
    for item in &items {
        if let Item::Mod(m) = item {
            if test_only(&m.attrs) {
                continue;
            }
            let child = path(module, &m.ident.to_string());
            let cfg = conditions(inherited, &m.attrs);
            if let Some((_, nested)) = &m.content {
                walk_units(
                    root,
                    file,
                    &child,
                    &cfg,
                    exposed && public(&m.vis),
                    nested.clone(),
                    out,
                )?;
            } else {
                if m.attrs.iter().any(|a| a.path().is_ident("path")) {
                    return Err(
                        format!("explicit #[path] requires inventory support: {child}").into(),
                    );
                }
                let base = if module.is_empty() {
                    root.join("src")
                } else {
                    root.join("src").join(module.replace("::", "/"))
                };
                let flat = base.join(format!("{}.rs", m.ident));
                let file = if flat.exists() {
                    flat
                } else {
                    base.join(m.ident.to_string()).join("mod.rs")
                };
                load_units(root, &file, &child, &cfg, exposed && public(&m.vis), out)?;
            }
        }
    }
    out.push(Unit {
        module: module.into(),
        source: file.strip_prefix(root)?.to_string_lossy().into(),
        conditions: inherited.into(),
        items,
        public: exposed,
    });
    Ok(())
}

fn imports(tree: &UseTree, prefix: &str, out: &mut Vec<(String, String)>) -> Result<()> {
    match tree {
        UseTree::Path(p) => imports(&p.tree, &path(prefix, &p.ident.to_string()), out)?,
        UseTree::Name(n) => out.push((path(prefix, &n.ident.to_string()), n.ident.to_string())),
        UseTree::Rename(n) => out.push((path(prefix, &n.ident.to_string()), n.rename.to_string())),
        UseTree::Group(g) => {
            for t in &g.items {
                imports(t, prefix, out)?;
            }
        }
        UseTree::Glob(_) => return Err("glob re-exports need explicit inventory support".into()),
    }
    Ok(())
}

fn exported(raw: &str, exports: &BTreeMap<String, String>, units: &[Unit]) -> Option<String> {
    if let Some(name) = exports.get(raw) {
        return Some(name.clone());
    }
    let parent = raw.rsplit_once("::").map_or("", |(p, _)| p);
    units
        .iter()
        .any(|u| u.module == parent && u.public)
        .then(|| raw.into())
}

fn type_ident(ty: &Type) -> Option<String> {
    match ty {
        Type::Reference(t) => type_ident(&t.elem),
        Type::Path(t) => t.path.segments.last().map(|s| s.ident.to_string()),
        _ => None,
    }
}

fn insert(
    out: &mut BTreeMap<String, Entry>,
    u: &Unit,
    id: String,
    kind: &str,
    signature: String,
    attrs: &[Attribute],
    receiver: &str,
) -> Result<()> {
    let entry = Entry {
        id: id.clone(),
        kind: kind.into(),
        signature,
        source: u.source.clone(),
        conditions: conditions(&u.conditions, attrs),
        receiver: receiver.into(),
    };
    if out.insert(id.clone(), entry).is_some() {
        return Err(format!("duplicate API ID: {id}").into());
    }
    Ok(())
}

fn derived(
    out: &mut BTreeMap<String, Entry>,
    u: &Unit,
    id: &str,
    attrs: &[Attribute],
) -> Result<()> {
    for attr in attrs.iter().filter(|a| a.path().is_ident("derive")) {
        let traits = attr.parse_args_with(Punctuated::<syn::Path, Token![,]>::parse_terminated)?;
        for t in traits {
            let name = tokens(&t);
            insert(
                out,
                u,
                format!("{id}::impl {name}"),
                "derived_trait",
                format!("impl {name} for {id}"),
                attrs,
                "trait",
            )?;
        }
    }
    Ok(())
}

fn fields(
    out: &mut BTreeMap<String, Entry>,
    u: &Unit,
    id: &str,
    fields: &Fields,
    attrs: &[Attribute],
) -> Result<()> {
    for (i, f) in fields.iter().enumerate().filter(|(_, f)| public(&f.vis)) {
        let name = f
            .ident
            .as_ref()
            .map_or_else(|| i.to_string(), ToString::to_string);
        let mut all = attrs.to_vec();
        all.extend(f.attrs.clone());
        insert(
            out,
            u,
            format!("{id}.{name}"),
            "field",
            format!("{name}: {}", tokens(&f.ty)),
            &all,
            "field",
        )?;
    }
    Ok(())
}

fn inventory(root: &Path) -> Result<Vec<Entry>> {
    let mut units = Vec::new();
    load_units(root, &root.join("src/lib.rs"), "", &[], true, &mut units)?;
    let mut exports = BTreeMap::new();
    let mut out = BTreeMap::new();
    for u in &units {
        for item in &u.items {
            if let Item::Use(v) = item {
                if !u.public || !public(&v.vis) {
                    continue;
                }
                let mut names = Vec::new();
                imports(&v.tree, "", &mut names)?;
                for (raw, alias) in names {
                    let raw = raw
                        .strip_prefix("crate::")
                        .map_or_else(|| path(&u.module, &raw), str::to_owned);
                    let id = path(&u.module, &alias);
                    exports.insert(raw.clone(), id.clone());
                    insert(
                        &mut out,
                        u,
                        format!("reexport::{id}"),
                        "reexport",
                        format!("pub use {raw} as {alias}"),
                        &v.attrs,
                        "none",
                    )?;
                }
            }
        }
    }
    let mut types = BTreeMap::new();
    for u in &units {
        for item in &u.items {
            let decl = match item {
                Item::Struct(v) if public(&v.vis) => Some((&v.ident, &v.attrs)),
                Item::Enum(v) if public(&v.vis) => Some((&v.ident, &v.attrs)),
                _ => None,
            };
            if let Some((name, _)) = decl {
                if let Some(id) = exported(&path(&u.module, &name.to_string()), &exports, &units) {
                    if types.insert(name.to_string(), id).is_some() {
                        return Err(format!("ambiguous public type {name}").into());
                    }
                }
            }
        }
    }
    for u in &units {
        for item in &u.items {
            match item {
                Item::Struct(v)
                    if public(&v.vis)
                        && exported(&path(&u.module, &v.ident.to_string()), &exports, &units)
                            .is_some() =>
                {
                    let id = &types[&v.ident.to_string()];
                    insert(
                        &mut out,
                        u,
                        id.clone(),
                        "struct",
                        format!("pub struct {}{}", v.ident, tokens(&v.generics)),
                        &v.attrs,
                        "none",
                    )?;
                    fields(&mut out, u, id, &v.fields, &v.attrs)?;
                    derived(&mut out, u, id, &v.attrs)?;
                }
                Item::Enum(v)
                    if public(&v.vis)
                        && exported(&path(&u.module, &v.ident.to_string()), &exports, &units)
                            .is_some() =>
                {
                    let id = &types[&v.ident.to_string()];
                    insert(
                        &mut out,
                        u,
                        id.clone(),
                        "enum",
                        format!("pub enum {}{}", v.ident, tokens(&v.generics)),
                        &v.attrs,
                        "none",
                    )?;
                    for variant in &v.variants {
                        let mut attrs = v.attrs.clone();
                        attrs.extend(variant.attrs.clone());
                        let mut clean = variant.clone();
                        clean.attrs.clear();
                        insert(
                            &mut out,
                            u,
                            format!("{id}::{}", variant.ident),
                            "variant",
                            tokens(&clean),
                            &attrs,
                            "none",
                        )?;
                    }
                    derived(&mut out, u, id, &v.attrs)?;
                }
                Item::Fn(v) if public(&v.vis) => {
                    if let Some(id) =
                        exported(&path(&u.module, &v.sig.ident.to_string()), &exports, &units)
                    {
                        insert(
                            &mut out,
                            u,
                            id,
                            "function",
                            tokens(&v.sig),
                            &v.attrs,
                            "none",
                        )?;
                    }
                }
                Item::Const(v) if public(&v.vis) => {
                    if let Some(id) =
                        exported(&path(&u.module, &v.ident.to_string()), &exports, &units)
                    {
                        insert(
                            &mut out,
                            u,
                            id,
                            "constant",
                            format!("const {}: {} = {}", v.ident, tokens(&v.ty), tokens(&v.expr)),
                            &v.attrs,
                            "none",
                        )?;
                    }
                }
                Item::Mod(v) if public(&v.vis) && !test_only(&v.attrs) && u.public => {
                    insert(
                        &mut out,
                        u,
                        path(&u.module, &v.ident.to_string()),
                        "module",
                        format!("pub mod {}", v.ident),
                        &v.attrs,
                        "none",
                    )?;
                }
                Item::Impl(v) => {
                    let Some(name) = type_ident(&v.self_ty) else {
                        continue;
                    };
                    let Some(id) = types.get(&name) else {
                        continue;
                    };
                    if let Some((_, t, _)) = &v.trait_ {
                        let trait_name = tokens(t);
                        let trait_id = format!("{id}::impl {trait_name}");
                        insert(
                            &mut out,
                            u,
                            trait_id.clone(),
                            "trait_impl",
                            format!(
                                "impl{} {trait_name} for {} {}",
                                tokens(&v.generics),
                                tokens(&v.self_ty),
                                v.generics
                                    .where_clause
                                    .as_ref()
                                    .map(tokens)
                                    .unwrap_or_default()
                            ),
                            &v.attrs,
                            "trait",
                        )?;
                        for member in &v.items {
                            let (name, kind, signature, member_attrs) = match member {
                                syn::ImplItem::Fn(f) => (
                                    f.sig.ident.to_string(),
                                    "trait_method",
                                    tokens(&f.sig),
                                    &f.attrs,
                                ),
                                syn::ImplItem::Type(t) => (
                                    t.ident.to_string(),
                                    "associated_type",
                                    format!(
                                        "type {}{} = {}",
                                        t.ident,
                                        tokens(&t.generics),
                                        tokens(&t.ty)
                                    ),
                                    &t.attrs,
                                ),
                                syn::ImplItem::Const(c) => (
                                    c.ident.to_string(),
                                    "associated_constant",
                                    format!(
                                        "const {}: {} = {}",
                                        c.ident,
                                        tokens(&c.ty),
                                        tokens(&c.expr)
                                    ),
                                    &c.attrs,
                                ),
                                _ => {
                                    return Err(
                                        format!("unsupported trait member in {trait_id}").into()
                                    );
                                }
                            };
                            let mut attrs = v.attrs.clone();
                            attrs.extend(member_attrs.clone());
                            insert(
                                &mut out,
                                u,
                                format!("{trait_id}::{name}"),
                                kind,
                                signature,
                                &attrs,
                                "trait",
                            )?;
                        }
                    } else {
                        for member in &v.items {
                            match member {
                                syn::ImplItem::Fn(f) if public(&f.vis) => {
                                    let receiver = f.sig.receiver().map_or("none", |r| {
                                        if r.reference.is_some() {
                                            if r.mutability.is_some() {
                                                "mutable"
                                            } else {
                                                "borrowed"
                                            }
                                        } else {
                                            "consuming"
                                        }
                                    });
                                    let mut attrs = v.attrs.clone();
                                    attrs.extend(f.attrs.clone());
                                    insert(
                                        &mut out,
                                        u,
                                        format!("{id}::{}", f.sig.ident),
                                        "method",
                                        tokens(&f.sig),
                                        &attrs,
                                        receiver,
                                    )?;
                                }
                                syn::ImplItem::Const(c) if public(&c.vis) => {
                                    let mut attrs = v.attrs.clone();
                                    attrs.extend(c.attrs.clone());
                                    insert(
                                        &mut out,
                                        u,
                                        format!("{id}::{}", c.ident),
                                        "constant",
                                        format!(
                                            "const {}: {} = {}",
                                            c.ident,
                                            tokens(&c.ty),
                                            tokens(&c.expr)
                                        ),
                                        &attrs,
                                        "none",
                                    )?;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Item::Macro(v) if v.mac.path.is_ident("from_integer") => {
                    let types = v
                        .mac
                        .parse_body_with(Punctuated::<Type, Token![,]>::parse_terminated)?;
                    for ty in types {
                        let t = tokens(&ty);
                        insert(
                            &mut out,
                            u,
                            format!("Decimal::impl From < {t} >"),
                            "macro_trait",
                            format!("impl From<{t}> for Decimal"),
                            &v.attrs,
                            "trait",
                        )?;
                    }
                }
                Item::Macro(v) if v.mac.path.is_ident("macro_rules") => {
                    if v.attrs
                        .iter()
                        .any(|a| tokens(&a.meta).contains("macro_export"))
                    {
                        return Err("exported macro definitions require inventory support".into());
                    }
                }
                Item::Macro(v) => {
                    return Err(format!(
                        "unhandled item macro in {}: {}",
                        u.source,
                        tokens(&v.mac.path)
                    )
                    .into());
                }
                Item::Type(v) if public(&v.vis) => {
                    return Err(format!("public alias needs inventory support: {}", v.ident).into());
                }
                Item::Trait(v) if public(&v.vis) => {
                    return Err(format!("public trait needs inventory support: {}", v.ident).into());
                }
                Item::Union(v) if public(&v.vis) => {
                    return Err(format!("public union needs inventory support: {}", v.ident).into());
                }
                Item::Static(v) if public(&v.vis) => {
                    return Err(
                        format!("public static needs inventory support: {}", v.ident).into(),
                    );
                }
                _ => {}
            }
        }
    }
    Ok(out.into_values().collect())
}

fn main() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let root = root.canonicalize()?;
    let args: Vec<_> = std::env::args().skip(1).collect();
    let entries = inventory(&root)?;
    let text = serde_json::to_string_pretty(&entries)? + "\n";
    let target = root.join("website/api/rust-api.json");
    match args.as_slice() {
        [arg] if arg == "--write" => {
            fs::create_dir_all(target.parent().unwrap())?;
            fs::write(target, text)?;
        }
        [arg] if arg == "--check" => {
            if fs::read_to_string(target)? != text {
                return Err("Rust API inventory drift; run cargo run -p decimal-web-api -- --write and review dispositions".into());
            }
            println!(
                "{} source-backed public API entries verified",
                entries.len()
            );
        }
        [] => print!("{text}"),
        _ => return Err("usage: decimal-web-api [--check|--write]".into()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repository_inventory_covers_hidden_and_generated_surfaces() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .unwrap();
        let entries = inventory(&root).unwrap();
        let by_id: BTreeMap<_, _> = entries.iter().map(|e| (e.id.as_str(), e)).collect();
        for id in [
            "Decimal::new",
            "NullDecimal.valid",
            "GoFormat.width",
            "ErrorKind::Unmarshal",
            "Decimal::impl From < u128 >",
            "Decimal::impl Default",
            "reexport::BigInt",
            "validation::default_validation_messages",
            "config::decode_decimal",
        ] {
            assert!(by_id.contains_key(id), "missing {id}");
        }
        assert_eq!(by_id["Decimal::into_rescaled"].receiver, "consuming");
        assert_eq!(by_id["Decimal::unmarshal_json"].receiver, "mutable");
        assert!(
            by_id["Decimal::unmarshal_json"]
                .conditions
                .iter()
                .any(|x| x.contains("json"))
        );
        assert!(!entries.iter().any(|e| e.id.contains("DecimalVisitor")
            || e.id.contains("Interval")
            || e.id.contains("tests::")));
    }
    #[test]
    fn associated_type_and_constant_changes_are_visible() {
        let root = std::env::temp_dir().join(format!("decimal-api-members-{}", std::process::id()));
        fs::create_dir_all(root.join("src")).unwrap();
        let source = "pub struct Example; impl Convert for Example { type Output = i64; const WIDTH: u32 = 64; fn convert(&self) -> Self::Output { 0 } }";
        fs::write(root.join("src/lib.rs"), source).unwrap();
        let first = inventory(&root).unwrap();
        assert!(
            first
                .iter()
                .any(|e| e.id == "Example::impl Convert::Output" && e.signature.contains("i64"))
        );
        assert!(
            first
                .iter()
                .any(|e| e.id == "Example::impl Convert::WIDTH" && e.signature.contains("64"))
        );
        fs::write(root.join("src/lib.rs"), source.replace("i64", "i32")).unwrap();
        assert_ne!(first, inventory(&root).unwrap());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn exported_macros_fail_closed() {
        let root = std::env::temp_dir().join(format!("decimal-api-macro-{}", std::process::id()));
        fs::create_dir_all(root.join("src")).unwrap();
        for attr in [
            "#[macro_export]",
            "#[cfg_attr(feature = \"extra\", macro_export)]",
        ] {
            fs::write(
                root.join("src/lib.rs"),
                format!("{attr} macro_rules! exported {{ () => {{}} }}"),
            )
            .unwrap();
            assert!(
                inventory(&root)
                    .unwrap_err()
                    .to_string()
                    .contains("exported macro")
            );
        }
        fs::write(root.join("src/lib.rs"), "macro_rules! private { () => {} }").unwrap();
        assert!(inventory(&root).unwrap().is_empty());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn reexport_groups_and_aliases_are_explicit() {
        let tree: syn::ItemUse = syn::parse_quote!(
            pub use source::{Thing as PublicThing, other};
        );
        let mut out = Vec::new();
        imports(&tree.tree, "", &mut out).unwrap();
        assert_eq!(
            out,
            vec![
                ("source::Thing".into(), "PublicThing".into()),
                ("source::other".into(), "other".into())
            ]
        );
        let glob: syn::ItemUse = syn::parse_quote!(
            pub use source::*;
        );
        assert!(imports(&glob.tree, "", &mut out).is_err());
    }
}
