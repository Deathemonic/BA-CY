#ifndef BacyBytes_H
#define BacyBytes_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BacyBytes.d.h"






DiplomatU8View BacyBytes_as_slice(const BacyBytes* self);

void BacyBytes_destroy(BacyBytes* self);





#endif // BacyBytes_H
