#ifndef BacyAes_H
#define BacyAes_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BacyBytes.d.h"
#include "BacyError.d.h"

#include "BacyAes.d.h"






typedef struct BacyAes_decrypt_result {union {BacyBytes* ok; BacyError* err;}; bool is_ok;} BacyAes_decrypt_result;
BacyAes_decrypt_result BacyAes_decrypt(DiplomatU8View data, DiplomatU8View key, DiplomatU8View iv);

void BacyAes_destroy(BacyAes* self);





#endif // BacyAes_H
