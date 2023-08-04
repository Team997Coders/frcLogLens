#pragma once

#include "baseField.h"

template <class T> bool numericalCastSafe(baseField<T> field) {
  for (T value : field.getValues()) {
    static_cast<value>
  }
}

template <class T> void castToNumerical(baseField<T> field) {}

class numericalField : baseField<double> {
public:
  numericalField scalarMultiply(double scalar);

  numericalField scalarDivide(double scalar);

  numericalField scalarAdd(double scalar);

  numericalField multiply(numericalField other);

  numericalField divide(numericalField other);

  numericalField add(numericalField other);

  numericalField subtract(numericalField other);

  numericalField finiteDifference();

  numericalField finiteIntegral();

  numericalField sum();

  numericalField deltaFromPrevious();
};