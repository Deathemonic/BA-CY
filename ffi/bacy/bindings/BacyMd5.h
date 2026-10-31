#ifndef BacyMd5_H
#define BacyMd5_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BacyBytes.d.h"
#include "BacyError.d.h"

#include "BacyMd5.d.h"






typedef struct BacyMd5_to_hex_string_result {union { BacyError* err;}; bool is_ok;} BacyMd5_to_hex_string_result;
BacyMd5_to_hex_string_result BacyMd5_to_hex_string(DiplomatU8View data, DiplomatWrite* write);

BacyBytes* BacyMd5_compute_hash(DiplomatU8View source);

BacyBytes* BacyMd5_compute_hash_hmac(DiplomatU8View source, DiplomatU8View key);

typedef struct BacyMd5_compute_hash_str_result {union { BacyError* err;}; bool is_ok;} BacyMd5_compute_hash_str_result;
BacyMd5_compute_hash_str_result BacyMd5_compute_hash_str(DiplomatStringView source, DiplomatWrite* write);

typedef struct BacyMd5_compute_hash_str_hmac_result {union { BacyError* err;}; bool is_ok;} BacyMd5_compute_hash_str_hmac_result;
BacyMd5_compute_hash_str_hmac_result BacyMd5_compute_hash_str_hmac(DiplomatStringView source, DiplomatStringView key, DiplomatWrite* write);

typedef struct BacyMd5_compute_streaming_result {union { BacyError* err;}; bool is_ok;} BacyMd5_compute_streaming_result;
BacyMd5_compute_streaming_result BacyMd5_compute_streaming(DiplomatStringView path, size_t buffer_size, DiplomatWrite* write);

uint32_t BacyMd5_compute_digest(DiplomatStringView source);

uint32_t BacyMd5_compute_digest_hmac(DiplomatStringView source, DiplomatStringView key);

uint64_t BacyMd5_compute_digest64(DiplomatStringView source);

uint64_t BacyMd5_compute_digest64_hmac(DiplomatStringView source, DiplomatStringView key);

typedef struct BacyMd5_compute_head_result {union { BacyError* err;}; bool is_ok;} BacyMd5_compute_head_result;
BacyMd5_compute_head_result BacyMd5_compute_head(DiplomatStringView source, DiplomatWrite* write);

void BacyMd5_destroy(BacyMd5* self);





#endif // BacyMd5_H
