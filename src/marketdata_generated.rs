// automatically generated by the FlatBuffers compiler, do not modify


#![allow(dead_code)]
#![allow(unused_imports)]
extern crate flatbuffers;

pub mod md_shootout {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MessageBody {
  NONE = 0,
  Trade = 1,
  LevelUpdate = 2,

}

const ENUM_MIN_MESSAGE_BODY: u8 = 0;
const ENUM_MAX_MESSAGE_BODY: u8 = 2;

impl<'a> flatbuffers::Follow<'a> for MessageBody {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for MessageBody {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const MessageBody;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const MessageBody;
    unsafe { *p }
  }
}

impl flatbuffers::Push for MessageBody {
    type Output = MessageBody;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<MessageBody>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_MESSAGE_BODY:[MessageBody; 3] = [
  MessageBody::NONE,
  MessageBody::Trade,
  MessageBody::LevelUpdate
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_MESSAGE_BODY:[&'static str; 3] = [
    "NONE",
    "Trade",
    "LevelUpdate"
];

pub fn enum_name_message_body(e: MessageBody) -> &'static str {
  let index: usize = e as usize;
  ENUM_NAMES_MESSAGE_BODY[index]
}

pub struct MessageBodyUnionTableOffset {}
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Side {
  Buy = 0,
  Sell = 1,

}

const ENUM_MIN_SIDE: u8 = 0;
const ENUM_MAX_SIDE: u8 = 1;

impl<'a> flatbuffers::Follow<'a> for Side {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for Side {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const Side;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const Side;
    unsafe { *p }
  }
}

impl flatbuffers::Push for Side {
    type Output = Side;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<Side>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_SIDE:[Side; 2] = [
  Side::Buy,
  Side::Sell
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_SIDE:[&'static str; 2] = [
    "Buy",
    "Sell"
];

pub fn enum_name_side(e: Side) -> &'static str {
  let index: usize = e as usize;
  ENUM_NAMES_SIDE[index]
}

pub enum TradeOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Trade<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Trade<'a> {
    type Inner = Trade<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Trade<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Trade {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TradeArgs) -> flatbuffers::WIPOffset<Trade<'bldr>> {
      let mut builder = TradeBuilder::new(_fbb);
      builder.add_price(args.price);
      builder.add_size_(args.size_);
      builder.finish()
    }

