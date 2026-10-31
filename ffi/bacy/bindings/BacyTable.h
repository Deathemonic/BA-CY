#ifndef BacyTable_H
#define BacyTable_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BacyBytes.d.h"
#include "BacyError.d.h"

#include "BacyTable.d.h"






BacyBytes* BacyTable_create_key(DiplomatStringView name);

typedef struct BacyTable_create_password_result {union { BacyError* err;}; bool is_ok;} BacyTable_create_password_result;
BacyTable_create_password_result BacyTable_create_password(DiplomatStringView key, size_t length, DiplomatWrite* write);

void BacyTable_xor(DiplomatStringView name, DiplomatU8ViewMut data);

int32_t BacyTable_decrypt_i32(int32_t value, DiplomatU8View key);

int64_t BacyTable_decrypt_i64(int64_t value, DiplomatU8View key);

uint32_t BacyTable_decrypt_u32(uint32_t value, DiplomatU8View key);

uint64_t BacyTable_decrypt_u64(uint64_t value, DiplomatU8View key);

typedef struct BacyTable_decrypt_f32_result {union {float ok; BacyError* err;}; bool is_ok;} BacyTable_decrypt_f32_result;
BacyTable_decrypt_f32_result BacyTable_decrypt_f32(float value, DiplomatU8View key);

typedef struct BacyTable_decrypt_f64_result {union {double ok; BacyError* err;}; bool is_ok;} BacyTable_decrypt_f64_result;
BacyTable_decrypt_f64_result BacyTable_decrypt_f64(double value, DiplomatU8View key);

typedef struct BacyTable_encrypt_f32_result {union {float ok; BacyError* err;}; bool is_ok;} BacyTable_encrypt_f32_result;
BacyTable_encrypt_f32_result BacyTable_encrypt_f32(float value, DiplomatU8View key);

typedef struct BacyTable_encrypt_f64_result {union {double ok; BacyError* err;}; bool is_ok;} BacyTable_encrypt_f64_result;
BacyTable_encrypt_f64_result BacyTable_encrypt_f64(double value, DiplomatU8View key);

typedef struct BacyTable_decrypt_string_result {union { BacyError* err;}; bool is_ok;} BacyTable_decrypt_string_result;
BacyTable_decrypt_string_result BacyTable_decrypt_string(DiplomatStringView value, DiplomatU8View key, DiplomatWrite* write);

typedef struct BacyTable_encrypt_string_result {union { BacyError* err;}; bool is_ok;} BacyTable_encrypt_string_result;
BacyTable_encrypt_string_result BacyTable_encrypt_string(DiplomatStringView value, DiplomatU8View key, DiplomatWrite* write);

void BacyTable_destroy(BacyTable* self);





#endif // BacyTable_H
