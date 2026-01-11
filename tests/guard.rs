mod test_macros;

use GuardState::{Armed, Disarmed};
use no_drop::guards::GuardState;
use no_drop::guards::drop_guard_empty::DropGuardEmpty;
use no_drop::guards::drop_guard_msg::DropGuardMsg;
use no_drop::guards::drop_guard_no_op::DropGuardNoOp;

use no_drop::markers::{Msg, NoMsg};
use no_drop::wrap::{NoDropEmpty, NoDropMsg, NoDropNoOpEmpty, NoDropNoOpMsg};
use test_macros::{guard_ctor, set_msg, transition, try_from};

mod drop_guard_msg {
    use super::*;

    guard_ctor!(new_armed, DropGuardMsg::new_armed("custom panic message"), armed, "custom panic message");
    guard_ctor!(new_disarmed, DropGuardMsg::new_disarmed("custom message"), disarmed);
    guard_ctor!(from_no_drop, DropGuardMsg::from(NoDropMsg::guard("custom")), armed, "custom");

    try_from!(try_from_armed, DropGuardMsg::new_armed("message"), NoDropMsg, armed);
    try_from!(try_from_disarmed, DropGuardMsg::new_disarmed("message"), NoDropMsg, disarmed);

    transition!(arm_when_disarmed, DropGuardMsg::new_disarmed("test"), arm, true, armed, "test");
    transition!(arm_when_armed, DropGuardMsg::new_armed("test"), arm, false, armed, "test");
    transition!(disarm_when_armed, DropGuardMsg::new_armed("test"), disarm, true, disarmed);
    transition!(disarm_when_disarmed, DropGuardMsg::new_disarmed("test"), disarm, false, disarmed);

    transition!(toggle_armed_to_disarmed, DropGuardMsg::new_armed("test"), toggle, Disarmed, disarmed);
    transition!(toggle_disarmed_to_armed, DropGuardMsg::new_disarmed("test"), toggle, Armed, armed, "test");

    set_msg!(set_msg_armed, DropGuardMsg::new_armed("old"), "new message");
    set_msg!(set_msg_disarmed, DropGuardMsg::new_disarmed("old"), "new message", no_panic);
}

mod drop_guard_empty {
    use super::*;

    guard_ctor!(new_armed, DropGuardEmpty::new_armed(), armed);
    guard_ctor!(new_disarmed, DropGuardEmpty::new_disarmed(), disarmed);
    guard_ctor!(from_no_drop, DropGuardEmpty::from(NoDropEmpty::guard()), armed);

    try_from!(try_from_armed, DropGuardEmpty::new_armed(), NoDropEmpty, armed);
    try_from!(try_from_disarmed, DropGuardEmpty::new_disarmed(), NoDropEmpty, disarmed);

    transition!(arm_when_disarmed, DropGuardEmpty::new_disarmed(), arm, true, armed);
    transition!(arm_when_armed, DropGuardEmpty::new_armed(), arm, false, armed);
    transition!(disarm_when_armed, DropGuardEmpty::new_armed(), disarm, true, disarmed);
    transition!(disarm_when_disarmed, DropGuardEmpty::new_disarmed(), disarm, false, disarmed);

    transition!(toggle_armed_to_disarmed, DropGuardEmpty::new_armed(), toggle, Disarmed, disarmed);
    transition!(toggle_disarmed_to_armed, DropGuardEmpty::new_disarmed(), toggle, Armed, armed);
}

mod drop_guard_no_op_empty {
    use super::*;

    guard_ctor!(new_armed, DropGuardNoOp::<NoMsg>::new_armed(), armed_no_panic);
    guard_ctor!(new_disarmed, DropGuardNoOp::<NoMsg>::new_disarmed(), disarmed);
    guard_ctor!(from, DropGuardNoOp::<NoMsg>::from(NoDropNoOpEmpty::guard()), armed_no_panic);

    try_from!(try_from_armed, DropGuardNoOp::<NoMsg>::new_armed(), NoDropNoOpEmpty, armed);
    try_from!(try_from_disarmed, DropGuardNoOp::<NoMsg>::new_disarmed(), NoDropNoOpEmpty, disarmed);

    transition!(arm_when_disarmed, DropGuardNoOp::<NoMsg>::new_disarmed(), arm, true, armed_no_panic);
    transition!(arm_when_armed, DropGuardNoOp::<NoMsg>::new_armed(), arm, false, armed_no_panic);
    transition!(disarm_when_armed, DropGuardNoOp::<NoMsg>::new_armed(), disarm, true, disarmed);
    transition!(disarm_when_disarmed, DropGuardNoOp::<NoMsg>::new_disarmed(), disarm, false, disarmed);

    transition!(toggle_armed_to_disarmed, DropGuardNoOp::<NoMsg>::new_armed(), toggle, Disarmed, disarmed);
    transition!(toggle_disarmed_to_armed, DropGuardNoOp::<NoMsg>::new_disarmed(), toggle, Armed, armed_no_panic);

    // Note: DropGuardNoOp<NoMsg> doesn't have set_msg method
}

mod drop_guard_no_op_msg {
    use super::*;

    guard_ctor!(new_armed, DropGuardNoOp::<Msg>::new_armed("message"), armed_no_panic);
    guard_ctor!(new_disarmed, DropGuardNoOp::<Msg>::new_disarmed("message"), disarmed);
    guard_ctor!(from, DropGuardNoOp::<Msg>::from(NoDropNoOpMsg::guard("message")), armed_no_panic);

    try_from!(try_from_armed, DropGuardNoOp::<Msg>::new_armed("msg"), NoDropNoOpMsg, armed);
    try_from!(try_from_disarmed, DropGuardNoOp::<Msg>::new_disarmed("msg"), NoDropNoOpMsg, disarmed);

    transition!(arm_when_disarmed, DropGuardNoOp::<Msg>::new_disarmed("test"), arm, true, armed_no_panic);
    transition!(arm_when_armed, DropGuardNoOp::<Msg>::new_armed("test"), arm, false, armed_no_panic);
    transition!(disarm_when_armed, DropGuardNoOp::<Msg>::new_armed("test"), disarm, true, disarmed);
    transition!(disarm_when_disarmed, DropGuardNoOp::<Msg>::new_disarmed("test"), disarm, false, disarmed);

    transition!(toggle_armed_to_disarmed, DropGuardNoOp::<Msg>::new_armed("msg"), toggle, Disarmed, disarmed);
    transition!(toggle_disarmed_to_armed, DropGuardNoOp::<Msg>::new_disarmed("msg"), toggle, Armed, armed_no_panic);

    set_msg!(set_msg_armed, DropGuardNoOp::<Msg>::new_armed("old"), "new", no_panic);
    set_msg!(set_msg_disarmed, DropGuardNoOp::<Msg>::new_disarmed("old"), "new", no_panic);
}
