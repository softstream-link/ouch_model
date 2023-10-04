use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use derive_more::TryInto;
use soupbintcp_model::prelude::{
    CltSoupBinTcpMsg, SoupBinTcpMsg, SoupBinTcpPayload, SvcSoupBinTcpMsg,
    SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG,
};

use crate::prelude::*;

use super::svc::order_aiq_canceled::OrderAiqCanceled;

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Debug, TryInto)]
#[try_into(owned, ref, ref_mut)]
#[byteserde(peek(0, 1))]
pub enum CltOuchPayload {
    #[byteserde(eq(PacketTypeEnterOrder::as_slice()))]
    Enter(EnterOrder),
    #[byteserde(eq(PacketTypeReplaceOrder::as_slice()))]
    Replace(ReplaceOrder),
    #[byteserde(eq(PacketTypeCancelOrder::as_slice()))]
    Cancel(CancelOrder),
    #[byteserde(eq(PacketTypeModifyOrder::as_slice()))]
    Modify(ModifyOrder),
    #[byteserde(eq(PacketTypeAccountQueryRequest::as_slice()))]
    AccQry(AccountQueryRequest),
}
impl SoupBinTcpPayload<CltOuchPayload> for CltOuchPayload {}

pub const SVC_OUCH_MAX_PLD_SIZE: usize = 72; // TODO revise Options fields and remeasure
pub const SVC_OUCH_MAX_FRAME_SIZE: usize =
    SVC_OUCH_MAX_PLD_SIZE + SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG;

pub const CLT_OUCH_MAX_PLD_SIZE: usize = 51; // TODO revise Options fields and remeasure
pub const CLT_OUCH_MAX_FRAME_SIZE: usize =
    CLT_OUCH_MAX_PLD_SIZE + SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG;
/// Both [ReplaceOrder] & [OrderReplaced] are serialized as b'U' hence it is impossible to distinguish deserialization type unless they are in two different enums.
#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Debug, TryInto)]
#[try_into(owned, ref, ref_mut)]
#[byteserde(peek(0, 1))]
pub enum SvcOuchPayload {
    #[byteserde(eq(PacketTypeOrderAccepted::as_slice()))]
    Accepted(OrderAccepted),
    #[byteserde(eq(PacketTypeOrderExecuted::as_slice()))]
    Executed(OrderExecuted),
    #[byteserde(eq(PacketTypeOrderReplaced::as_slice()))]
    Replaced(OrderReplaced),
    #[byteserde(eq(PacketTypeOrderCanceled::as_slice()))]
    Canceled(OrderCanceled),
    #[byteserde(eq(PacketTypeOrderRejected::as_slice()))]
    Rejected(OrderRejected),
    #[byteserde(eq(PacketTypeOrderModified::as_slice()))]
    Modified(OrderModified),
    #[byteserde(eq(PacketTypeOrderRestated::as_slice()))]
    Restated(OrderRestated),

    #[byteserde(eq(PacketTypeCancelPending::as_slice()))]
    CanPending(CancelPending),
    #[byteserde(eq(PacketTypeCancelReject::as_slice()))]
    CanReject(CancelReject),
    #[byteserde(eq(PacketTypeOrderAiqCanceled::as_slice()))]
    AiqCanceled(OrderAiqCanceled),

    #[byteserde(eq(PacketTypeBrokenTrade::as_slice()))]
    BrokenTrade(BrokenTrade),    
    #[byteserde(eq(PacketTypePriorityUpdate::as_slice()))]
    PrioUpdate(PriorityUpdate),
    #[byteserde(eq(PacketTypeAccountQueryResponse::as_slice()))]
    AccQryRes(AccountQueryResponse),
    #[byteserde(eq(PacketTypeSystemEvent::as_slice()))]
    SysEvt(SystemEvent),
}

impl SoupBinTcpPayload<SvcOuchPayload> for SvcOuchPayload {}

pub type CltOuchMsg = CltSoupBinTcpMsg<CltOuchPayload>;
pub type SvcOuchMsg = SvcSoupBinTcpMsg<SvcOuchPayload>;

pub type OuchMsg = SoupBinTcpMsg<CltOuchPayload, SvcOuchPayload>;

