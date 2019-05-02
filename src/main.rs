use juniper::*;

#[derive(GraphQLObject)]
struct User {
    id: String,
}

struct Context;

impl juniper::Context for Context {}

struct Query;

graphql_object!(Query: Context |&self| {
    field user(&executor, id: Option<String>) -> FieldResult<Option<User>> {
        // Doesn't panic if you remove this line
        executor.look_ahead();

        if let Some(id) = id {
            Ok(Some(User { id }))
        } else {
            Ok(None)
        }
    }
});

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;

fn main() {}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn query_with_null() {
        let ctx = Context;

        let mut vars = Variables::new();
        vars.insert("id".to_string(), InputValue::Null);

        let (res, _errors) = juniper::execute(
            r#"
            query Foo($id: String) {
                user(id: $id) {
                    id
                }
            }
            "#,
            None,
            &Schema::new(Query, EmptyMutation::new()),
            &vars,
            &ctx,
        )
        .unwrap();

        dbg!(res);
    }

    #[test]
    fn query_without_setting_variable() {
        let ctx = Context;

        let vars = Variables::new();

        let (res, _errors) = juniper::execute(
            r#"
            query Foo($id: String) {
                user(id: $id) {
                    id
                }
            }
            "#,
            None,
            &Schema::new(Query, EmptyMutation::new()),
            &vars,
            &ctx,
        )
        .unwrap();

        dbg!(res);
    }
}
