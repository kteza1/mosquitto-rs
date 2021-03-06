/* automatically generated by rust-bindgen */
#![allow(non_camel_case_types)]

extern crate libc;

pub type Enum_mosq_err_t = ::libc::c_int;
pub const MOSQ_ERR_CONN_PENDING: ::libc::c_int = -1;
pub const MOSQ_ERR_SUCCESS: ::libc::c_int = 0;
pub const MOSQ_ERR_NOMEM: ::libc::c_int = 1;
pub const MOSQ_ERR_PROTOCOL: ::libc::c_int = 2;
pub const MOSQ_ERR_INVAL: ::libc::c_int = 3;
pub const MOSQ_ERR_NO_CONN: ::libc::c_int = 4;
pub const MOSQ_ERR_CONN_REFUSED: ::libc::c_int = 5;
pub const MOSQ_ERR_NOT_FOUND: ::libc::c_int = 6;
pub const MOSQ_ERR_CONN_LOST: ::libc::c_int = 7;
pub const MOSQ_ERR_TLS: ::libc::c_int = 8;
pub const MOSQ_ERR_PAYLOAD_SIZE: ::libc::c_int = 9;
pub const MOSQ_ERR_NOT_SUPPORTED: ::libc::c_int = 10;
pub const MOSQ_ERR_AUTH: ::libc::c_int = 11;
pub const MOSQ_ERR_ACL_DENIED: ::libc::c_int = 12;
pub const MOSQ_ERR_UNKNOWN: ::libc::c_int = 13;
pub const MOSQ_ERR_ERRNO: ::libc::c_int = 14;
pub const MOSQ_ERR_EAI: ::libc::c_int = 15;
pub const MOSQ_ERR_PROXY: ::libc::c_int = 16;
pub type Enum_mosq_opt_t = ::libc::c_uint;
pub const MOSQ_OPT_PROTOCOL_VERSION: ::libc::c_uint = 1;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mosquitto_message {
    pub mid: ::libc::c_int,
    pub topic: *mut ::libc::c_char,
    pub payload: *mut ::libc::c_void,
    pub payloadlen: ::libc::c_int,
    pub qos: ::libc::c_int,
    pub retain: u8,
}
impl ::std::clone::Clone for Struct_mosquitto_message {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_mosquitto_message {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_mosquitto { }
extern "C" {
    pub fn mosquitto_lib_version(major: *mut ::libc::c_int,
                                 minor: *mut ::libc::c_int,
                                 revision: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn mosquitto_lib_init() -> ::libc::c_int;
    pub fn mosquitto_lib_cleanup() -> ::libc::c_int;
    pub fn mosquitto_new(id: *const ::libc::c_char, clean_session: u8,
                         obj: *mut ::libc::c_void) -> *mut Struct_mosquitto;
    pub fn mosquitto_destroy(mosq: *mut Struct_mosquitto) -> ();
    pub fn mosquitto_reinitialise(mosq: *mut Struct_mosquitto,
                                  id: *const ::libc::c_char,
                                  clean_session: u8, obj: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn mosquitto_will_set(mosq: *mut Struct_mosquitto,
                              topic: *const ::libc::c_char,
                              payloadlen: ::libc::c_int,
                              payload: *const ::libc::c_void,
                              qos: ::libc::c_int, retain: u8)
     -> ::libc::c_int;
    pub fn mosquitto_will_clear(mosq: *mut Struct_mosquitto) -> ::libc::c_int;
    pub fn mosquitto_username_pw_set(mosq: *mut Struct_mosquitto,
                                     username: *const ::libc::c_char,
                                     password: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn mosquitto_connect(mosq: *mut Struct_mosquitto,
                             host: *const ::libc::c_char, port: ::libc::c_int,
                             keepalive: ::libc::c_int) -> ::libc::c_int;
    pub fn mosquitto_connect_bind(mosq: *mut Struct_mosquitto,
                                  host: *const ::libc::c_char,
                                  port: ::libc::c_int,
                                  keepalive: ::libc::c_int,
                                  bind_address: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn mosquitto_connect_async(mosq: *mut Struct_mosquitto,
                                   host: *const ::libc::c_char,
                                   port: ::libc::c_int,
                                   keepalive: ::libc::c_int) -> ::libc::c_int;
    pub fn mosquitto_connect_bind_async(mosq: *mut Struct_mosquitto,
                                        host: *const ::libc::c_char,
                                        port: ::libc::c_int,
                                        keepalive: ::libc::c_int,
                                        bind_address: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn mosquitto_connect_srv(mosq: *mut Struct_mosquitto,
                                 host: *const ::libc::c_char,
                                 keepalive: ::libc::c_int,
                                 bind_address: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn mosquitto_reconnect(mosq: *mut Struct_mosquitto) -> ::libc::c_int;
    pub fn mosquitto_reconnect_async(mosq: *mut Struct_mosquitto)
     -> ::libc::c_int;
    pub fn mosquitto_disconnect(mosq: *mut Struct_mosquitto) -> ::libc::c_int;
    pub fn mosquitto_publish(mosq: *mut Struct_mosquitto,
                             mid: *mut ::libc::c_int,
                             topic: *const ::libc::c_char,
                             payloadlen: ::libc::c_int,
                             payload: *const ::libc::c_void,
                             qos: ::libc::c_int, retain: u8) -> ::libc::c_int;
    pub fn mosquitto_subscribe(mosq: *mut Struct_mosquitto,
                               mid: *mut ::libc::c_int,
                               sub: *const ::libc::c_char, qos: ::libc::c_int)
     -> ::libc::c_int;
    pub fn mosquitto_unsubscribe(mosq: *mut Struct_mosquitto,
                                 mid: *mut ::libc::c_int,
                                 sub: *const ::libc::c_char) -> ::libc::c_int;
    pub fn mosquitto_message_copy(dst: *mut Struct_mosquitto_message,
                                  src: *const Struct_mosquitto_message)
     -> ::libc::c_int;
    pub fn mosquitto_message_free(message: *mut *mut Struct_mosquitto_message)
     -> ();
    pub fn mosquitto_loop(mosq: *mut Struct_mosquitto, timeout: ::libc::c_int,
                          max_packets: ::libc::c_int) -> ::libc::c_int;
    pub fn mosquitto_loop_forever(mosq: *mut Struct_mosquitto,
                                  timeout: ::libc::c_int,
                                  max_packets: ::libc::c_int)
     -> ::libc::c_int;
    pub fn mosquitto_loop_start(mosq: *mut Struct_mosquitto) -> ::libc::c_int;
    pub fn mosquitto_loop_stop(mosq: *mut Struct_mosquitto, force: u8)
     -> ::libc::c_int;
    pub fn mosquitto_socket(mosq: *mut Struct_mosquitto) -> ::libc::c_int;
    pub fn mosquitto_loop_read(mosq: *mut Struct_mosquitto,
                               max_packets: ::libc::c_int) -> ::libc::c_int;
    pub fn mosquitto_loop_write(mosq: *mut Struct_mosquitto,
                                max_packets: ::libc::c_int) -> ::libc::c_int;
    pub fn mosquitto_loop_misc(mosq: *mut Struct_mosquitto) -> ::libc::c_int;
    pub fn mosquitto_want_write(mosq: *mut Struct_mosquitto) -> u8;
    pub fn mosquitto_threaded_set(mosq: *mut Struct_mosquitto, threaded: u8)
     -> ::libc::c_int;
    pub fn mosquitto_opts_set(mosq: *mut Struct_mosquitto,
                              option: Enum_mosq_opt_t,
                              value: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn mosquitto_tls_set(mosq: *mut Struct_mosquitto,
                             cafile: *const ::libc::c_char,
                             capath: *const ::libc::c_char,
                             certfile: *const ::libc::c_char,
                             keyfile: *const ::libc::c_char,
                             pw_callback:
                                 ::std::option::Option<unsafe extern "C" fn(buf:
                                                                                *mut ::libc::c_char,
                                                                            size:
                                                                                ::libc::c_int,
                                                                            rwflag:
                                                                                ::libc::c_int,
                                                                            userdata:
                                                                                *mut ::libc::c_void)
                                                           -> ::libc::c_int>)
     -> ::libc::c_int;
    pub fn mosquitto_tls_insecure_set(mosq: *mut Struct_mosquitto, value: u8)
     -> ::libc::c_int;
    pub fn mosquitto_tls_opts_set(mosq: *mut Struct_mosquitto,
                                  cert_reqs: ::libc::c_int,
                                  tls_version: *const ::libc::c_char,
                                  ciphers: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn mosquitto_tls_psk_set(mosq: *mut Struct_mosquitto,
                                 psk: *const ::libc::c_char,
                                 identity: *const ::libc::c_char,
                                 ciphers: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn mosquitto_connect_callback_set(mosq: *mut Struct_mosquitto,
                                          on_connect:
                                              ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                             *mut Struct_mosquitto,
                                                                                         arg2:
                                                                                             *mut ::libc::c_void,
                                                                                         arg3:
                                                                                             ::libc::c_int)
                                                                        ->
                                                                            ()>)
     -> ();
    pub fn mosquitto_disconnect_callback_set(mosq: *mut Struct_mosquitto,
                                             on_disconnect:
                                                 ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                *mut Struct_mosquitto,
                                                                                            arg2:
                                                                                                *mut ::libc::c_void,
                                                                                            arg3:
                                                                                                ::libc::c_int)
                                                                           ->
                                                                               ()>)
     -> ();
    pub fn mosquitto_publish_callback_set(mosq: *mut Struct_mosquitto,
                                          on_publish:
                                              ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                             *mut Struct_mosquitto,
                                                                                         arg2:
                                                                                             *mut ::libc::c_void,
                                                                                         arg3:
                                                                                             ::libc::c_int)
                                                                        ->
                                                                            ()>)
     -> ();
    pub fn mosquitto_message_callback_set(mosq: *mut Struct_mosquitto,
                                          on_message:
                                              ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                             *mut Struct_mosquitto,
                                                                                         arg2:
                                                                                             *mut ::libc::c_void,
                                                                                         arg3:
                                                                                             *const Struct_mosquitto_message)
                                                                        ->
                                                                            ()>)
     -> ();
    pub fn mosquitto_subscribe_callback_set(mosq: *mut Struct_mosquitto,
                                            on_subscribe:
                                                ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                               *mut Struct_mosquitto,
                                                                                           arg2:
                                                                                               *mut ::libc::c_void,
                                                                                           arg3:
                                                                                               ::libc::c_int,
                                                                                           arg4:
                                                                                               ::libc::c_int,
                                                                                           arg5:
                                                                                               *const ::libc::c_int)
                                                                          ->
                                                                              ()>)
     -> ();
    pub fn mosquitto_unsubscribe_callback_set(mosq: *mut Struct_mosquitto,
                                              on_unsubscribe:
                                                  ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                 *mut Struct_mosquitto,
                                                                                             arg2:
                                                                                                 *mut ::libc::c_void,
                                                                                             arg3:
                                                                                                 ::libc::c_int)
                                                                            ->
                                                                                ()>)
     -> ();
    pub fn mosquitto_log_callback_set(mosq: *mut Struct_mosquitto,
                                      on_log:
                                          ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                         *mut Struct_mosquitto,
                                                                                     arg2:
                                                                                         *mut ::libc::c_void,
                                                                                     arg3:
                                                                                         ::libc::c_int,
                                                                                     arg4:
                                                                                         *const ::libc::c_char)
                                                                    -> ()>)
     -> ();
    pub fn mosquitto_reconnect_delay_set(mosq: *mut Struct_mosquitto,
                                         reconnect_delay: ::libc::c_uint,
                                         reconnect_delay_max: ::libc::c_uint,
                                         reconnect_exponential_backoff: u8)
     -> ::libc::c_int;
    pub fn mosquitto_max_inflight_messages_set(mosq: *mut Struct_mosquitto,
                                               max_inflight_messages:
                                                   ::libc::c_uint)
     -> ::libc::c_int;
    pub fn mosquitto_message_retry_set(mosq: *mut Struct_mosquitto,
                                       message_retry: ::libc::c_uint) -> ();
    pub fn mosquitto_user_data_set(mosq: *mut Struct_mosquitto,
                                   obj: *mut ::libc::c_void) -> ();
    pub fn mosquitto_socks5_set(mosq: *mut Struct_mosquitto,
                                host: *const ::libc::c_char,
                                port: ::libc::c_int,
                                username: *const ::libc::c_char,
                                password: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn mosquitto_strerror(mosq_errno: ::libc::c_int)
     -> *const ::libc::c_char;
    pub fn mosquitto_connack_string(connack_code: ::libc::c_int)
     -> *const ::libc::c_char;
    pub fn mosquitto_sub_topic_tokenise(subtopic: *const ::libc::c_char,
                                        topics: *mut *mut *mut ::libc::c_char,
                                        count: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn mosquitto_sub_topic_tokens_free(topics:
                                               *mut *mut *mut ::libc::c_char,
                                           count: ::libc::c_int)
     -> ::libc::c_int;
    pub fn mosquitto_topic_matches_sub(sub: *const ::libc::c_char,
                                       topic: *const ::libc::c_char,
                                       result: *mut u8) -> ::libc::c_int;
    pub fn mosquitto_pub_topic_check(topic: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn mosquitto_sub_topic_check(topic: *const ::libc::c_char)
     -> ::libc::c_int;
}
