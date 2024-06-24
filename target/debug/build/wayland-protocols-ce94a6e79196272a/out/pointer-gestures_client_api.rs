//
// This file was auto-generated, do not edit directly
//

pub mod zwp_pointer_gestures_v1 {
    //! touchpad gestures
    //!
    //! A global interface to provide semantic touchpad gestures for a given
    //! pointer.
    //! 
    //! Two gestures are currently supported: swipe and zoom/rotate.
    //! All gestures follow a three-stage cycle: begin, update, end and
    //! are identified by a unique id.
    //! 
    //! Warning! The protocol described in this file is experimental and
    //! backward incompatible changes may be made. Backward compatible changes
    //! may be added together with the corresponding interface version bump.
    //! Backward incompatible changes are done by bumping the version number in
    //! the protocol and interface names and resetting the interface version.
    //! Once the protocol is to be declared stable, the 'z' prefix and the
    //! version number in the protocol and interface names are removed and the
    //! interface version number is reset.
    use super::EventQueueHandle;
    use super::Proxy;
    use super::RequestResult;

    use super::Liveness;
    use super::interfaces::*;
    use wayland_sys::common::*;
    use std::any::Any;
    use std::ffi::{CString,CStr};
    use std::os::raw::c_void;
    use std::ptr;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
    use wayland_sys::RUST_MANAGED;
    use wayland_sys::client::*;

    pub struct ZwpPointerGesturesV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpPointerGesturesV1 {}
    unsafe impl Sync for ZwpPointerGesturesV1 {}
    impl Proxy for ZwpPointerGesturesV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpPointerGesturesV1 {
            let data = Box::into_raw(Box::new((
                ptr::null_mut::<c_void>(),
                ptr::null_mut::<c_void>(),
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut())))
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpPointerGesturesV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpPointerGesturesV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut (*mut c_void, *mut c_void, Arc<(AtomicBool, AtomicPtr<()>)>);
                ZwpPointerGesturesV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpPointerGesturesV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_pointer_gestures_v1_interface } }
        fn interface_name() -> &'static str { "zwp_pointer_gestures_v1"  }
        fn supported_version() -> u32 { 1 }
        fn version(&self) -> u32 { unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_version, self.ptr()) } }

        fn status(&self) -> Liveness {
            if let Some(ref data) = self.data {
                if data.0.load(Ordering::SeqCst) {
                    Liveness::Alive
                } else {
                    Liveness::Dead
                }
            } else {
                Liveness::Unmanaged
            }
        }

        fn equals(&self, other: &ZwpPointerGesturesV1) -> bool {
            self.status() != Liveness::Dead && other.status() != Liveness::Dead && self.ptr == other.ptr
        }

        fn set_user_data(&self, ptr: *mut ()) {
            if let Some(ref data) = self.data {
                data.1.store(ptr, Ordering::SeqCst);
            }
        }
        fn get_user_data(&self) -> *mut () {
            if let Some(ref data) = self.data {
                data.1.load(Ordering::SeqCst)
            } else {
                ::std::ptr::null_mut()
            }
        }
    }
    const ZWP_POINTER_GESTURES_V1_GET_SWIPE_GESTURE: u32 = 0;
    const ZWP_POINTER_GESTURES_V1_GET_PINCH_GESTURE: u32 = 1;
    impl ZwpPointerGesturesV1 {
        /// get swipe gesture
        ///
        /// Create a swipe gesture object. See the
        /// wl_pointer_gesture_swipe interface for details.
        pub fn get_swipe_gesture(&self, pointer: &super::wl_pointer::WlPointer) ->super::zwp_pointer_gesture_swipe_v1::ZwpPointerGestureSwipeV1 {
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_POINTER_GESTURES_V1_GET_SWIPE_GESTURE, &zwp_pointer_gesture_swipe_v1_interface, ptr::null_mut::<wl_proxy>(), pointer.ptr()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            proxy
        }
        /// get pinch gesture
        ///
        /// Create a pinch gesture object. See the
        /// wl_pointer_gesture_pinch interface for details.
        pub fn get_pinch_gesture(&self, pointer: &super::wl_pointer::WlPointer) ->super::zwp_pointer_gesture_pinch_v1::ZwpPointerGesturePinchV1 {
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_POINTER_GESTURES_V1_GET_PINCH_GESTURE, &zwp_pointer_gesture_pinch_v1_interface, ptr::null_mut::<wl_proxy>(), pointer.ptr()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            proxy
        }
    }
}
pub mod zwp_pointer_gesture_swipe_v1 {
    //! a swipe gesture object
    //!
    //! A swipe gesture object notifies a client about a multi-finger swipe
    //! gesture detected on an indirect input device such as a touchpad.
    //! The gesture is usually initiated by multiple fingers moving in the
    //! same direction but once initiated the direction may change.
    //! The precise conditions of when such a gesture is detected are
    //! implementation-dependent.
    //! 
    //! A gesture consists of three stages: begin, update (optional) and end.
    //! There cannot be multiple simultaneous pinch or swipe gestures on a
    //! same pointer/seat, how compositors prevent these situations is
    //! implementation-dependent.
    //! 
    //! A gesture may be cancelled by the compositor or the hardware.
    //! Clients should not consider performing permanent or irreversible
    //! actions until the end of a gesture has been received.
    use super::EventQueueHandle;
    use super::Proxy;
    use super::RequestResult;

