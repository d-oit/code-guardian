use proptest::prelude::*;

prop_compose! {
    fn arbitrary_code_content()(
        lines in prop::collection::vec(
            prop::string::string_regex(r"[a-zA-Z0-9\s._/\\-]*").unwrap(),
            1..20
        )
    ) -> String {
        lines.join("\n")
    }
}
