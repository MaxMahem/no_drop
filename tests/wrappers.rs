mod test_macros;

use no_drop::into::{IntoNoDropDbg, IntoNoDropRls};
use no_drop::wrap::no_drop_empty::NoDropEmpty;
use no_drop::wrap::no_drop_msg::NoDropMsg;
use test_macros::{test_clone, test_ctor, test_forget};

mod no_drop_empty {
    use super::*;

    #[test]
    #[should_panic(expected = "Value was dropped without being unwrapped")]
    fn no_drop_empty_panics() {
        let wrapper = NoDropEmpty::wrap(42);
        drop(wrapper);
    }

    test_ctor!(wrap, NoDropEmpty::wrap, (42), 42);
    test_ctor!(into_no_drop_dbg_trait, IntoNoDropDbg::no_drop, (42), 42);
    test_ctor!(into_no_drop_rls_trait, IntoNoDropRls::no_drop, (42), 42);
    test_ctor!(new, NoDropEmpty::new, (), ());
    test_ctor!(default, NoDropEmpty::default, (), ());

    test_clone!(clone, NoDropEmpty, NoDropEmpty::new, ());

    test_forget!(forget, NoDropEmpty::new, ());
}

mod no_drop_msg {
    use super::*;

    #[test]
    #[should_panic(expected = "custom panic message")]
    fn no_drop_msg_panics() {
        let wrapper = NoDropMsg::wrap(42, "custom panic message");
        drop(wrapper);
    }

    test_ctor!(wrap_static_str, NoDropMsg::wrap, (42, "custom message"), 42);
    test_ctor!(wrap_string, NoDropMsg::wrap, (42, String::from("owned message")), 42);

    test_ctor!(into_no_drop_msg_dbg_trait, IntoNoDropDbg::expect_no_drop, (42, "msg"), 42);
    test_ctor!(into_no_drop_msg_rls_trait, IntoNoDropRls::expect_no_drop, (42, "msg"), 42);

    test_ctor!(guard_static_str, NoDropMsg::guard, ("expected message"), ());
    test_ctor!(guard_string, NoDropMsg::guard, (String::from("owned expected message")), ());

    test_clone!(clone, NoDropMsg, NoDropMsg::guard, ("custom message"));

    test_forget!(forget, NoDropMsg::wrap, (42, "custom message"));

    #[test]
    fn no_drop_msg_borrowed() {
        let msg = String::from("borrowed message");
        let wrapper = NoDropMsg::wrap(42, msg.as_str());
        assert_eq!(wrapper.unwrap(), 42);
    }

    #[test]
    #[should_panic(expected = "unit value must be consumed")]
    fn no_drop_msg_expect_panics() {
        let wrapper = NoDropMsg::guard("unit value must be consumed");
        drop(wrapper);
    }
}

mod no_drop_no_op_empty {
    use super::*;
    use no_drop::wrap::no_drop_no_op_empty::NoDropNoOp;

    test_ctor!(wrap, NoDropNoOp::wrap, (42), 42);
    test_ctor!(new, NoDropNoOp::<()>::new, (), ());
    test_ctor!(default, NoDropNoOp::<()>::default, (), ());

    test_forget!(forget, NoDropNoOp::wrap, (42));

    test_clone!(clone, NoDropNoOp<()>, NoDropNoOp::new, ());

    #[test]
    fn drop_no_panic() {
        let wrapper = NoDropNoOp::wrap(42);
        drop(wrapper); // No panic
    }
}

mod no_drop_no_op_msg {
    use super::*;
    use no_drop::wrap::no_drop_no_op_msg::NoDropNoOp;

    test_ctor!(wrap, NoDropNoOp::wrap, (42, "message"), 42);
    test_ctor!(guard, NoDropNoOp::<()>::guard, ("expected message"), ());

    test_forget!(forget, NoDropNoOp::wrap, (42, "message"));

    test_clone!(clone, NoDropNoOp<()>, NoDropNoOp::guard, ("message"));

    #[test]
    fn passthrough_msg_drop_no_panic() {
        let wrapper = NoDropNoOp::wrap(42, "should not panic");
        drop(wrapper); // No panic
    }

    #[test]
    fn passthrough_msg_borrowed() {
        let msg = String::from("test");
        let wrapper = NoDropNoOp::wrap(42, msg.as_str());
        drop(wrapper);
    }
}
