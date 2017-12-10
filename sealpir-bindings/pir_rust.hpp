#ifndef SEAL_PIR_RUST_H
#define SEAL_PIR_RUST_H

#include "pir.hpp"
#include "pir_client.hpp"
#include "pir_server.hpp"

extern "C" {

struct Parameters {
    seal::EncryptionParameters params;
    seal::EncryptionParameters expanded_params;
    PirParams pir_params;
};

// returns a pointer to SealPIR's parameters
void *new_parameters(uint32_t ele_num, uint32_t ele_size, uint32_t N, uint32_t logt, uint32_t d);
void update_parameters(void *params, uint32_t ele_num, uint32_t ele_size, uint32_t d);
void delete_parameters(void *params);

// Client operations

// returns a pointer to a PirClient object
void *new_pir_client(const void *params);
void update_client_params(void *pir_client, const void *params);
void delete_pir_client(void *pir_client);

// returns index of FV plaintext given the index of an element
uint32_t get_fv_index(const void *pir_client, uint32_t ele_index, uint32_t ele_size);

// returns the offset within an FV plaintext for the given element index
uint32_t get_fv_offset(const void *pir_client, uint32_t ele_index, uint32_t ele_size);

// get the serialized representation of a galois key
uint8_t *get_galois_key(const void *pir_client, uint32_t *key_size);

// get the serialized version of a PIR query for the given index
// num: number of ciphertexts making up the query
// query_size: size in bytes
uint8_t *generate_query(const void *pir_client, uint32_t index, uint32_t *query_size,
                        uint32_t *query_num);

// decodes the given reply and returns a pointer to the N coefficients
// reply_num: number of ciphertexts making up the query
// reply_size: size in bytes of the reply
// size: size in bytes of the decoded elements
uint8_t *decode_reply(const void *pir_client, const void *param, const uint8_t *reply,
                      uint32_t reply_size, uint32_t reply_num, uint32_t *size);

// Server operations

// returns a pointer to a PirServer object
void *new_pir_server(const void *params);
void update_server_params(void *pir_server, const void *params);
void delete_pir_server(void *pir_server);

// deserializes the galois key and configures it for the given client
void set_galois_key(void *pir_server, const uint8_t *galois_key, uint32_t key_size,
                    uint32_t client_id);

// sets the existing database
void set_database(void *pir_server, const uint8_t *database, uint32_t ele_num, uint32_t ele_size);

// preprocesses the database
void preprocess_db(void *pir_server);

// For microbenchmark purposes only (generate_reply does this already)
void expand_query(const void *pir_server, const void *params, const uint8_t *query,
                  uint32_t query_size, uint32_t query_num, uint32_t client_id);

// generates a reply for the given client
// query_size: bytes of query
// query_num: number of ciphertexts
// reply_num: number of ciphertexts
// reply_size: bytes of reply
uint8_t *generate_reply(const void *pir_server, const uint8_t *query, uint32_t query_size,
                        uint32_t query_num, uint32_t *reply_size, uint32_t *reply_num,
                        uint32_t client_id);
}
#endif
