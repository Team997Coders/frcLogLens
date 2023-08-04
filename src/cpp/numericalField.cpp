#include "numericalField.h"

#include <algorithm>

numericalField numericalField::scalarMultiply(double scalar) {
  std::vector<double, std::allocator<double>> values = this->getValueSet();

  std::transform(values.cbegin(), values.cend(), values.begin(),
                 [scalar](double value) { return scalar * value; });

  return numericalField{values, this->getTimestampSet()};
}