pub use from_clt_pld::*;
mod from_clt_pld {
    use super::*;
    impl From<EnterOrder> for CltOuchMsg {
        #[inline(always)]
        fn from(payload: EnterOrder) -> Self {
            CltOuchMsg::udata(CltOuchPayload::Enter(payload))
        }
    }
    impl From<EnterOrder> for OuchMsg {
        #[inline(always)]
        fn from(payload: EnterOrder) -> Self {
            OuchMsg::Clt(payload.into())
        }
    }
    impl From<ReplaceOrder> for CltOuchMsg {
        #[inline(always)]
        fn from(payload: ReplaceOrder) -> Self {
            CltOuchMsg::udata(CltOuchPayload::Replace(payload))
        }
    }
    impl From<ReplaceOrder> for OuchMsg {
        #[inline(always)]
        fn from(payload: ReplaceOrder) -> Self {
            OuchMsg::Clt(payload.into())
        }
    }
    impl From<CancelOrder> for CltOuchMsg {
        #[inline(always)]
        fn from(payload: CancelOrder) -> Self {
            CltOuchMsg::udata(CltOuchPayload::Cancel(payload))
        }
    }
    impl From<CancelOrder> for OuchMsg {
        #[inline(always)]
        fn from(payload: CancelOrder) -> Self {
            OuchMsg::Clt(payload.into())
        }
    }
    impl From<ModifyOrder> for CltOuchMsg {
        #[inline(always)]
        fn from(payload: ModifyOrder) -> Self {
            CltOuchMsg::udata(CltOuchPayload::Modify(payload))
        }
    }
    impl From<ModifyOrder> for OuchMsg {
        #[inline(always)]
        fn from(payload: ModifyOrder) -> Self {
            OuchMsg::Clt(payload.into())
        }
    }
    impl From<AccountQueryRequest> for CltOuchMsg {
        #[inline(always)]
        fn from(payload: AccountQueryRequest) -> Self {
            CltOuchMsg::udata(CltOuchPayload::AccQry(payload))
        }
    }
    impl From<AccountQueryRequest> for OuchMsg {
        #[inline(always)]
        fn from(payload: AccountQueryRequest) -> Self {
            OuchMsg::Clt(payload.into())
        }
    }
}