    use super::Liveness;
    use super::interfaces::*;
    use wayland_sys::common::*;
    use std::any::Any;
    use std::ffi::{CString,CStr};
    use std::os::raw::c_void;
    use std::ptr;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
    use wayland_sys::RUST_MANAGED;
    use wayland_sys::client::*;

    pub struct ZwpPointerGestureSwipeV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpPointerGestureSwipeV1 {}
    unsafe impl Sync for ZwpPointerGestureSwipeV1 {}
    impl Proxy for ZwpPointerGestureSwipeV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpPointerGestureSwipeV1 {
            let data = Box::into_raw(Box::new((
                ptr::null_mut::<c_void>(),
                ptr::null_mut::<c_void>(),
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut())))
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpPointerGestureSwipeV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpPointerGestureSwipeV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut (*mut c_void, *mut c_void, Arc<(AtomicBool, AtomicPtr<()>)>);
                ZwpPointerGestureSwipeV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpPointerGestureSwipeV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_pointer_gesture_swipe_v1_interface } }
        fn interface_name() -> &'static str { "zwp_pointer_gesture_swipe_v1"  }
        fn supported_version() -> u32 { 1 }
        fn version(&self) -> u32 { unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_version, self.ptr()) } }

        fn status(&self) -> Liveness {
            if let Some(ref data) = self.data {
                if data.0.load(Ordering::SeqCst) {
                    Liveness::Alive
                } else {
                    Liveness::Dead
                }
            } else {
                Liveness::Unmanaged
            }
        }

        fn equals(&self, other: &ZwpPointerGestureSwipeV1) -> bool {
            self.status() != Liveness::Dead && other.status() != Liveness::Dead && self.ptr == other.ptr
        }

        fn set_user_data(&self, ptr: *mut ()) {
            if let Some(ref data) = self.data {
                data.1.store(ptr, Ordering::SeqCst);
            }
        }
        fn get_user_data(&self) -> *mut () {
            if let Some(ref data) = self.data {
                data.1.load(Ordering::SeqCst)
            } else {
                ::std::ptr::null_mut()
            }
        }
    }
    pub trait Handler {
        /// multi-finger swipe begin
        ///
        /// This event is sent when a multi-finger swipe gesture is detected
        /// on the device.
        fn begin(&mut self, evqh: &mut EventQueueHandle,  proxy: &ZwpPointerGestureSwipeV1, serial: u32, time: u32, surface: &super::wl_surface::WlSurface, fingers: u32) {}
        /// multi-finger swipe motion
        ///
        /// This event is sent when a multi-finger swipe gesture changes the
        /// position of the logical center.
        /// 
        /// The dx and dy coordinates are relative coordinates of the logical
        /// center of the gesture compared to the previous event.
        fn update(&mut self, evqh: &mut EventQueueHandle,  proxy: &ZwpPointerGestureSwipeV1, time: u32, dx: f64, dy: f64) {}
        /// multi-finger swipe end
        ///
        /// This event is sent when a multi-finger swipe gesture ceases to
        /// be valid. This may happen when one or more fingers are lifted or
        /// the gesture is cancelled.
        /// 
        /// When a gesture is cancelled, the client should undo state changes
        /// caused by this gesture. What causes a gesture to be cancelled is
        /// implementation-dependent.
        fn end(&mut self, evqh: &mut EventQueueHandle,  proxy: &ZwpPointerGestureSwipeV1, serial: u32, time: u32, cancelled: i32) {}
        #[doc(hidden)]
        unsafe fn __message(&mut self, evq: &mut EventQueueHandle,  proxy: &ZwpPointerGestureSwipeV1, opcode: u32, args: *const wl_argument) -> Result<(),()> {
            match opcode {
                0 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    let time = {*(args.offset(1) as *const u32)};
                    let surface = {Proxy::from_ptr_initialized(*(args.offset(2) as *const *mut wl_proxy))};
                    let fingers = {*(args.offset(3) as *const u32)};
                    self.begin(evq,  proxy, serial, time, &surface, fingers);
                },
                1 => {
                    let time = {*(args.offset(0) as *const u32)};
                    let dx = {wl_fixed_to_double(*(args.offset(1) as *const i32))};
                    let dy = {wl_fixed_to_double(*(args.offset(2) as *const i32))};
                    self.update(evq,  proxy, time, dx, dy);
                },
                2 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    let time = {*(args.offset(1) as *const u32)};
                    let cancelled = {*(args.offset(2) as *const i32)};
                    self.end(evq,  proxy, serial, time, cancelled);
                },
                _ => return Err(())
            }
            Ok(())
        }
    }
    const ZWP_POINTER_GESTURE_SWIPE_V1_DESTROY: u32 = 0;
    impl ZwpPointerGestureSwipeV1 {
        /// destroy the pointer swipe gesture object
        ///
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_POINTER_GESTURE_SWIPE_V1_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
    }
}
pub mod zwp_pointer_gesture_pinch_v1 {
    //! a pinch gesture object
    //!
    //! A pinch gesture object notifies a client about a multi-finger pinch
    //! gesture detected on an indirect input device such as a touchpad.
    //! The gesture is usually initiated by multiple fingers moving towards
    //! each other or away from each other, or by two or more fingers rotating
    //! around a logical center of gravity. The precise conditions of when
    //! such a gesture is detected are implementation-dependent.
    //! 
    //! A gesture consists of three stages: begin, update (optional) and end.
    //! There cannot be multiple simultaneous pinch or swipe gestures on a
    //! same pointer/seat, how compositors prevent these situations is
    //! implementation-dependent.
    //! 
    //! A gesture may be cancelled by the compositor or the hardware.
    //! Clients should not consider performing permanent or irreversible
    //! actions until the end of a gesture has been received.
    use super::EventQueueHandle;
    use super::Proxy;
    use super::RequestResult;