    pub const VT_PRICE: flatbuffers::VOffsetT = 4;
    pub const VT_SIZE_: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn price(&self) -> u64 {
    self._tab.get::<u64>(Trade::VT_PRICE, Some(0)).unwrap()
  }
  #[inline]
  pub fn size_(&self) -> u32 {
    self._tab.get::<u32>(Trade::VT_SIZE_, Some(0)).unwrap()
  }
}

pub struct TradeArgs {
    pub price: u64,
    pub size_: u32,
}
impl<'a> Default for TradeArgs {
    #[inline]
    fn default() -> Self {
        TradeArgs {
            price: 0,
            size_: 0,
        }
    }
}
pub struct TradeBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TradeBuilder<'a, 'b> {
  #[inline]
  pub fn add_price(&mut self, price: u64) {
    self.fbb_.push_slot::<u64>(Trade::VT_PRICE, price, 0);
  }
  #[inline]
  pub fn add_size_(&mut self, size_: u32) {
    self.fbb_.push_slot::<u32>(Trade::VT_SIZE_, size_, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TradeBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TradeBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Trade<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum LevelUpdateOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct LevelUpdate<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for LevelUpdate<'a> {
    type Inner = LevelUpdate<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> LevelUpdate<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        LevelUpdate {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args LevelUpdateArgs) -> flatbuffers::WIPOffset<LevelUpdate<'bldr>> {
      let mut builder = LevelUpdateBuilder::new(_fbb);
      builder.add_price(args.price);
      builder.add_size_(args.size_);
      builder.add_side(args.side);
      builder.add_flags(args.flags);
      builder.finish()
    }

    pub const VT_PRICE: flatbuffers::VOffsetT = 4;
    pub const VT_SIZE_: flatbuffers::VOffsetT = 6;
    pub const VT_FLAGS: flatbuffers::VOffsetT = 8;
    pub const VT_SIDE: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn price(&self) -> u64 {
    self._tab.get::<u64>(LevelUpdate::VT_PRICE, Some(0)).unwrap()
  }
  #[inline]
  pub fn size_(&self) -> u32 {
    self._tab.get::<u32>(LevelUpdate::VT_SIZE_, Some(0)).unwrap()
  }
  #[inline]
  pub fn flags(&self) -> u8 {
    self._tab.get::<u8>(LevelUpdate::VT_FLAGS, Some(0)).unwrap()
  }
  #[inline]
  pub fn side(&self) -> Side {
    self._tab.get::<Side>(LevelUpdate::VT_SIDE, Some(Side::Buy)).unwrap()
  }
}

pub struct LevelUpdateArgs {
    pub price: u64,
    pub size_: u32,
    pub flags: u8,
    pub side: Side,
}
impl<'a> Default for LevelUpdateArgs {
    #[inline]
    fn default() -> Self {
        LevelUpdateArgs {
            price: 0,
            size_: 0,
            flags: 0,
            side: Side::Buy,
        }
    }
}
pub struct LevelUpdateBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> LevelUpdateBuilder<'a, 'b> {
  #[inline]
  pub fn add_price(&mut self, price: u64) {
    self.fbb_.push_slot::<u64>(LevelUpdate::VT_PRICE, price, 0);
  }
  #[inline]
  pub fn add_size_(&mut self, size_: u32) {
    self.fbb_.push_slot::<u32>(LevelUpdate::VT_SIZE_, size_, 0);
  }
  #[inline]
  pub fn add_flags(&mut self, flags: u8) {
    self.fbb_.push_slot::<u8>(LevelUpdate::VT_FLAGS, flags, 0);
  }
  #[inline]
  pub fn add_side(&mut self, side: Side) {
    self.fbb_.push_slot::<Side>(LevelUpdate::VT_SIDE, side, Side::Buy);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> LevelUpdateBuilder<'a, 'b> {
    let start = _fbb.start_table();
    LevelUpdateBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<LevelUpdate<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum MessageOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Message<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Message<'a> {
    type Inner = Message<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Message<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Message {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MessageArgs<'args>) -> flatbuffers::WIPOffset<Message<'bldr>> {
      let mut builder = MessageBuilder::new(_fbb);
      builder.add_ts_nanos(args.ts_nanos);
      if let Some(x) = args.body { builder.add_body(x); }
      if let Some(x) = args.symbol { builder.add_symbol(x); }
      builder.add_body_type(args.body_type);
      builder.finish()
    }

    pub const VT_TS_NANOS: flatbuffers::VOffsetT = 4;
    pub const VT_SYMBOL: flatbuffers::VOffsetT = 6;
    pub const VT_BODY_TYPE: flatbuffers::VOffsetT = 8;
    pub const VT_BODY: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn ts_nanos(&self) -> i64 {
    self._tab.get::<i64>(Message::VT_TS_NANOS, Some(0)).unwrap()
  }
  #[inline]
  pub fn symbol(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Message::VT_SYMBOL, None)
  }
  #[inline]
  pub fn body_type(&self) -> MessageBody {
    self._tab.get::<MessageBody>(Message::VT_BODY_TYPE, Some(MessageBody::NONE)).unwrap()
  }
  #[inline]
  pub fn body(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Message::VT_BODY, None)
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn body_as_trade(&'a self) -> Option<Trade> {
    if self.body_type() == MessageBody::Trade {
      self.body().map(|u| Trade::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn body_as_level_update(&'a self) -> Option<LevelUpdate> {
    if self.body_type() == MessageBody::LevelUpdate {
      self.body().map(|u| LevelUpdate::init_from_table(u))
    } else {
      None
    }
  }

}

pub struct MessageArgs<'a> {
    pub ts_nanos: i64,
    pub symbol: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub body_type: MessageBody,
    pub body: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for MessageArgs<'a> {
    #[inline]
    fn default() -> Self {
        MessageArgs {
            ts_nanos: 0,
            symbol: None,
            body_type: MessageBody::NONE,
            body: None,
        }
    }
}
pub struct MessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_ts_nanos(&mut self, ts_nanos: i64) {
    self.fbb_.push_slot::<i64>(Message::VT_TS_NANOS, ts_nanos, 0);
  }
  #[inline]
  pub fn add_symbol(&mut self, symbol: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_SYMBOL, symbol);
  }
  #[inline]
  pub fn add_body_type(&mut self, body_type: MessageBody) {
    self.fbb_.push_slot::<MessageBody>(Message::VT_BODY_TYPE, body_type, MessageBody::NONE);
  }
  #[inline]
  pub fn add_body(&mut self, body: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_BODY, body);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Message<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum MultiMessageOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct MultiMessage<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for MultiMessage<'a> {
    type Inner = MultiMessage<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> MultiMessage<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        MultiMessage {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MultiMessageArgs<'args>) -> flatbuffers::WIPOffset<MultiMessage<'bldr>> {
      let mut builder = MultiMessageBuilder::new(_fbb);
      builder.add_seq_no(args.seq_no);
      if let Some(x) = args.messages { builder.add_messages(x); }
      builder.finish()
    }

    pub const VT_SEQ_NO: flatbuffers::VOffsetT = 4;
    pub const VT_MESSAGES: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn seq_no(&self) -> u64 {
    self._tab.get::<u64>(MultiMessage::VT_SEQ_NO, Some(0)).unwrap()
  }
  #[inline]
  pub fn messages(&self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Message<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Message<'a>>>>>(MultiMessage::VT_MESSAGES, None)
  }
}

pub struct MultiMessageArgs<'a> {
    pub seq_no: u64,
    pub messages: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Message<'a >>>>>,
}
impl<'a> Default for MultiMessageArgs<'a> {
    #[inline]
    fn default() -> Self {
        MultiMessageArgs {
            seq_no: 0,
            messages: None,
        }
    }
}
pub struct MultiMessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MultiMessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_seq_no(&mut self, seq_no: u64) {
    self.fbb_.push_slot::<u64>(MultiMessage::VT_SEQ_NO, seq_no, 0);
  }
  #[inline]
  pub fn add_messages(&mut self, messages: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Message<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MultiMessage::VT_MESSAGES, messages);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MultiMessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MultiMessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<MultiMessage<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

}  // pub mod MdShootout

