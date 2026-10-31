#ifndef BacyCrcResult_H
#define BacyCrcResult_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BacyError.d.h"

#include "BacyCrcResult.d.h"






typedef struct BacyCrcResult_hex_result {union { BacyError* err;}; bool is_ok;} BacyCrcResult_hex_result;
BacyCrcResult_hex_result BacyCrcResult_hex(const BacyCrcResult* self, DiplomatWrite* write);





#endif // BacyCrcResult_H
