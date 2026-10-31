#ifndef BacyError_H
#define BacyError_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BacyErrorKind.d.h"

#include "BacyError.d.h"






typedef struct BacyError_message_result {union { BacyError* err;}; bool is_ok;} BacyError_message_result;
BacyError_message_result BacyError_message(const BacyError* self, DiplomatWrite* write);

BacyErrorKind BacyError_kind(const BacyError* self);

void BacyError_destroy(BacyError* self);





#endif // BacyError_H
