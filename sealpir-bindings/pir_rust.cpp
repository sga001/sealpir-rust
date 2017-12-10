#include "pir_rust.hpp"

void *new_parameters(uint32_t ele_num, uint32_t ele_size, uint32_t N, uint32_t logt, uint32_t d) {
    Parameters *param = new Parameters;
    gen_params(ele_num, ele_size, N, logt, d, param->params, param->expanded_params,
               param->pir_params);
    return (void *)param;
}

void update_parameters(void *params, uint32_t ele_num, uint32_t ele_size, uint32_t d) {
    Parameters *param = (Parameters *) params;
    update_params(ele_num, ele_size, d, param->params, param->expanded_params, param->pir_params);
}

void delete_parameters(void *params) { delete ((Parameters *)params); }

void *new_pir_client(const void *params) {
    Parameters *param = (Parameters *)params;
    PIRClient *client = new PIRClient(param->params, param->expanded_params, param->pir_params);
    return (void *)client;
}

void update_client_params(void *pir_client, const void *params) {
    PIRClient *client = (PIRClient *) pir_client;
    Parameters *param = (Parameters *)params;
    client->update_parameters(param->expanded_params, param->pir_params);
}

void delete_pir_client(void *pir_client) { delete ((PIRClient *)pir_client); }

void *new_pir_server(const void *params) {
    Parameters *param = (Parameters *)params;
    PIRServer *server = new PIRServer(param->expanded_params, param->pir_params);
    return (void *)server;
}

void update_server_params(void *pir_server, const void *params) {
    PIRServer *server = (PIRServer *) pir_server;
    Parameters *param = (Parameters *)params;
    server->update_parameters(param->expanded_params, param->pir_params);
}

void delete_pir_server(void *pir_server) { delete ((PIRServer *)pir_server); }

uint8_t *get_galois_key(const void *pir_client, uint32_t *key_size) {
    PIRClient *client = (PIRClient *)pir_client;
    seal::GaloisKeys galois = client->generate_galois_keys();
    string ser = serialize_galoiskeys(galois);

    uint32_t size = ser.size();
    uint8_t *out = (uint8_t *)malloc(size);
    memcpy(out, ser.data(), size);
    *key_size = size;
    return out;
}

void set_galois_key(void *pir_server, const uint8_t *galois_key, uint32_t key_size,
                    uint32_t client_id) {
    PIRServer *server = (PIRServer *)pir_server;
    string gal_str = string((const char *)galois_key, key_size);
    seal::GaloisKeys *galois = deserialize_galoiskeys(gal_str);
    server->set_galois_key(client_id, *galois);
    delete galois;
}

uint32_t get_fv_index(const void *pir_client, uint32_t ele_index, uint32_t ele_size) {
    PIRClient *client = (PIRClient *)pir_client;
    return client->get_fv_index(ele_index, ele_size);
}

uint32_t get_fv_offset(const void *pir_client, uint32_t ele_index, uint32_t ele_size) {
    PIRClient *client = (PIRClient *)pir_client;
    return client->get_fv_offset(ele_index, ele_size);
}

uint8_t *generate_query(const void *pir_client, uint32_t index, uint32_t *query_size,
                        uint32_t *query_num) {
    PIRClient *client = (PIRClient *)pir_client;
    PirQuery query = client->generate_query(index);
    *query_num = query.size();
    string ser = serialize_ciphertexts(query);

    uint32_t size = ser.size();
    uint8_t *out = (uint8_t *)malloc(size);
    memcpy(out, ser.data(), size);
    *query_size = size;
    return out;
}

void expand_query(const void *pir_server, const void *params, const uint8_t *query,
                  uint32_t query_size, uint32_t query_num, uint32_t client_id) {

    PIRServer *server = (PIRServer *)pir_server;
    Parameters *param = (Parameters *)params;
    string query_str = string((const char *)query, query_size);

    PirQuery query_des = deserialize_ciphertexts(query_num, query_str, CIPHER_SIZE);

    for (uint32_t i = 0; i < query_num; i++) {
        uint32_t m = param->pir_params.nvec[i];
        PirQuery expanded = server->expand_query(query_des[i], m, client_id);
    }
}

uint8_t *generate_reply(const void *pir_server, const uint8_t *query, uint32_t query_size,
                        uint32_t query_num, uint32_t *reply_size, uint32_t *reply_num,
                        uint32_t client_id) {

    PIRServer *server = (PIRServer *)pir_server;

    string query_str = string((const char *)query, query_size);

    PirQuery query_des = deserialize_ciphertexts(query_num, query_str, CIPHER_SIZE);
    PirReply reply = server->generate_reply(query_des, client_id);
    *reply_num = reply.size();

    string ser = serialize_ciphertexts(reply);
    uint32_t size = ser.size();
    uint8_t *out = (uint8_t *)malloc(size);
    memcpy(out, ser.data(), size);
    *reply_size = size;
    return out;
}

void set_database(void *pir_server, const uint8_t *database, uint32_t ele_num, uint32_t ele_size) {
    PIRServer *server = (PIRServer *)pir_server;
    server->set_database(database, ele_num, ele_size);
}

void preprocess_db(void *pir_server) {
    PIRServer *server = (PIRServer *)pir_server;
    server->preprocess_database();
}

uint8_t *decode_reply(const void *pir_client, const void *params, const uint8_t *reply,
                      uint32_t reply_size, uint32_t reply_num, uint32_t *size) {

    PIRClient *client = (PIRClient *)pir_client;
    Parameters *param = (Parameters *)params;

    string reply_str = string((const char *)reply, reply_size);

    PirReply reply_res = deserialize_ciphertexts(reply_num, reply_str, CIPHER_SIZE);
    seal::Plaintext result = client->decode_reply(reply_res);

    uint32_t logtp = ceil(log2(param->expanded_params.plain_modulus().value()));
    uint32_t N = param->expanded_params.poly_modulus().coeff_count() - 1;

    uint8_t *elems = (uint8_t *)malloc((N * logtp) / 8);
    coeffs_to_bytes(logtp, result, elems, (N * logtp) / 8);

    *size = (N * logtp) / 8;
    return elems;
}
