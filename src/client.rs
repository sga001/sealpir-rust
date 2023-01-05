use libc;
use std::mem;
use std::slice;

use super::{PirQuery, PirReply};

extern "C" {
    fn new_parameters(ele_num: u32, ele_size: u32, N: u32, logt: u32, d: u32) -> *mut libc::c_void;
    fn update_parameters(params: *mut libc::c_void, ele_num: u32, ele_size: u32, d: u32);
    fn delete_parameters(params: *mut libc::c_void);

    fn new_pir_client(params: *const libc::c_void) -> *mut libc::c_void;
    fn update_client_params(pir_client: *mut libc::c_void, params: *const libc::c_void);
    fn delete_pir_client(pir_client: *mut libc::c_void);

    fn get_fv_index(pir_client: *const libc::c_void, ele_idx: u32, ele_size: u32) -> u32;
    fn get_fv_offset(pir_client: *const libc::c_void, ele_idx: u32, ele_size: u32) -> u32;

    fn get_galois_key(pir_client: *const libc::c_void, key_size: &mut u32) -> *mut u8;

    fn generate_query(
        pir_client: *const libc::c_void,
        index: u32,
        query_size: &mut u32,
        query_num: &mut u32,
    ) -> *mut u8;

    fn decode_reply(
        pir_client: *const libc::c_void,
        params: *const libc::c_void,
        reply: *const u8,
        reply_size: u32,
        reply_num: u32,
        result_size: &mut u32,
    ) -> *mut u8;
}

pub struct PirClient<'a> {
    client: &'a mut libc::c_void,
    params: &'a mut libc::c_void,
    ele_size: u32,
    ele_num: u32,
    key: Vec<u8>,
}

impl<'a> Drop for PirClient<'a> {
    fn drop(&mut self) {
        unsafe {
            delete_pir_client(self.client);
            delete_parameters(self.params);
        }
    }
}

impl<'a> PirClient<'a> {
    pub fn new(
        ele_num: u32,
        ele_size: u32,
        poly_degree: u32,
        log_plain_mod: u32,
        d: u32,
    ) -> PirClient<'a> {
        let param_ptr: &'a mut libc::c_void =
            unsafe { &mut *(new_parameters(ele_num, ele_size, poly_degree, log_plain_mod, d)) };

        let client_ptr: &'a mut libc::c_void = unsafe { &mut *(new_pir_client(param_ptr)) };

        let mut key_size: u32 = 0;

        let key: Vec<u8> = unsafe {
            let ptr = get_galois_key(client_ptr, &mut key_size);
            let key = slice::from_raw_parts_mut(ptr as *mut u8, key_size as usize).to_vec();
            libc::free(ptr as *mut libc::c_void);
            key
        };

        PirClient {
            client: client_ptr,
            params: param_ptr,
            ele_size,
            ele_num,
            key,
        }
    }

    pub fn update_params(&mut self, ele_num: u32, ele_size: u32, d: u32) {
        unsafe {
            update_parameters(self.params, ele_num, ele_size, d);
            update_client_params(self.client, self.params);
        }

        self.ele_size = ele_size;
        self.ele_num = ele_num;
    }

    pub fn get_key(&'a self) -> &'a Vec<u8> {
        &self.key
    }

    pub fn gen_query(&self, index: u32) -> PirQuery {
        assert!(index <= self.ele_num);

        let mut query_size: u32 = 0; // # of bytes
        let mut query_num: u32 = 0; // # of ciphertexts

        let query: Vec<u8> = unsafe {
            let fv_index = get_fv_index(self.client, index, self.ele_size);
            let ptr = generate_query(self.client, fv_index, &mut query_size, &mut query_num);
            let q = slice::from_raw_parts_mut(ptr as *mut u8, query_size as usize).to_vec();
            libc::free(ptr as *mut libc::c_void);
            q
        };

        PirQuery {
            query,
            num: query_num,
        }
    }

    pub fn decode_reply<T>(&self, ele_index: u32, reply: &PirReply) -> T
    where
        T: Clone,
    {
        assert_eq!(self.ele_size as usize, mem::size_of::<T>());

        let mut result_size: u32 = 0;
        let result: T = unsafe {
            // returns the content of the FV plaintext
            let ptr = decode_reply(
                self.client,
                self.params,
                reply.reply.as_ptr(),
                reply.reply.len() as u32,
                reply.num,
                &mut result_size,
            );

            // offset into the FV plaintext
            let offset = get_fv_offset(self.client, ele_index, self.ele_size);
            assert!(offset + self.ele_size <= result_size);

            let r = slice::from_raw_parts_mut((ptr as *mut T).offset(offset as isize), 1).to_vec();
            libc::free(ptr as *mut libc::c_void);
            r[0].clone()
        };

        result
    }
}
