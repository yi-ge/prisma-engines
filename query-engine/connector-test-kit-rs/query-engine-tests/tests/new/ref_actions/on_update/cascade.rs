use indoc::indoc;
use query_engine_tests::*;

#[test_suite(suite = "cascade_onU_1to1_req", schema(required))]
mod one2one_req {
    fn required() -> String {
        let schema = indoc! {
            r#"model Parent {
                #id(id, Int, @id)
                uniq  String @unique
                child Child?
            }

            model Child {
                #id(id, Int, @id)
                parent_uniq String
                parent    Parent @relation(fields: [parent_uniq], references: [uniq], onUpdate: Cascade)
            }"#
        };

        schema.to_owned()
    }

    /// Updating the parent updates the child as well.
    #[connector_test]
    async fn update_parent(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
            run_query!(&runner, r#"mutation { createOneParent(data: { id: 1, uniq: "1", child: { create: { id: 1 }}}) { id }}"#),
            @r###"{"data":{"createOneParent":{"id":1}}}"###
        );

        insta::assert_snapshot!(
            run_query!(&runner, r#"mutation { updateOneParent(where: { id: 1 }, data: { uniq: "1u" }) { uniq }}"#),
            @r###"{"data":{"updateOneParent":{"uniq":"1u"}}}"###
        );

        insta::assert_snapshot!(
            run_query!(&runner, "query { findManyParent { uniq child { parent_uniq } }}"),
            @r###"{"data":{"findManyParent":[{"uniq":"1u","child":{"parent_uniq":"1u"}}]}}"###
        );

        Ok(())
    }
}

#[test_suite(suite = "cascade_onU_1to1_opt", schema(optional))]
mod one2one_opt {
    fn optional() -> String {
        let schema = indoc! {
            r#"model Parent {
                #id(id, Int, @id)
                uniq  String @unique
                child Child?
            }

            model Child {
                #id(id, Int, @id)
                parent_uniq String?
                parent      Parent? @relation(fields: [parent_uniq], references: [uniq], onUpdate: Cascade)
            }"#
        };

        schema.to_owned()
    }

    /// Updating the parent updates the child as well.
    #[connector_test]
    async fn update_parent(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
            run_query!(&runner, r#"mutation { createOneParent(data: { id: 1, uniq: "1", child: { create: { id: 1 }}}) { id }}"#),
            @r###"{"data":{"createOneParent":{"id":1}}}"###
        );

        insta::assert_snapshot!(
            run_query!(&runner, r#"mutation { updateOneParent(where: { id: 1 }, data: { uniq: "1u" }) { uniq }}"#),
            @r###"{"data":{"updateOneParent":{"uniq":"1u"}}}"###
        );

        insta::assert_snapshot!(
            run_query!(&runner, "query { findManyParent { uniq child { parent_uniq } }}"),
            @r###"{"data":{"findManyParent":[{"uniq":"1u","child":{"parent_uniq":"1u"}}]}}"###
        );

        Ok(())
    }

    fn cascade_optional() -> String {
        let schema = indoc! {
            r#"model Parent {
                #id(id, Int, @id)
                uniq Int @unique
                childOpt Child?
              }
              
              model Child {
                #id(id, Int, @id)
                parentUniq  Int?
                parent    Parent? @relation(fields: [parentUniq], references: [uniq], onUpdate: Cascade)
                child2Opt Child2?
              }
              
              model Child2 {
                #id(id, Int, @id)
                childUniq Int?
                child   Child? @relation(fields: [childUniq], references: [parentUniq], onUpdate: Cascade)
              }
              "#
        };

        schema.to_owned()
    }

    #[connector_test(schema(cascade_optional))]
    async fn update_parent_cascade(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
            run_query!(&runner, r#"mutation {
                createOneParent(data: {
                    id: 1,
                    uniq: 1,
                    childOpt: {
                        create: {
                            id: 1,
                            child2Opt: { create: { id: 1 } }
                        }
                    }
                }) { id }
            }"#),
            @r###"{"data":{"createOneParent":{"id":1}}}"###
        );

        insta::assert_snapshot!(
          run_query!(runner, r#"mutation { updateOneParent(where: { id: 1 }, data: { uniq: 2 }) { uniq } }"#),
          @r###"{"data":{"updateOneParent":{"uniq":2}}}"###
        );

        insta::assert_snapshot!(
          run_query!(runner, r#"query { findManyParent { uniq childOpt { parentUniq child2Opt { childUniq } } } }"#),
          @r###"{"data":{"findManyParent":[{"uniq":2,"childOpt":{"parentUniq":2,"child2Opt":{"childUniq":2}}}]}}"###
        );

        Ok(())
    }
}

#[test_suite(suite = "cascade_onU_1toM_req", schema(required))]
mod one2many_req {
    fn required() -> String {
        let schema = indoc! {
            r#"model Parent {
                #id(id, Int, @id)
                uniq     String @unique
                children Child[]
            }

            model Child {
                #id(id, Int, @id)
                parent_uniq String
                parent    Parent @relation(fields: [parent_uniq], references: [uniq], onUpdate: Cascade)
            }"#
        };

        schema.to_owned()
    }

    /// Updating the parent updates the child as well.
    #[connector_test]
    async fn update_parent(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
            run_query!(&runner, r#"mutation { createOneParent(data: { id: 1, uniq: "1", children: { create: { id: 1 }}}) { id }}"#),
            @r###"{"data":{"createOneParent":{"id":1}}}"###
        );

        insta::assert_snapshot!(
            run_query!(&runner, r#"mutation { updateOneParent(where: { id: 1 }, data: { uniq: "1u" }) { uniq }}"#),
            @r###"{"data":{"updateOneParent":{"uniq":"1u"}}}"###
        );

        insta::assert_snapshot!(
            run_query!(&runner, "query { findManyParent { uniq children { parent_uniq } }}"),
            @r###"{"data":{"findManyParent":[{"uniq":"1u","children":[{"parent_uniq":"1u"}]}]}}"###
        );

        Ok(())
    }
}

#[test_suite(suite = "cascade_onU_1toM_opt", schema(optional))]
mod one2many_opt {
    fn optional() -> String {
        let schema = indoc! {
            r#"model Parent {
                #id(id, Int, @id)
                uniq     String  @unique
                children Child[]
            }

            model Child {
                #id(id, Int, @id)
                parent_uniq String?
                parent    Parent? @relation(fields: [parent_uniq], references: [uniq], onUpdate: Cascade)
            }"#
        };

        schema.to_owned()
    }

    /// Updating the parent updates the child as well.
    #[connector_test]
    async fn update_parent(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
            run_query!(&runner, r#"mutation { createOneParent(data: { id: 1, uniq: "1", children: { create: { id: 1 }}}) { id }}"#),
            @r###"{"data":{"createOneParent":{"id":1}}}"###
        );

        insta::assert_snapshot!(
            run_query!(&runner, r#"mutation { updateOneParent(where: { id: 1 }, data: { uniq: "1u" }) { uniq }}"#),
            @r###"{"data":{"updateOneParent":{"uniq":"1u"}}}"###
        );

        insta::assert_snapshot!(
            run_query!(&runner, "query { findManyParent { uniq children { parent_uniq } }}"),
            @r###"{"data":{"findManyParent":[{"uniq":"1u","children":[{"parent_uniq":"1u"}]}]}}"###
        );

        Ok(())
    }
}
