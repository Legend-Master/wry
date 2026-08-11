use std::{
  borrow::Cow,
  io::{Cursor, Read},
  sync::Mutex,
};

use windows::Win32::{
  Foundation::{E_FAIL, E_POINTER, STG_E_CANTSAVE, S_FALSE, S_OK},
  System::Com::{ISequentialStream_Impl, IStream, IStream_Impl},
};
use windows_core::implement;

#[implement(IStream)]
pub struct Stream(Mutex<Cursor<Cow<'static, [u8]>>>);

impl Stream {
  pub fn new(content: Cow<'static, [u8]>) -> Self {
    Self(Mutex::new(Cursor::new(content)))
  }
}

#[allow(non_snake_case)]
impl ISequentialStream_Impl for Stream_Impl {
  fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT {
    eprintln!("Stream::Read(cb={cb})");

    if !pcbread.is_null() {
      unsafe { pcbread.write(0) };
    }

    if cb == 0 {
      return S_OK;
    }
    if pv.is_null() {
      return E_POINTER;
    }

    let buffer = unsafe { std::slice::from_raw_parts_mut(pv.cast::<u8>(), cb as usize) };
    let Ok(mut cursor) = self.0.lock() else {
      return E_FAIL;
    };

    match cursor.read(buffer) {
      Ok(bytes_read) => {
        if !pcbread.is_null() {
          unsafe { pcbread.write(bytes_read as u32) };
        }

        if bytes_read == cb as usize {
          S_OK
        } else {
          S_FALSE
        }
      }
      Err(_) => E_FAIL,
    }
  }

  #[allow(unused_variables)]
  fn Write(
    &self,
    pv: *const core::ffi::c_void,
    cb: u32,
    pcbwritten: *mut u32,
  ) -> windows_core::HRESULT {
    eprintln!("Stream::Write(cb={cb})");
    STG_E_CANTSAVE
  }
}

#[allow(unused_variables)]
#[allow(non_snake_case)]
impl IStream_Impl for Stream_Impl {
  fn Seek(
    &self,
    dlibmove: i64,
    dworigin: windows::Win32::System::Com::STREAM_SEEK,
    plibnewposition: *mut u64,
  ) -> windows_core::Result<()> {
    eprintln!("Stream::Seek(dlibmove={dlibmove}, dworigin={dworigin:?})");
    Err(windows::Win32::Foundation::E_NOTIMPL.into())
  }

  fn SetSize(&self, libnewsize: u64) -> windows_core::Result<()> {
    eprintln!("Stream::SetSize(libnewsize={libnewsize})");
    Err(windows::Win32::Foundation::E_NOTIMPL.into())
  }

  fn CopyTo(
    &self,
    pstm: windows_core::Ref<IStream>,
    cb: u64,
    pcbread: *mut u64,
    pcbwritten: *mut u64,
  ) -> windows_core::Result<()> {
    eprintln!("Stream::CopyTo(cb={cb})");
    Err(windows::Win32::Foundation::E_NOTIMPL.into())
  }

  fn Commit(&self, grfcommitflags: &windows::Win32::System::Com::STGC) -> windows_core::Result<()> {
    eprintln!("Stream::Commit(grfcommitflags={grfcommitflags:?})");
    Err(windows::Win32::Foundation::E_NOTIMPL.into())
  }

  fn Revert(&self) -> windows_core::Result<()> {
    eprintln!("Stream::Revert()");
    Err(windows::Win32::Foundation::E_NOTIMPL.into())
  }

  fn LockRegion(
    &self,
    liboffset: u64,
    cb: u64,
    dwlocktype: &windows::Win32::System::Com::LOCKTYPE,
  ) -> windows_core::Result<()> {
    eprintln!("Stream::LockRegion(liboffset={liboffset}, cb={cb}, dwlocktype={dwlocktype:?})");
    Err(windows::Win32::Foundation::E_NOTIMPL.into())
  }

  fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()> {
    eprintln!("Stream::UnlockRegion(liboffset={liboffset}, cb={cb}, dwlocktype={dwlocktype})");
    Err(windows::Win32::Foundation::E_NOTIMPL.into())
  }

  fn Stat(
    &self,
    pstatstg: *mut windows::Win32::System::Com::STATSTG,
    grfstatflag: &windows::Win32::System::Com::STATFLAG,
  ) -> windows_core::Result<()> {
    eprintln!("Stream::Stat(grfstatflag={grfstatflag:?})");
    Err(windows::Win32::Foundation::E_NOTIMPL.into())
  }

  fn Clone(&self) -> windows_core::Result<IStream> {
    eprintln!("Stream::Clone()");
    Err(windows::Win32::Foundation::E_NOTIMPL.into())
  }
}