    use super::Liveness;
    use super::interfaces::*;
    use wayland_sys::common::*;
    use std::any::Any;
    use std::ffi::{CString,CStr};
    use std::os::raw::c_void;
    use std::ptr;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
    use wayland_sys::RUST_MANAGED;
    use wayland_sys::client::*;

    pub struct ZwpPointerGesturePinchV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpPointerGesturePinchV1 {}
    unsafe impl Sync for ZwpPointerGesturePinchV1 {}
    impl Proxy for ZwpPointerGesturePinchV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpPointerGesturePinchV1 {
            let data = Box::into_raw(Box::new((
                ptr::null_mut::<c_void>(),
                ptr::null_mut::<c_void>(),
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut())))
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpPointerGesturePinchV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpPointerGesturePinchV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut (*mut c_void, *mut c_void, Arc<(AtomicBool, AtomicPtr<()>)>);
                ZwpPointerGesturePinchV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpPointerGesturePinchV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_pointer_gesture_pinch_v1_interface } }
        fn interface_name() -> &'static str { "zwp_pointer_gesture_pinch_v1"  }
        fn supported_version() -> u32 { 1 }
        fn version(&self) -> u32 { unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_version, self.ptr()) } }

        fn status(&self) -> Liveness {
            if let Some(ref data) = self.data {
                if data.0.load(Ordering::SeqCst) {
                    Liveness::Alive
                } else {
                    Liveness::Dead
                }
            } else {
                Liveness::Unmanaged
            }
        }

        fn equals(&self, other: &ZwpPointerGesturePinchV1) -> bool {
            self.status() != Liveness::Dead && other.status() != Liveness::Dead && self.ptr == other.ptr
        }

        fn set_user_data(&self, ptr: *mut ()) {
            if let Some(ref data) = self.data {
                data.1.store(ptr, Ordering::SeqCst);
            }
        }
        fn get_user_data(&self) -> *mut () {
            if let Some(ref data) = self.data {
                data.1.load(Ordering::SeqCst)
            } else {
                ::std::ptr::null_mut()
            }
        }
    }
    pub trait Handler {
        /// multi-finger pinch begin
        ///
        /// This event is sent when a multi-finger pinch gesture is detected
        /// on the device.
        fn begin(&mut self, evqh: &mut EventQueueHandle,  proxy: &ZwpPointerGesturePinchV1, serial: u32, time: u32, surface: &super::wl_surface::WlSurface, fingers: u32) {}
        /// multi-finger pinch motion
        ///
        /// This event is sent when a multi-finger pinch gesture changes the
        /// position of the logical center, the rotation or the relative scale.
        /// 
        /// The dx and dy coordinates are relative coordinates in the
        /// surface coordinate space of the logical center of the gesture.
        /// 
        /// The scale factor is an absolute scale compared to the
        /// pointer_gesture_pinch.begin event, e.g. a scale of 2 means the fingers
        /// are now twice as far apart as on pointer_gesture_pinch.begin.
        /// 
        /// The rotation is the relative angle in degrees clockwise compared to the previous
        /// pointer_gesture_pinch.begin or pointer_gesture_pinch.update event.
        fn update(&mut self, evqh: &mut EventQueueHandle,  proxy: &ZwpPointerGesturePinchV1, time: u32, dx: f64, dy: f64, scale: f64, rotation: f64) {}
        /// multi-finger pinch end
        ///
        /// This event is sent when a multi-finger pinch gesture ceases to
        /// be valid. This may happen when one or more fingers are lifted or
        /// the gesture is cancelled.
        /// 
        /// When a gesture is cancelled, the client should undo state changes
        /// caused by this gesture. What causes a gesture to be cancelled is
        /// implementation-dependent.
        fn end(&mut self, evqh: &mut EventQueueHandle,  proxy: &ZwpPointerGesturePinchV1, serial: u32, time: u32, cancelled: i32) {}
        #[doc(hidden)]
        unsafe fn __message(&mut self, evq: &mut EventQueueHandle,  proxy: &ZwpPointerGesturePinchV1, opcode: u32, args: *const wl_argument) -> Result<(),()> {
            match opcode {
                0 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    let time = {*(args.offset(1) as *const u32)};
                    let surface = {Proxy::from_ptr_initialized(*(args.offset(2) as *const *mut wl_proxy))};
                    let fingers = {*(args.offset(3) as *const u32)};
                    self.begin(evq,  proxy, serial, time, &surface, fingers);
                },
                1 => {
                    let time = {*(args.offset(0) as *const u32)};
                    let dx = {wl_fixed_to_double(*(args.offset(1) as *const i32))};
                    let dy = {wl_fixed_to_double(*(args.offset(2) as *const i32))};
                    let scale = {wl_fixed_to_double(*(args.offset(3) as *const i32))};
                    let rotation = {wl_fixed_to_double(*(args.offset(4) as *const i32))};
                    self.update(evq,  proxy, time, dx, dy, scale, rotation);
                },
                2 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    let time = {*(args.offset(1) as *const u32)};
                    let cancelled = {*(args.offset(2) as *const i32)};
                    self.end(evq,  proxy, serial, time, cancelled);
                },
                _ => return Err(())
            }
            Ok(())
        }
    }
    const ZWP_POINTER_GESTURE_PINCH_V1_DESTROY: u32 = 0;
    impl ZwpPointerGesturePinchV1 {
        /// destroy the pinch gesture object
        ///
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_POINTER_GESTURE_PINCH_V1_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
    }
}
