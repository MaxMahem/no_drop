mod test_macros;

use no_drop::into::{IntoNoDropDbg, IntoNoDropRls};
use no_drop::wrap::no_drop_empty::NoDropEmpty;
use no_drop::wrap::no_drop_msg::NoDropMsg;
use test_macros::{test_drop, test_forget, wrap_ctor};

mod no_drop_empty {
    use super::*;

    wrap_ctor!(wrap, NoDropEmpty::wrap(42), 42);
    wrap_ctor!(into_no_drop_dbg_trait, IntoNoDropDbg::no_drop(42), 42);
    wrap_ctor!(into_no_drop_rls_trait, IntoNoDropRls::no_drop(42), 42);
    wrap_ctor!(empty, NoDropEmpty::guard(), ());

    test_forget!(forget, NoDropEmpty::guard());

    test_drop!(wrap_drop, NoDropEmpty::wrap(42), panic: "Value was dropped without being unwrapped");
    test_drop!(default_drop, NoDropEmpty::<()>::default(), panic: "Value was dropped without being unwrapped");
}

mod no_drop_msg {
    use super::*;

    test_drop!(no_drop_msg_panics, NoDropMsg::wrap(42, "custom panic message"), panic: "custom panic message");

    wrap_ctor!(wrap_static_str, NoDropMsg::wrap(42, "custom message"), 42);
    wrap_ctor!(wrap_string, NoDropMsg::wrap(42, String::from("owned message")), 42);

    wrap_ctor!(into_no_drop_msg_dbg_trait, IntoNoDropDbg::expect_no_drop(42, "msg"), 42);
    wrap_ctor!(into_no_drop_msg_rls_trait, IntoNoDropRls::expect_no_drop(42, "msg"), 42);

    wrap_ctor!(guard_static_str, NoDropMsg::guard("expected message"), ());
    wrap_ctor!(guard_string, NoDropMsg::guard(String::from("owned expected message")), ());
    wrap_ctor!(default, NoDropMsg::<()>::default(), ());

    test_forget!(forget, NoDropMsg::wrap(42, "custom message"));

    test_drop!(guard_drop, NoDropMsg::guard("test"), panic: "test");
    test_drop!(default_drop, NoDropMsg::<()>::default(), panic: "Value was dropped without being unwrapped");
}

mod no_drop_no_op_empty {
    use super::*;
    use no_drop::markers::NoMsg;
    use no_drop::wrap::no_drop_no_op::NoDropNoOp;

    wrap_ctor!(wrap, NoDropNoOp::<'static, _, NoMsg>::wrap(42), 42);
    wrap_ctor!(empty, NoDropNoOp::<'static, (), NoMsg>::guard(), ());

    test_forget!(forget, NoDropNoOp::<'static, _, NoMsg>::wrap(42));

    test_drop!(drop_no_panic, NoDropNoOp::<'static, _, NoMsg>::wrap(42), no_panic);
    test_drop!(default_drop_no_panic, NoDropNoOp::<'static, (), NoMsg>::default(), no_panic);
}

mod no_drop_no_op_msg {
    use super::*;
    use no_drop::markers::Msg;
    use no_drop::wrap::no_drop_no_op::NoDropNoOp;

    wrap_ctor!(wrap, NoDropNoOp::<'_, _, Msg>::wrap(42, "message"), 42);
    wrap_ctor!(guard, NoDropNoOp::<'_, (), Msg>::guard("expected message"), ());

    test_forget!(forget, NoDropNoOp::<'_, _, Msg>::wrap(42, "message"));

    test_drop!(wrap_drop_no_panic, NoDropNoOp::<'_, _, Msg>::wrap(42, "should not panic"), no_panic);
    test_drop!(guard_drop_no_panic, NoDropNoOp::<'_, _, Msg>::guard("should not panic"), no_panic);
    test_drop!(default_drop_no_panic, NoDropNoOp::<'_, (), Msg>::default(), no_panic);
}