pub use from_svc_pld::*;
mod from_svc_pld {
    use super::*;
    impl From<OrderAccepted> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderAccepted) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Accepted(payload))
        }
    }
    impl From<OrderAccepted> for OuchMsg {
        #[inline(always)]
        fn from(payload: OrderAccepted) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderExecuted> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderExecuted) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Executed(payload))
        }
    }
    impl From<OrderExecuted> for OuchMsg {
        #[inline(always)]
        fn from(payload: OrderExecuted) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderReplaced> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderReplaced) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Replaced(payload))
        }
    }
    impl From<OrderReplaced> for OuchMsg {
        #[inline(always)]
        fn from(payload: OrderReplaced) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderCanceled> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderCanceled) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Canceled(payload))
        }
    }
    impl From<OrderCanceled> for OuchMsg {
        #[inline(always)]
        fn from(payload: OrderCanceled) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderRejected> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderRejected) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Rejected(payload))
        }
    }
    impl From<OrderRejected> for OuchMsg {
        #[inline(always)]
        fn from(payload: OrderRejected) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderModified> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderModified) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Modified(payload))
        }
    }
    impl From<OrderModified> for OuchMsg {
        #[inline(always)]
        fn from(payload: OrderModified) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderRestated> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderRestated) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::Restated(payload))
        }
    }
    impl From<OrderRestated> for OuchMsg {
        #[inline(always)]
        fn from(payload: OrderRestated) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
    impl From<CancelPending> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: CancelPending) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::CanPending(payload))
        }
    }
    impl From<CancelPending> for OuchMsg {
        #[inline(always)]
        fn from(payload: CancelPending) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
    impl From<CancelReject> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: CancelReject) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::CanReject(payload))
        }
    }
    impl From<CancelReject> for OuchMsg {
        #[inline(always)]
        fn from(payload: CancelReject) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
    impl From<OrderAiqCanceled> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: OrderAiqCanceled) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::AiqCanceled(payload))
        }
    }
    impl From<OrderAiqCanceled> for OuchMsg {
        #[inline(always)]
        fn from(payload: OrderAiqCanceled) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
    impl From<BrokenTrade> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: BrokenTrade) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::BrokenTrade(payload))
        }
    }
    impl From<BrokenTrade> for OuchMsg {
        #[inline(always)]
        fn from(payload: BrokenTrade) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
    impl From<PriorityUpdate> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: PriorityUpdate) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::PrioUpdate(payload))
        }
    }
    impl From<PriorityUpdate> for OuchMsg {
        #[inline(always)]
        fn from(payload: PriorityUpdate) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
    impl From<AccountQueryResponse> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: AccountQueryResponse) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::AccQryRes(payload))
        }
    }
    impl From<AccountQueryResponse> for OuchMsg {
        #[inline(always)]
        fn from(payload: AccountQueryResponse) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
    impl From<SystemEvent> for SvcOuchMsg {
        #[inline(always)]
        fn from(payload: SystemEvent) -> Self {
            SvcOuchMsg::udata(SvcOuchPayload::SysEvt(payload))
        }
    }
    impl From<SystemEvent> for OuchMsg {
        #[inline(always)]
        fn from(payload: SystemEvent) -> Self {
            OuchMsg::Svc(payload.into())
        }
    }
}

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {

    use crate::unittest::setup;
    use crate::{
        model::ouch::{CLT_OUCH_MAX_PLD_SIZE, SVC_OUCH_MAX_PLD_SIZE},
        prelude::*,
    };
    use byteserde::prelude::*;
    use log::info;

    // TODO max message length needed to optimize stack serialization assume 512 bytes for now
    #[test]
    fn test_ouch_with_envelope_ser_des() {
        setup::log::configure();

        let enter_ord = EnterOrder::default();
        let replace_ord = ReplaceOrder::from(&enter_ord);
        let cancel_ord = CancelOrder::from(&enter_ord);

        let ord_accepted = OrderAccepted::from(&enter_ord);
        let ord_replaced = OrderReplaced::from((&enter_ord, &replace_ord));
        let ord_canceled = OrderCanceled::from((&enter_ord, &cancel_ord));
        let ord_aqi_canceled = OrderAiqCanceled::from(&enter_ord);
        let ord_executed = OrderExecuted::from(&enter_ord);
        let brkn_trade = BrokenTrade::from(&enter_ord);
        let ord_rejected = OrderRejected::from((&enter_ord, RejectReason::halted()));
        let can_pending = CancelPending::from(&enter_ord);
        let can_reject = CancelReject::from(&enter_ord);
        let prio_update = PriorityUpdate::from((&enter_ord, OrderReferenceNumber::default()));
        let ord_modified = OrderModified::from((&enter_ord, Side::buy()));
        let ord_rstd = OrderRestated::from((&enter_ord, RestatedReason::refresh_of_display()));

        let msg_inp = vec![
            enter_ord.into(),
            replace_ord.into(),
            cancel_ord.into(),
            ModifyOrder::default().into(),
            AccountQueryRequest::default().into(),
            ord_accepted.into(),
            ord_executed.into(),
            ord_replaced.into(),
            ord_canceled.into(),
            ord_rejected.into(),
            ord_modified.into(),
            ord_rstd.into(),
            can_pending.into(),
            can_reject.into(),
            ord_aqi_canceled.into(),
            brkn_trade.into(),
            prio_update.into(),
            AccountQueryResponse::default().into(),
            SystemEvent::default().into(),
        ];
        let mut ser = ByteSerializerStack::<1024>::default();
        for msg in msg_inp.iter() {
            match msg {
                OuchMsg::Clt(msg_inp_inb) => {
                    info!("msg_inp_inb: {:?}", msg_inp_inb);
                    let _ = ser.serialize(msg_inp_inb).unwrap();
                }
                OuchMsg::Svc(msg_inp_oub) => {
                    info!("msg_inp_oub: {:?}", msg_inp_oub);
                    let _ = ser.serialize(msg_inp_oub).unwrap();
                }
            }
        }
        let mut des = ByteDeserializerSlice::new(ser.as_slice());

        for ouch in msg_inp.iter() {
            match ouch {
                OuchMsg::Clt(msg_inp_inb) => {
                    let msg_out_inb = des.deserialize::<CltOuchMsg>().unwrap();
                    info!("msg_out_inb: {:?}", msg_out_inb);
                    assert_eq!(msg_inp_inb, &msg_out_inb);
                }
                OuchMsg::Svc(msg_inp_oub) => {
                    let msg_out_oub = des.deserialize::<SvcOuchMsg>().unwrap();
                    info!("msg_out_oub: {:?}", msg_out_oub);
                    assert_eq!(msg_inp_oub, &msg_out_oub);
                }
            }
        }
        assert!(des.is_empty());
    }

    #[test]
    fn test_ouch5_max_size() {
        setup::log::configure();

        let enter_ord = EnterOrder::default();
        let replace_ord = ReplaceOrder::from(&enter_ord);
        let cancel_ord = CancelOrder::from(&enter_ord);

        let ord_accepted = OrderAccepted::from(&enter_ord);
        let ord_replaced = OrderReplaced::from((&enter_ord, &replace_ord));
        let ord_canceled = OrderCanceled::from((&enter_ord, &cancel_ord));
        let ord_aqi_canceled = OrderAiqCanceled::from(&enter_ord);
        let ord_executed = OrderExecuted::from(&enter_ord);
        let brkn_trade = BrokenTrade::from(&enter_ord);
        let ord_rejected = OrderRejected::from((&enter_ord, RejectReason::halted()));
        let can_pending = CancelPending::from(&enter_ord);
        let can_reject = CancelReject::from(&enter_ord);
        let prio_update = PriorityUpdate::from((&enter_ord, OrderReferenceNumber::default()));
        let ord_modified = OrderModified::from((&enter_ord, Side::buy()));
        let ord_rstd = OrderRestated::from((&enter_ord, RestatedReason::refresh_of_display()));
        let inb = vec![
            CltOuchPayload::Enter(enter_ord),
            CltOuchPayload::Replace(replace_ord),
            CltOuchPayload::Cancel(cancel_ord),
            CltOuchPayload::Modify(ModifyOrder::default()),
            CltOuchPayload::AccQry(AccountQueryRequest::default()),
        ];
        let oub = vec![
            SvcOuchPayload::SysEvt(SystemEvent::default()),
            SvcOuchPayload::Accepted(ord_accepted),
            SvcOuchPayload::Replaced(ord_replaced),
            SvcOuchPayload::Canceled(ord_canceled),
            SvcOuchPayload::AiqCanceled(ord_aqi_canceled),
            SvcOuchPayload::Executed(ord_executed),
            SvcOuchPayload::BrokenTrade(brkn_trade),
            SvcOuchPayload::Rejected(ord_rejected),
            SvcOuchPayload::CanPending(can_pending),
            SvcOuchPayload::CanReject(can_reject),
            SvcOuchPayload::PrioUpdate(prio_update),
            SvcOuchPayload::Modified(ord_modified),
            SvcOuchPayload::Restated(ord_rstd),
            SvcOuchPayload::AccQryRes(AccountQueryResponse::default()),
        ];

        let inb = inb
            .into_iter()
            .map(|msg| (msg.byte_len(), msg))
            .collect::<Vec<_>>();
        // for (len, msg) in inb.iter() {
        //     info!("len: {:>3}, msg: Ouch5Inb::{:?}", len,  msg);
        // }
        let max_frame_size_clt = inb.iter().map(|(len, _)| *len).max().unwrap();
        info!("max_frame_size_clt: {}", max_frame_size_clt);
        assert_eq!(max_frame_size_clt, CLT_OUCH_MAX_PLD_SIZE);

        let oub = oub
            .into_iter()
            .map(|msg| (msg.byte_len(), msg))
            .collect::<Vec<_>>();
        // for (len, msg) in oub.iter() {
        //     info!("len: {:>3}, msg: Ouch5Oub::{:?}", len, msg);
        // }
        let max_frame_size_svc = oub.iter().map(|(len, _)| *len).max().unwrap();
        info!("max_frame_size_svc: {}", max_frame_size_svc);
        assert_eq!(max_frame_size_svc, SVC_OUCH_MAX_PLD_SIZE);
    }
}
