use pgrx::prelude::*;
use slug::slugify;

pgrx::pg_module_magic!();

#[pg_extern]
fn pg_slugify(inp: String) -> String {
    slugify(inp)
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_pg_slugify() {
        assert_eq!(
            String::from("my-test-string-1-1"),
            crate::pg_slugify(String::from("My Test String!!!1!1"))
        );
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
