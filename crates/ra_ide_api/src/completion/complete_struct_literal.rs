use hir::{Substs, Ty};

use crate::completion::{CompletionContext, Completions};

/// Complete fields in fields literals.
pub(super) fn complete_struct_literal(acc: &mut Completions, ctx: &CompletionContext) {
    let (ty, variant) = match ctx.struct_lit_syntax.and_then(|it| {
        Some((ctx.analyzer.type_of(ctx.db, it.into())?, ctx.analyzer.resolve_variant(it)?))
    }) {
        Some(it) => it,
        _ => return,
    };

    let ty_substs = match ty {
        Ty::Apply(it) => it.parameters,
        _ => Substs::empty(),
    };

    for field in variant.fields(ctx.db) {
        acc.add_field(ctx, field, &ty_substs);
    }
}

#[cfg(test)]
mod tests {
    use crate::completion::{do_completion, CompletionItem, CompletionKind};
    use insta::assert_debug_snapshot_matches;

    fn complete(code: &str) -> Vec<CompletionItem> {
        do_completion(code, CompletionKind::Reference)
    }

    #[test]
    fn test_struct_literal_field() {
        let completions = complete(
            r"
            struct A { the_field: u32 }
            fn foo() {
               A { the<|> }
            }
            ",
        );
        assert_debug_snapshot_matches!(completions, @r###"
       ⋮[
       ⋮    CompletionItem {
       ⋮        label: "the_field",
       ⋮        source_range: [83; 86),
       ⋮        delete: [83; 86),
       ⋮        insert: "the_field",
       ⋮        kind: Field,
       ⋮        detail: "u32",
       ⋮    },
       ⋮]
        "###);
    }

    #[test]
    fn test_struct_literal_enum_variant() {
        let completions = complete(
            r"
            enum E {
                A { a: u32 }
            }
            fn foo() {
                let _ = E::A { <|> }
            }
            ",
        );
        assert_debug_snapshot_matches!(completions, @r###"
       ⋮[
       ⋮    CompletionItem {
       ⋮        label: "a",
       ⋮        source_range: [119; 119),
       ⋮        delete: [119; 119),
       ⋮        insert: "a",
       ⋮        kind: Field,
       ⋮        detail: "u32",
       ⋮    },
       ⋮]
        "###);
    }

    #[test]
    fn test_struct_literal_two_structs() {
        let completions = complete(
            r"
            struct A { a: u32 }
            struct B { b: u32 }

            fn foo() {
               let _: A = B { <|> }
            }
            ",
        );
        assert_debug_snapshot_matches!(completions, @r###"
       ⋮[
       ⋮    CompletionItem {
       ⋮        label: "b",
       ⋮        source_range: [119; 119),
       ⋮        delete: [119; 119),
       ⋮        insert: "b",
       ⋮        kind: Field,
       ⋮        detail: "u32",
       ⋮    },
       ⋮]
        "###);
    }

    #[test]
    fn test_struct_literal_generic_struct() {
        let completions = complete(
            r"
            struct A<T> { a: T }

            fn foo() {
               let _: A<u32> = A { <|> }
            }
            ",
        );
        assert_debug_snapshot_matches!(completions, @r###"
       ⋮[
       ⋮    CompletionItem {
       ⋮        label: "a",
       ⋮        source_range: [93; 93),
       ⋮        delete: [93; 93),
       ⋮        insert: "a",
       ⋮        kind: Field,
       ⋮        detail: "u32",
       ⋮    },
       ⋮]
        "###);
    }
}
