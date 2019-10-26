use std::io::{BufRead, Write};
use std::str::from_utf8_unchecked;

use crate::iex::{IexMessage, IexPayload};
use crate::{marketdata_proto, RunnerDeserialize, RunnerSerialize, StreamVec, Summarizer};

pub struct ProtoWriter;

impl ProtoWriter {
    pub fn new() -> ProtoWriter {
        ProtoWriter { }
    }
}

impl RunnerSerialize for ProtoWriter {
    fn serialize(&mut self, payload: &IexPayload, output: &mut Vec<u8>) {
    }
}

pub struct ProtoReader;

impl ProtoReader {
    pub fn new() -> ProtoReader {
        ProtoReader {}
    }
}

impl RunnerDeserialize for ProtoReader {
    fn deserialize<'a>(&self, buf: &'a mut StreamVec, stats: &mut Summarizer) -> Result<(), ()> {
        let data = buf.fill_buf().unwrap();
        if data.len() == 0 {
            return Err(());
        }


        buf.consume(0);
        Ok(())
    }
}
