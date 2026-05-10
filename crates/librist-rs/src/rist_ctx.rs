use super::*;

pub struct RistCtx<'a> {
    raw_ptr: std::ptr::NonNull<librist_sys::rist_ctx>,
    receiver_data_callback_handler: Option<Box<dyn ReceiverDataCallbackHandler2 + 'a>>,
    receiver_session_timeout_callback_handler:
        Option<Box<dyn ReceiverSessionTimeoutCallbackHandler + 'a>>,
    receiver_data_notify_fd: Option<std::os::fd::BorrowedFd<'a>>,
    librist_rs_ctx_id: u128,
    rist_peers: utility::FastIntDictionary<(
        std::ptr::NonNull<librist_sys::rist_peer>,
        RistPeerConfig,
        u128,
    )>,
}

impl<'a> RistCtx<'a> {
    /// [`librist_sys::rist_receiver_create`]
    pub fn receiver_create(
        rist_profile: RistProfile,
        rist_logging_settings: &mut RistLoggingSettings,
    ) -> Result<Self, CommonError> {
        let mut rist_ctx = std::ptr::null_mut();
        let code = unsafe {
            librist_sys::rist_receiver_create(
                &mut rist_ctx,
                rist_profile,
                rist_logging_settings.as_mut_ptr(),
            )
        };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_receiver_create",
                code,
            });
        }
        match std::ptr::NonNull::new(rist_ctx) {
            Some(raw_ptr) => Ok(Self {
                raw_ptr,
                receiver_data_callback_handler: None,
                receiver_session_timeout_callback_handler: None,
                receiver_data_notify_fd: None,
                librist_rs_ctx_id: rand::random::<u128>(),
                rist_peers: utility::FastIntDictionary::new(),
            }),
            None => Err(CommonError::NullPointer {
                function: "rist_receiver_create",
                value_type: "rist_ctx",
            }),
        }
    }

    /// [`librist_sys::rist_receiver_nack_type_set`]
    pub fn receiver_set_nack_type(
        &mut self,
        rist_nack_type: RistNackType,
    ) -> Result<(), CommonError> {
        let code = unsafe {
            librist_sys::rist_receiver_nack_type_set(self.raw_ptr.as_ptr(), rist_nack_type)
        };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_receiver_nack_type_set",
                code,
            });
        }
        Ok(())
    }

    /// [`librist_sys::rist_receiver_set_output_fifo_size`]
    ///
    /// C function may return -4, -3, -2, -1 and 0.
    pub fn receiver_set_output_fifo_size(&mut self, desired_size: u32) -> Result<(), CommonError> {
        let code = unsafe {
            librist_sys::rist_receiver_set_output_fifo_size(self.raw_ptr.as_ptr(), desired_size)
        };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_receiver_set_output_fifo_size",
                code,
            });
        }
        Ok(())
    }

    /// [`librist_sys::rist_receiver_data_read2`]
    ///
    /// [`librist_sys::rist_receiver_data_read`] is deprecated.
    pub fn receiver_read_data2(&mut self, timeout: i32) -> Result<RistDataBlock, CommonError> {
        let mut rist_data_block = std::ptr::null_mut();
        let code = unsafe {
            librist_sys::rist_receiver_data_read2(
                self.raw_ptr.as_ptr(),
                &mut rist_data_block,
                timeout,
            )
        };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_receiver_data_read2",
                code,
            });
        }
        match std::ptr::NonNull::new(rist_data_block) {
            Some(raw_ptr) => Ok(RistDataBlock::from_ptr(raw_ptr)),
            None => Err(CommonError::NullPointer {
                function: "rist_receiver_data_read2",
                value_type: "rist_data_block",
            }),
        }
    }

    /// [`librist_sys::rist_receiver_data_callback_set2`]
    ///
    /// [`librist_sys::rist_receiver_data_callback_set`] is deprecated.
    pub fn receiver_set_data_callback2<Handler>(
        &mut self,
        handler: Handler,
    ) -> Result<(), CommonError>
    where
        Handler: ReceiverDataCallbackHandler2 + 'a,
    {
        let callback = Handler::handle_raw;
        let mut handler = Box::new(handler);
        let handler_ptr = handler.as_mut() as *mut Handler as *mut std::os::raw::c_void;
        let code = unsafe {
            librist_sys::rist_receiver_data_callback_set2(
                self.raw_ptr.as_ptr(),
                Some(callback),
                handler_ptr,
            )
        };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_receiver_data_callback_set2",
                code,
            });
        }
        self.receiver_data_callback_handler = Some(handler);
        Ok(())
    }

    /// [`librist_sys::rist_receiver_session_timeout_callback_set`]
    pub fn receiver_set_session_timeout_callback<Handler>(
        &mut self,
        handler: Handler,
    ) -> Result<(), CommonError>
    where
        Handler: ReceiverSessionTimeoutCallbackHandler + 'a,
    {
        let callback = Handler::handle_raw;
        let mut handler = Box::new(handler);
        let handler_ptr = handler.as_mut() as *mut Handler as *mut std::os::raw::c_void;
        let code = unsafe {
            librist_sys::rist_receiver_session_timeout_callback_set(
                self.raw_ptr.as_ptr(),
                Some(callback),
                handler_ptr,
            )
        };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_receiver_session_timeout_callback_set",
                code,
            });
        }
        self.receiver_session_timeout_callback_handler = Some(handler);
        Ok(())
    }

    /// [`librist_sys::rist_receiver_data_notify_fd_set`]
    pub fn receiver_set_data_notify_fd(
        &mut self,
        fd: std::os::fd::BorrowedFd<'a>,
    ) -> Result<(), CommonError> {
        let code = unsafe {
            librist_sys::rist_receiver_data_notify_fd_set(
                self.raw_ptr.as_ptr(),
                std::os::fd::AsRawFd::as_raw_fd(&fd),
            )
        };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_receiver_data_notify_fd_set",
                code,
            });
        }
        self.receiver_data_notify_fd = Some(fd);
        Ok(())
    }

    /// [`librist_sys::rist_sender_create`]
    pub fn sender_create(
        rist_profile: RistProfile,
        flow_id: u32,
        rist_logging_settings: &mut RistLoggingSettings,
    ) -> Result<Self, CommonError> {
        let mut rist_ctx = std::ptr::null_mut();
        let code = unsafe {
            librist_sys::rist_sender_create(
                &mut rist_ctx,
                rist_profile,
                flow_id,
                rist_logging_settings.as_mut_ptr(),
            )
        };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_sender_create",
                code,
            });
        }
        match std::ptr::NonNull::new(rist_ctx) {
            Some(raw_ptr) => Ok(Self {
                raw_ptr,
                receiver_data_callback_handler: None,
                receiver_session_timeout_callback_handler: None,
                receiver_data_notify_fd: None,
                librist_rs_ctx_id: rand::random::<u128>(),
                rist_peers: utility::FastIntDictionary::new(),
            }),
            None => Err(CommonError::NullPointer {
                function: "rist_sender_create",
                value_type: "rist_ctx",
            }),
        }
    }

    /// [`librist_sys::rist_sender_npd_get`]
    pub fn sender_get_npd(&self) -> Result<bool, CommonError> {
        let mut npd = false;
        let code = unsafe { librist_sys::rist_sender_npd_get(self.as_ptr(), &mut npd) };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_sender_npd_get",
                code,
            });
        }
        Ok(npd)
    }

    /// [`librist_sys::rist_sender_npd_enable`]
    pub fn sender_enable_npd(&mut self) -> Result<(), CommonError> {
        let code = unsafe { librist_sys::rist_sender_npd_enable(self.as_mut_ptr()) };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_sender_npd_enable",
                code,
            });
        }
        Ok(())
    }

    /// [`librist_sys::rist_sender_npd_disable`]
    pub fn sender_disable_npd(&mut self) -> Result<(), CommonError> {
        let code = unsafe { librist_sys::rist_sender_npd_disable(self.as_mut_ptr()) };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_sender_npd_disable",
                code,
            });
        }
        Ok(())
    }

    /// [`librist_sys::rist_sender_flow_id_get`]
    pub fn sender_get_flow_id(&mut self) -> Result<u32, CommonError> {
        let mut flow_id = 0;
        let code = unsafe { librist_sys::rist_sender_flow_id_get(self.as_mut_ptr(), &mut flow_id) };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_sender_flow_id_get",
                code,
            });
        }
        Ok(flow_id)
    }

    /// [`librist_sys::rist_sender_flow_id_set`]
    pub fn sender_set_flow_id(&mut self, flow_id: u32) -> Result<(), CommonError> {
        let code = unsafe { librist_sys::rist_sender_flow_id_set(self.as_mut_ptr(), flow_id) };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_sender_flow_id_set",
                code,
            });
        }
        Ok(())
    }

    /// [`librist_sys::rist_sender_data_write`]
    pub fn sender_write_data(&mut self, data_block: &RistDataBlock) -> Result<(), CommonError> {
        let data_block = data_block.as_ptr();
        let code = unsafe { librist_sys::rist_sender_data_write(self.as_mut_ptr(), data_block) };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_sender_data_write",
                code,
            });
        }
        Ok(())
    }

    /// [`librist_sys::rist_peer_create`]
    pub fn peer_create(&mut self, config: &RistPeerConfig) -> Result<RistPeer, CommonError> {
        let config_ptr = config.as_ptr();
        let mut peer = std::ptr::null_mut();
        let code =
            unsafe { librist_sys::rist_peer_create(self.as_mut_ptr(), &mut peer, config_ptr) };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_peer_create",
                code,
            });
        }
        match std::ptr::NonNull::new(peer) {
            Some(peer) => {
                let librist_rs_peer_id = rand::random();
                let key = self
                    .rist_peers
                    .insert((peer, config.clone(), librist_rs_peer_id));

                Ok(RistPeer {
                    librist_rs_ctx_id: self.librist_rs_ctx_id,
                    librist_rs_peer_key: key,
                    librist_rs_peer_id,
                })
            }
            None => Err(CommonError::NullPointer {
                function: "rist_peer_create",
                value_type: "rist_peer",
            }),
        }
    }

    fn internal_get_peer_ptr(
        &self,
        peer: &RistPeer,
    ) -> Option<std::ptr::NonNull<librist_sys::rist_peer>> {
        let (ptr, _, id) = self.rist_peers.get(peer.librist_rs_peer_key)?;
        if peer.librist_rs_peer_id == *id {
            Some(*ptr)
        } else {
            None
        }
    }

    /// [`librist_sys::rist_peer_destroy`]
    pub fn peer_destroy(&mut self, peer: RistPeer) -> Result<(), PeerOperationError> {
        if let Some(peer_ptr) = self.internal_get_peer_ptr(&peer) {
            let code =
                unsafe { librist_sys::rist_peer_destroy(self.as_mut_ptr(), peer_ptr.as_ptr()) };
            if code != 0 {
                return Err(PeerOperationError::CallFailed {
                    function: "rist_peer_destroy",
                    code,
                });
            }
            self.rist_peers.remove(peer.librist_rs_peer_key);
            Ok(())
        } else {
            Err(PeerOperationError::InvalidPeer(peer))
        }
    }

    /// [`librist_sys::rist_peer_weight_set`]
    pub fn peer_set_weight(
        &mut self,
        peer: RistPeer,
        weight: u32,
    ) -> Result<(), PeerOperationError> {
        if let Some(peer_ptr) = self.internal_get_peer_ptr(&peer) {
            let code = unsafe {
                librist_sys::rist_peer_weight_set(self.as_mut_ptr(), peer_ptr.as_ptr(), weight)
            };
            if code != 0 {
                return Err(PeerOperationError::CallFailed {
                    function: "rist_peer_weight_set",
                    code,
                });
            }
            Ok(())
        } else {
            Err(PeerOperationError::InvalidPeer(peer))
        }
    }

    /// [`librist_sys::rist_peer_get_socket`]
    ///
    /// Returns data socket (`socket` of [`librist_sys::rist_peer_get_socket`])
    pub fn peer_get_socket(
        &self,
        peer: RistPeer,
    ) -> Result<std::os::fd::BorrowedFd<'_>, PeerGetSocketError> {
        if let Some(peer_ptr) = self.internal_get_peer_ptr(&peer) {
            let mut socket = -1;
            let mut _socket_extra = -1;
            let code = unsafe {
                librist_sys::rist_peer_get_socket(
                    peer_ptr.as_ptr(),
                    &mut socket,
                    &mut _socket_extra,
                )
            };
            // C library returns 1 when both socket is available.
            if code < 0 {
                return Err(PeerGetSocketError::CallFailed {
                    function: "rist_peer_get_socket",
                    code,
                });
            }
            if socket < 0 {
                return Err(PeerGetSocketError::InvalidSocket { socket });
            }
            let socket = unsafe { std::os::fd::BorrowedFd::borrow_raw(socket) };
            Ok(socket)
        } else {
            Err(PeerGetSocketError::InvalidPeer(peer))
        }
    }

    /// [`librist_sys::rist_peer_get_socket`]
    ///
    /// Returns rtcp socket (`socket_extra` of [`librist_sys::rist_peer_get_socket`])
    ///
    /// Simple profile only.
    pub fn peer_get_rtcp_socket(
        &self,
        peer: RistPeer,
    ) -> Result<Option<std::os::fd::BorrowedFd<'_>>, PeerGetSocketError> {
        if let Some(peer_ptr) = self.internal_get_peer_ptr(&peer) {
            let mut _socket = -1;
            let mut socket_extra = -1;
            let code = unsafe {
                librist_sys::rist_peer_get_socket(
                    peer_ptr.as_ptr(),
                    &mut _socket,
                    &mut socket_extra,
                )
            };
            // C library returns 1 when both socket is available.
            if code < 0 {
                return Err(PeerGetSocketError::CallFailed {
                    function: "rist_peer_get_socket",
                    code,
                });
            }
            if code == 0 {
                return Ok(None);
            }
            if socket_extra < 0 {
                return Err(PeerGetSocketError::InvalidSocket {
                    socket: socket_extra,
                });
            }
            let socket = unsafe { std::os::fd::BorrowedFd::borrow_raw(socket_extra) };
            Ok(Some(socket))
        } else {
            Err(PeerGetSocketError::InvalidPeer(peer))
        }
    }

    /// [`librist_sys::rist_destroy`]
    ///
    /// Explicit stop. This is the same to dropping the context.
    pub fn destroy(self) {}
}

impl<'a> Drop for RistCtx<'a> {
    fn drop(&mut self) {
        unsafe {
            librist_sys::rist_destroy(self.as_mut_ptr());
        }
    }
}

impl<'a> StatefulWrapper<librist_sys::rist_ctx> for RistCtx<'a> {
    fn as_ptr(&self) -> *const librist_sys::rist_ctx {
        self.raw_ptr.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut librist_sys::rist_ctx {
        self.raw_ptr.as_ptr()
    }
}
