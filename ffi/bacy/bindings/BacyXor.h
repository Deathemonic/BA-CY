#ifndef BacyXor_H
#define BacyXor_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BacyBytes.d.h"
#include "BacyError.d.h"

#include "BacyXor.d.h"






typedef struct BacyXor_encrypt_result {union { BacyError* err;}; bool is_ok;} BacyXor_encrypt_result;
BacyXor_encrypt_result BacyXor_encrypt(DiplomatU8ViewMut data, size_t offset, size_t length);

BacyBytes* BacyXor_encrypt_with_key(DiplomatU8View data, DiplomatU8View key);

BacyBytes* BacyXor_xor_exact(DiplomatU8View value, DiplomatU8View key);

void BacyXor_xor_inplace(DiplomatU8ViewMut data, DiplomatU8View key);

void BacyXor_destroy(BacyXor* self);





#endif // BacyXor_H
