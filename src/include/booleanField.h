#pragma once

#include "baseField.h"

class booleanField : baseField<bool> {
public:
  booleanField logicalAnd(booleanField other);

  booleanField logicalOr(booleanField other);

  booleanField logicalXor(booleanField other);

  booleanField logicalNot();
};